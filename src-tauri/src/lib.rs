pub mod audio;
pub mod commands;
pub mod db;
pub mod notifications;
pub mod settings;
pub mod shortcuts;
pub mod themes;
pub mod timer;
pub mod tray;
pub mod websocket;

use std::sync::Arc;

use log::LevelFilter;
use tauri::Manager;
use tauri_plugin_log::{Builder as LogBuilder, RotationStrategy, Target, TargetKind};

use commands::{
    accessibility_trusted,
    tray_supported,
    app_version,
    check_update,
    install_update,
    audio_clear_custom, audio_get_custom_info, audio_set_custom,
    get_log_dir, open_log_dir,
    notification_show,
    settings_get, settings_reset_defaults, settings_set,
    shortcuts_reload,
    sessions_clear, sessions_rename_label,
    stats_get_detailed, stats_get_heatmap, stats_get_label_breakdown, stats_get_weekly_labels,
    themes_list,
    timer_get_state, timer_reset, timer_restart_round, timer_skip, timer_toggle, timer_set_label,
    window_set_visibility,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            LogBuilder::new()
                .targets([Target::new(TargetKind::LogDir { file_name: None })])
                .max_file_size(5 * 1024 * 1024)
                .rotation_strategy(RotationStrategy::KeepOne)
                .level(LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Capture Rust panics to the log file before the process terminates.
            std::panic::set_hook(Box::new(|info| {
                log::error!("PANIC: {info}");
            }));

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("failed to create app data directory");

            // --- Database ---
            let db = match db::open(&app_data_dir) {
                Ok(d) => {
                    log::info!(
                        "[app] version={} sha={}",
                        env!("APP_BUILD_VERSION"),
                        env!("APP_BUILD_SHA")
                    );
                    log::info!(
                        "Pomotroid v{} — data dir: {}",
                        env!("CARGO_PKG_VERSION"),
                        app_data_dir.display()
                    );
                    log::info!("Database opened successfully");
                    d
                }
                Err(e) => {
                    log::error!("Failed to open database: {e}");
                    panic!("failed to open database: {e}");
                }
            };
            {
                let conn = db.lock().unwrap();
                settings::seed_defaults(&conn).expect("failed to seed default settings");
            }
            app.manage(db.clone());

            // --- Tray state (always created; icon populated only when min_to_tray is on) ---
            let tray_state = tray::TrayState::new();
            app.manage(Arc::clone(&tray_state));

            // --- Load settings once (used by Timer, Audio, etc.) ---
            let initial_settings = {
                let conn = db.lock().unwrap();
                settings::load(&conn).expect("failed to load settings")
            };

            // Apply the persisted log level before any further setup.
            if initial_settings.verbose_logging {
                log::set_max_level(LevelFilter::Debug);
                log::info!("Verbose logging enabled — log level set to DEBUG");
            } else {
                log::set_max_level(LevelFilter::Info);
            }

            // Sync tray state from saved settings.
            *tray_state.countdown_mode.lock().unwrap() = initial_settings.dial_countdown;
            let tray_theme_name = match initial_settings.theme_mode.as_str() {
                "dark" => &initial_settings.theme_dark,
                _ => &initial_settings.theme_light,
            };
            if let Some(theme) = themes::find(&app_data_dir, tray_theme_name) {
                *tray_state.colors.lock().unwrap() = tray::TrayColors::from_colors_map(&theme.colors);
            }

            // --- Audio engine (optional — graceful if no audio device) ---
            if let Some(audio) = audio::AudioManager::new(&initial_settings) {
                // Restore any previously saved custom audio files.
                let audio_dir = app_data_dir.join("audio");
                if audio_dir.exists() {
                    let custom = audio::find_custom_files(&audio_dir);
                    *audio.custom_paths.lock().unwrap() = custom;
                }
                app.manage(audio);
            }

            // --- Timer controller (needs settings + AppHandle + TrayState + DB) ---
            let timer = timer::TimerController::new(
                app.handle().clone(),
                initial_settings.clone(),
                Arc::clone(&tray_state),
                db.clone(),
            );
            app.manage(timer);

            // Create initial tray icon if tray_icon_enabled is on, or if an
            // existing user has min_to_tray enabled (backwards compatibility).
            //
            // On Linux, TrayIconBuilder::build() can block the main thread
            // indefinitely on KDE Plasma 6 / Wayland while waiting for the
            // StatusNotifierWatcher D-Bus service to respond.  Spawning on a
            // background thread lets setup() return so the event loop starts
            // and the window can appear while the tray registers asynchronously.
            if initial_settings.tray_icon_enabled || initial_settings.min_to_tray {
                #[cfg(target_os = "linux")]
                {
                    let app_handle = app.handle().clone();
                    let ts = Arc::clone(&tray_state);
                    std::thread::spawn(move || {
                        tray::create_tray(&app_handle, &ts);
                    });
                }
                #[cfg(not(target_os = "linux"))]
                tray::create_tray(app.handle(), &tray_state);
            }

            // --- Theme hot-reload watcher ---
            // The watcher must stay alive for the duration of the app.
            // Wrap in a Mutex so it satisfies Send + Sync for Tauri manage.
            if let Some(watcher) = themes::watcher::spawn_watcher(
                app_data_dir.clone(),
                app.handle().clone(),
            ) {
                app.manage(std::sync::Mutex::new(watcher));
            }

            // --- Global shortcuts ---
            shortcuts::register_all(app.handle(), &initial_settings);

            // --- WebSocket server (opt-in) ---
            let ws_state = websocket::WsState::new();
            app.manage(Arc::clone(&ws_state));

            if initial_settings.websocket_enabled {
                let port = initial_settings.websocket_port;
                let app_clone = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    websocket::start(port, app_clone, &ws_state).await;
                });
            }

            // --- Window event handlers ---
            let main_window = app
                .get_webview_window("main")
                .expect("main window not found");

            // The config sets decorations:true so macOS renders the window correctly
            // (decorations:false prevents event processing on macOS). On every other
            // platform, restore the decorations-free window immediately; the window is
            // still hidden at this point so there is no visible flash.
            #[cfg(not(target_os = "macos"))]
            let _ = main_window.set_decorations(false);

            // Enable macOS window tiling/arrangement.
            //
            // Two things are required for the full native experience:
            //
            // 1. NSWindowCollectionBehaviorManaged on the NSWindow — enables the
            //    tiling popup on the green traffic-light button AND tells macOS
            //    this window participates in Spaces / tiling.
            //
            // 2. NSApplication.setWindowsMenu: — registers the "Window" menu from
            //    the menu bar as the official windows menu. macOS then dynamically
            //    injects the standard window-management items (Fill, Center,
            //    Move & Resize with Halves/Quarters/Arrange, Full Screen Tile,
            //    Bring All to Front, etc.) when the menu is opened. Tauri may or
            //    may not wire this up, so we do it explicitly here.
            #[cfg(target_os = "macos")]
            {
                use objc2::msg_send;
                use objc2::runtime::{AnyClass, AnyObject};
                use raw_window_handle::{HasWindowHandle, RawWindowHandle};

                if let Ok(handle) = main_window.window_handle() {
                    if let RawWindowHandle::AppKit(h) = handle.as_raw() {
                        let ns_view = h.ns_view.as_ptr() as *mut AnyObject;
                        // SAFETY: ns_view is a valid NSView* supplied by Tauri/WRY.
                        let ns_window: *mut AnyObject =
                            unsafe { msg_send![ns_view, window] };

                        // NSWindowCollectionBehaviorManaged        = 1 << 2  (tiling)
                        // NSWindowCollectionBehaviorFullScreenPrimary = 1 << 10 (full-screen)
                        let behavior: u64 = (1 << 2) | (1 << 10);
                        unsafe {
                            let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
                        }

                        // Walk the main menu bar looking for the "Window" menu and
                        // register it with NSApplication. This causes macOS to
                        // auto-populate it with all standard window-management items.
                        unsafe {
                            let ns_app_class = AnyClass::get(
                                std::ffi::CStr::from_bytes_with_nul_unchecked(b"NSApplication\0"),
                            );
                            if let Some(cls) = ns_app_class {
                                let ns_app: *mut AnyObject =
                                    msg_send![cls, sharedApplication];
                                let main_menu: *mut AnyObject =
                                    msg_send![ns_app, mainMenu];
                                if !main_menu.is_null() {
                                    let count: isize =
                                        msg_send![main_menu, numberOfItems];
                                    for i in 0..count {
                                        let item: *mut AnyObject =
                                            msg_send![main_menu, itemAtIndex: i];
                                        let submenu: *mut AnyObject =
                                            msg_send![item, submenu];
                                        if submenu.is_null() {
                                            continue;
                                        }
                                        let title: *mut AnyObject =
                                            msg_send![item, title];
                                        let utf8: *const std::ffi::c_char =
                                            msg_send![title, UTF8String];
                                        if utf8.is_null() {
                                            continue;
                                        }
                                        let s = std::ffi::CStr::from_ptr(utf8)
                                            .to_str()
                                            .unwrap_or("");
                                        if s == "Window" {
                                            let _: () = msg_send![
                                                ns_app,
                                                setWindowsMenu: submenu
                                            ];
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Apply always-on-top from saved settings on startup.
            if initial_settings.always_on_top {
                let _ = main_window.set_always_on_top(true);
            }

            // CloseRequested: hide to tray instead of quitting if min_to_tray_on_close.
            // When actually closing (not hiding), also close any child windows so they
            // cannot be left orphaned with no way to reopen the main window.
            let db_for_close = db.clone();
            let win_for_close = main_window.clone();
            let app_for_close = app.handle().clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let hide = db_for_close
                        .lock()
                        .ok()
                        .and_then(|conn| settings::load(&conn).ok())
                        .map(|s| s.min_to_tray_on_close)
                        .unwrap_or(false);
                    if hide {
                        api.prevent_close();
                        let _ = win_for_close.hide();
                    } else {
                        // Main window is truly closing — close child windows if open.
                        for label in ["settings", "stats"] {
                            if let Some(win) = app_for_close.get_webview_window(label) {
                                let _ = win.close();
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Timer
            timer_toggle,
            timer_reset,
            timer_restart_round,
            timer_skip,
            timer_get_state,
            timer_set_label,
            // Settings
            settings_get,
            settings_set,
            settings_reset_defaults,
            // Themes
            themes_list,
            // Sessions
            sessions_clear,
            sessions_rename_label,
            // Stats
            stats_get_detailed,
            stats_get_heatmap,
            stats_get_label_breakdown,
            stats_get_weekly_labels,
            // Window
            window_set_visibility,
            // Shortcuts
            shortcuts_reload,
            // Audio
            audio_set_custom,
            audio_clear_custom,
            audio_get_custom_info,
            // Notifications
            notification_show,
            // Diagnostics
            open_log_dir,
            get_log_dir,
            accessibility_trusted,
            tray_supported,
            app_version,
            // Updater
            check_update,
            install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
