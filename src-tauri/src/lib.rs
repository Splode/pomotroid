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

use tauri::Manager;

use commands::{
    audio_clear_custom, audio_get_custom_info, audio_set_custom,
    notification_show,
    settings_get, settings_reset_defaults, settings_set,
    shortcuts_reload,
    stats_get_all_time, stats_get_session,
    themes_list,
    timer_get_state, timer_reset, timer_restart_round, timer_skip, timer_toggle,
    window_set_visibility,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("failed to create app data directory");

            // --- Database ---
            let db = db::open(&app_data_dir).expect("failed to open database");
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

            // Create initial tray icon if min_to_tray is already on.
            if initial_settings.min_to_tray {
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

            // Apply always-on-top from saved settings on startup.
            if initial_settings.always_on_top {
                let _ = main_window.set_always_on_top(true);
            }

            // CloseRequested: hide to tray instead of quitting if min_to_tray_on_close.
            let db_for_close = db.clone();
            let win_for_close = main_window.clone();
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
            // Settings
            settings_get,
            settings_set,
            settings_reset_defaults,
            // Themes
            themes_list,
            // Stats
            stats_get_all_time,
            stats_get_session,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
