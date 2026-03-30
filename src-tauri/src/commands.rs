/// All #[tauri::command] functions exposed to the Svelte frontend via Tauri IPC.
///
/// Commands are grouped by domain: Timer, Settings, Themes, Stats.
/// Each command returns `Result<T, String>` so errors surface cleanly in JS.
use log::LevelFilter;
use tauri::{AppHandle, Emitter, Manager, State};

use std::sync::Arc;

use crate::achievements::{eval as achievements_eval, AchievementView};
use crate::audio::{self, AudioManager};
use crate::notifications;
use crate::db::{queries, DbState};
use crate::settings::{self, Settings};
use crate::shortcuts;
use crate::themes::{self, Theme};
use crate::timer::{TimerController, TimerSnapshot};
use crate::tray::{self, TrayState};
use crate::websocket::{self, WsState};

// ---------------------------------------------------------------------------
// CMD-01 — Timer commands
// ---------------------------------------------------------------------------

/// Toggle the timer: start if idle, resume if paused, pause if running.
/// This is the primary action bound to the space bar and the play/pause button.
#[tauri::command]
pub fn timer_toggle(timer: State<'_, TimerController>) {
    timer.toggle();
}

/// Reset the current round's timer without advancing the sequence.
#[tauri::command]
pub fn timer_reset(timer: State<'_, TimerController>) {
    timer.reset();
}

/// Skip the current round: fires Complete immediately and advances to the next.
#[tauri::command]
pub fn timer_skip(timer: State<'_, TimerController>) {
    timer.skip();
}

/// Restart the current round from zero without advancing the sequence.
/// Round type and round number are preserved; only elapsed time is reset.
#[tauri::command]
pub fn timer_restart_round(timer: State<'_, TimerController>) {
    timer.restart_round();
}

/// Return a full snapshot of the current timer state.
/// Called once on frontend mount to hydrate stores.
#[tauri::command]
pub fn timer_get_state(timer: State<'_, TimerController>) -> TimerSnapshot {
    timer.get_snapshot()
}

// ---------------------------------------------------------------------------
// CMD-02 — Settings commands
// ---------------------------------------------------------------------------

/// Return all current settings.
#[tauri::command]
pub fn settings_get(db: State<'_, DbState>) -> Result<Settings, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    settings::load(&conn).map_err(|e| {
        log::error!("[settings] failed to load settings: {e}");
        e.to_string()
    })
}

/// Persist a single setting and emit `settings:changed` with the updated set.
///
/// `key` must be one of the DB column names (see `settings::defaults::DEFAULTS`).
/// `value` is always a string; the loader converts it to the appropriate type.
#[tauri::command]
pub fn settings_set(
    key: String,
    value: String,
    db: State<'_, DbState>,
    timer: State<'_, TimerController>,
    tray_state: State<'_, Arc<TrayState>>,
    ws_state: State<'_, Arc<WsState>>,
    app: AppHandle,
) -> Result<Settings, String> {
    log::debug!("[settings] set {key}={value}");
    let new_settings = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        settings::save_setting(&conn, &key, &value).map_err(|e| {
            log::error!("[settings] failed to save '{key}': {e}");
            e.to_string()
        })?;
        // When SIT is turned off, cascade-reset the dependent tray settings so
        // the close-to-tray handler cannot hide the window with no icon to
        // restore from.
        if key == "tray_icon_enabled" && value == "false" {
            settings::save_setting(&conn, "min_to_tray", "false").map_err(|e| e.to_string())?;
            settings::save_setting(&conn, "min_to_tray_on_close", "false").map_err(|e| e.to_string())?;
        }
        settings::load(&conn).map_err(|e| {
            log::error!("[settings] failed to reload after save: {e}");
            e.to_string()
        })?
    };

    // Apply verbose_logging change immediately without a restart.
    if key == "verbose_logging" {
        if new_settings.verbose_logging {
            log::set_max_level(LevelFilter::Debug);
            log::info!("Verbose logging enabled — log level set to DEBUG");
        } else {
            log::set_max_level(LevelFilter::Info);
            log::info!("Verbose logging disabled — log level set to INFO");
        }
    }

    // Keep the timer engine in sync when time-related settings change.
    timer.apply_settings(new_settings.clone());

    // Broadcast an updated snapshot so the frontend immediately reflects any
    // changed settings (round count, durations, etc.) regardless of timer
    // state.  The timer:reset handler only calls timerState.set(), so emitting
    // while running does not interrupt the countdown; the next timer:tick
    // event will reconcile total_secs from the engine within one second.
    app.emit("timer:reset", &timer.get_snapshot()).ok();

    // Propagate volume and tick-sound changes to the audio engine (optional state).
    if let Some(audio) = app.try_state::<Arc<AudioManager>>() {
        audio.apply_settings(&new_settings);
    }

    // Apply always-on-top window flag immediately when the setting changes,
    // accounting for the current round type so break_always_on_top takes
    // effect without waiting for the next round transition.
    if matches!(key.as_str(), "always_on_top" | "break_always_on_top") {
        if let Some(window) = app.get_webview_window("main") {
            let snap = timer.get_snapshot();
            let is_break = snap.round_type != "work";
            let effective_aot = new_settings.always_on_top
                && !(new_settings.break_always_on_top && is_break);
            let _ = window.set_always_on_top(effective_aot);
        }
    }

    // Sync tray countdown mode when the dial setting changes, then immediately
    // re-render the icon so it matches the dial without waiting for a timer event.
    if key == "dial_countdown" {
        *tray_state.countdown_mode.lock().unwrap() = new_settings.dial_countdown;
        let snap = timer.get_snapshot();
        let progress = if snap.total_secs > 0 {
            snap.elapsed_secs as f32 / snap.total_secs as f32
        } else {
            0.0
        };
        tray::update_icon(&tray_state, &snap.round_type, snap.is_paused, progress);
    }

    // Update tray icon colors when the active theme changes.
    if matches!(key.as_str(), "theme_mode" | "theme_light" | "theme_dark") {
        let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let tray_theme_name = match new_settings.theme_mode.as_str() {
            "dark" => &new_settings.theme_dark,
            _ => &new_settings.theme_light,
        };
        if let Some(theme) = themes::find(&data_dir, tray_theme_name) {
            *tray_state.colors.lock().unwrap() = tray::TrayColors::from_colors_map(&theme.colors);
            let snap = timer.get_snapshot();
            let progress = if snap.total_secs > 0 {
                snap.elapsed_secs as f32 / snap.total_secs as f32
            } else {
                0.0
            };
            tray::update_icon(&tray_state, &snap.round_type, snap.is_paused, progress);
        }
    }

    // Create or destroy the tray when tray_icon_enabled or min_to_tray changes.
    // The tray exists when either flag is true.
    // On Linux, spawn tray creation on a background thread to avoid blocking
    // the main thread on KDE Plasma 6 / Wayland (D-Bus StatusNotifier hang).
    if matches!(key.as_str(), "tray_icon_enabled" | "min_to_tray") {
        if new_settings.tray_icon_enabled || new_settings.min_to_tray {
            #[cfg(target_os = "linux")]
            {
                let app_handle = app.clone();
                let ts = Arc::clone(&tray_state);
                std::thread::spawn(move || {
                    tray::create_tray(&app_handle, &ts);
                });
            }
            #[cfg(not(target_os = "linux"))]
            tray::create_tray(&app, &tray_state);
        } else {
            tray::destroy_tray(&tray_state);
        }
    }

    // Re-register global shortcuts when any shortcut key changes or the enabled flag toggles.
    if matches!(key.as_str(), "shortcut_toggle" | "shortcut_reset" | "shortcut_skip" | "shortcut_restart" | "global_shortcuts_enabled") {
        shortcuts::register_all(&app, &new_settings);
    }

    // Start or stop the WebSocket server when the enabled flag or port changes.
    if matches!(key.as_str(), "websocket_enabled" | "websocket_port") {
        let ws = Arc::clone(&*ws_state);
        let port = new_settings.websocket_port;
        let enabled = new_settings.websocket_enabled;
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            // Always stop the old server first.
            websocket::stop(&ws).await;
            if enabled {
                websocket::start(port, app_clone, &ws).await;
            }
        });
    }

    // Publish domain events; the achievement subscriber handles recording + evaluation.
    if let Some(bus) = app.try_state::<Arc<crate::bus::EventBus>>() {
        bus.publish(crate::bus::AppEvent::SettingsSaved { key: key.clone() }, &app);

        if matches!(key.as_str(), "theme_light" | "theme_dark") {
            let name = if key == "theme_light" {
                new_settings.theme_light.clone()
            } else {
                new_settings.theme_dark.clone()
            };
            bus.publish(crate::bus::AppEvent::ThemeApplied { name }, &app);
        }
        if key == "language" {
            bus.publish(
                crate::bus::AppEvent::LanguageChanged {
                    language: new_settings.language.clone(),
                },
                &app,
            );
        }
        if key == "websocket_enabled" && new_settings.websocket_enabled {
            bus.publish(crate::bus::AppEvent::WebSocketEnabled, &app);
        }
    }

    app.emit("settings:changed", &new_settings).ok();
    Ok(new_settings)
}

// ---------------------------------------------------------------------------
// CMD-06 — Shortcuts command
// ---------------------------------------------------------------------------

/// Re-register all global shortcuts from the current settings.
/// The frontend can call this after bulk-updating shortcut settings.
#[tauri::command]
pub fn shortcuts_reload(db: State<'_, DbState>, app: AppHandle) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let s = settings::load(&conn).map_err(|e| e.to_string())?;
    shortcuts::register_all(&app, &s);
    Ok(())
}

/// Reset all settings to factory defaults and return the resulting settings.
#[tauri::command]
pub fn settings_reset_defaults(
    db: State<'_, DbState>,
    timer: State<'_, TimerController>,
    tray_state: State<'_, Arc<TrayState>>,
    app: AppHandle,
) -> Result<Settings, String> {
    log::info!("[settings] reset to defaults");
    let new_settings = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        // Delete all rows so seed_defaults can insert fresh defaults.
        conn.execute("DELETE FROM settings", [])
            .map_err(|e| e.to_string())?;
        settings::seed_defaults(&conn).map_err(|e| e.to_string())?;
        settings::load(&conn).map_err(|e| e.to_string())?
    };

    timer.apply_settings(new_settings.clone());
    *tray_state.countdown_mode.lock().unwrap() = new_settings.dial_countdown;

    // Broadcast a reset snapshot so the frontend dial and display reflect the
    // restored default durations without requiring the user to manually reset.
    {
        let snap = timer.get_snapshot();
        if !snap.is_running && !snap.is_paused {
            app.emit("timer:reset", &snap).ok();
        }
    }

    // After reset, defaults have tray_icon_enabled=false and min_to_tray=false,
    // so destroy any active tray icon.
    tray::destroy_tray(&tray_state);

    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    // Clear custom alert sounds: delete files from disk and reset in-memory paths.
    if let Some(audio_state) = app.try_state::<Arc<AudioManager>>() {
        let audio_dir = data_dir.join("audio");
        for stem in [audio::STEM_WORK, audio::STEM_SHORT, audio::STEM_LONG] {
            if let Ok(entries) = std::fs::read_dir(&audio_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let p = entry.path();
                    if p.file_stem().and_then(|s| s.to_str()) == Some(stem) {
                        let _ = std::fs::remove_file(&p);
                    }
                }
            }
        }
        audio_state.clear_custom_path("work_alert");
        audio_state.clear_custom_path("short_break_alert");
        audio_state.clear_custom_path("long_break_alert");
        log::info!("[audio] custom sounds cleared on settings reset");
    }

    let tray_theme_name = match new_settings.theme_mode.as_str() {
        "dark" => &new_settings.theme_dark,
        _ => &new_settings.theme_light,
    };
    if let Some(theme) = themes::find(&data_dir, tray_theme_name) {
        *tray_state.colors.lock().unwrap() = tray::TrayColors::from_colors_map(&theme.colors);
    }
    shortcuts::register_all(&app, &new_settings);
    app.emit("settings:changed", &new_settings).ok();
    Ok(new_settings)
}

// ---------------------------------------------------------------------------
// CMD-03 — Theme commands
// ---------------------------------------------------------------------------

/// List all available themes (17 bundled + any user-created ones).
#[tauri::command]
pub fn themes_list(app: AppHandle) -> Result<Vec<Theme>, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(themes::list_all(&data_dir))
}

// ---------------------------------------------------------------------------
// CMD-04 — Sessions commands
// ---------------------------------------------------------------------------

/// Deletes all rows from the `sessions` table and any achievements/events that are
/// derived purely from session data.  Achievements earned through other means
/// (e.g. theme creation, app launches, shortcuts) are intentionally preserved.
/// Emits `sessions:cleared` and `achievements:cleared` so open windows can refresh.
#[tauri::command]
pub fn sessions_clear(db: State<'_, DbState>, app: AppHandle) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let n = conn.execute("DELETE FROM sessions", []).map_err(|e| {
        log::error!("[sessions] failed to clear history: {e}");
        e.to_string()
    })?;

    // Remove events tied to sessions; leave unrelated events intact.
    conn.execute(
        "DELETE FROM events WHERE name = ?1",
        rusqlite::params![crate::achievements::event::SESSION_COMPLETED],
    ).map_err(|e| {
        log::error!("[sessions] failed to clear session events: {e}");
        e.to_string()
    })?;

    log::info!("[sessions] cleared {n} session rows");
    drop(conn); // Release DB lock before publishing — subscriber re-acquires it.

    // The achievement subscriber handles selective achievement deletion.
    if let Some(bus) = app.try_state::<Arc<crate::bus::EventBus>>() {
        bus.publish(crate::bus::AppEvent::SessionsCleared, &app);
    }

    app.emit("sessions:cleared", ()).ok();
    app.emit("achievements:cleared", ()).ok();
    Ok(())
}

// ---------------------------------------------------------------------------
// CMD-06 — Achievement commands
// ---------------------------------------------------------------------------

/// Return the full list of achievements with earned status and progress.
/// Silently catches up on any achievements earned from pre-existing session
/// data (e.g. sessions completed before the achievement system was added).
#[tauri::command]
pub fn achievements_get_all(
    db: State<'_, DbState>,
    app: AppHandle,
) -> Result<Vec<AchievementView>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    // Silently catch up on any retroactively-earned achievements (no toast).
    achievements_eval::check_all_achievements(&conn, &app);
    Ok(achievements_eval::build_all_views(&conn, &app))
}

/// Record a named achievement event from the frontend (e.g. settings_opened,
/// shortcut_used).  Fires toasts for any newly-unlocked achievements.
#[tauri::command]
pub fn achievement_record_event(
    db: State<'_, DbState>,
    app: AppHandle,
    name: String,
    payload: Option<String>,
) {
    // DB lock must be released before notify_and_spawn_toast — it re-acquires the lock.
    let newly_unlocked = if let Ok(conn) = db.lock() {
        achievements_eval::record_event(&conn, &app, &name, payload.as_deref())
    } else {
        return;
    };
    if !newly_unlocked.is_empty() {
        achievements_eval::notify_and_spawn_toast(newly_unlocked, &app, false);
    }
}

// CMD-05 — Stats commands
// ---------------------------------------------------------------------------

/// Batched stats for Today + This Week tabs (minimises IPC round-trips).
#[tauri::command]
pub fn stats_get_detailed(db: State<'_, DbState>) -> Result<DetailedStats, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let today = queries::get_daily_stats(&conn).map_err(|e| {
        log::error!("[stats] failed to query daily stats: {e}");
        e.to_string()
    })?;
    let week = queries::get_weekly_stats(&conn).map_err(|e| {
        log::error!("[stats] failed to query weekly stats: {e}");
        e.to_string()
    })?;
    let streak = queries::get_streak(&conn).map_err(|e| {
        log::error!("[stats] failed to query streak: {e}");
        e.to_string()
    })?;
    Ok(DetailedStats { today, week, streak })
}

/// Heatmap data + lifetime totals for the All Time tab.
#[tauri::command]
pub fn stats_get_heatmap(db: State<'_, DbState>) -> Result<HeatmapStats, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let entries = queries::get_heatmap_data(&conn).map_err(|e| {
        log::error!("[stats] failed to query heatmap data: {e}");
        e.to_string()
    })?;
    let raw = queries::get_all_time_stats(&conn).map_err(|e| {
        log::error!("[stats] failed to query all-time stats: {e}");
        e.to_string()
    })?;
    let streak = queries::get_streak(&conn).map_err(|e| {
        log::error!("[stats] failed to query streak for heatmap: {e}");
        e.to_string()
    })?;
    Ok(HeatmapStats {
        entries,
        total_rounds: raw.completed_work_sessions as u32,
        total_hours: (raw.total_work_secs / 3600) as u32,
        longest_streak: streak.longest,
    })
}

// ---------------------------------------------------------------------------
// CMD-05 — Window commands
// ---------------------------------------------------------------------------

/// Show or hide the main window.
#[tauri::command]
pub fn window_set_visibility(visible: bool, app: AppHandle) -> Result<(), String> {
    log::debug!("[window] set visibility={visible}");
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    if visible {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// CMD-07 — Audio commands
// ---------------------------------------------------------------------------

/// Copy a user-selected audio file into the app config dir for the given cue slot.
///
/// `cue` must be one of: `"work_alert"`, `"short_break_alert"`, `"long_break_alert"`.
/// `src_path` is the full path to the file chosen by the user.
///
/// The file is stored with a fixed stem (e.g. `custom_work_alert.mp3`) so that
/// selecting a new file for the same slot automatically replaces the old one —
/// no orphan files accumulate.
///
/// Returns the original filename for display in the UI.
#[tauri::command]
pub fn audio_set_custom(
    cue: String,
    src_path: String,
    db: State<'_, DbState>,
    app: AppHandle,
) -> Result<String, String> {
    let audio_state = app
        .try_state::<Arc<AudioManager>>()
        .ok_or_else(|| "audio engine is not available".to_string())?;

    let stem = cue_to_stem(&cue)?;

    let audio_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("audio");
    std::fs::create_dir_all(&audio_dir).map_err(|e| e.to_string())?;

    let src = std::path::Path::new(&src_path);
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp3");

    // Remove any existing custom file for this slot (preserves zero orphans).
    if let Ok(entries) = std::fs::read_dir(&audio_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.file_stem().and_then(|s| s.to_str()) == Some(stem) {
                let _ = std::fs::remove_file(&p);
            }
        }
    }

    let dest = audio_dir.join(format!("{stem}.{ext}"));
    std::fs::copy(src, &dest).map_err(|e| e.to_string())?;

    audio_state.set_custom_path(&cue, dest);

    let display_name = src
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("custom")
        .to_string();

    // Persist the original filename so it survives restarts.
    let name_key = cue_to_name_key(&cue)?;
    let conn = db.lock().map_err(|e| e.to_string())?;
    settings::save_setting(&conn, name_key, &display_name).map_err(|e| e.to_string())?;

    log::info!("[audio] custom sound set cue={cue} file={display_name}");
    drop(conn); // Release DB lock before publishing to bus.

    if let Some(bus) = app.try_state::<Arc<crate::bus::EventBus>>() {
        bus.publish(crate::bus::AppEvent::AudioCustomLoaded, &app);
    }

    Ok(display_name)
}

/// Restore the built-in sound for the given cue slot by deleting the custom file.
#[tauri::command]
pub fn audio_clear_custom(
    cue: String,
    db: State<'_, DbState>,
    app: AppHandle,
) -> Result<(), String> {
    let audio_state = app
        .try_state::<Arc<AudioManager>>()
        .ok_or_else(|| "audio engine is not available".to_string())?;

    let stem = cue_to_stem(&cue)?;

    let audio_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("audio");

    if let Ok(entries) = std::fs::read_dir(&audio_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.file_stem().and_then(|s| s.to_str()) == Some(stem) {
                std::fs::remove_file(&p).map_err(|e| e.to_string())?;
            }
        }
    }

    audio_state.clear_custom_path(&cue);

    // Remove the persisted display name.
    let name_key = cue_to_name_key(&cue)?;
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM settings WHERE key = ?1", rusqlite::params![name_key])
        .map_err(|e| e.to_string())?;

    log::info!("[audio] custom sound cleared cue={cue}");
    Ok(())
}

/// Return the display names of any currently configured custom audio files.
/// Fields are `null` when the built-in sound is in use for that slot.
#[tauri::command]
pub fn audio_get_custom_info(
    db: State<'_, DbState>,
    app: AppHandle,
) -> Result<audio::CustomAudioInfo, String> {
    let audio_state = app
        .try_state::<Arc<AudioManager>>()
        .ok_or_else(|| "audio engine is not available".to_string())?;

    // Start from the AudioManager's paths (determines which slots are active).
    let mut info = audio_state.get_custom_info();

    // Override each active slot's name with the persisted original filename.
    let conn = db.lock().map_err(|e| e.to_string())?;
    let override_name = |stored: &Option<String>, key: &str| -> Option<String> {
        stored.as_ref()?; // slot not active — leave as None
        settings::get_setting(&conn, key).or_else(|| stored.clone())
    };
    info.work_alert = override_name(&info.work_alert, "custom_work_alert_name");
    info.short_break_alert = override_name(&info.short_break_alert, "custom_short_break_alert_name");
    info.long_break_alert = override_name(&info.long_break_alert, "custom_long_break_alert_name");

    Ok(info)
}

// ---------------------------------------------------------------------------
// CMD-08 — Notification command
// ---------------------------------------------------------------------------

/// Show a desktop notification with the given title and body.
///
/// String construction (including translation) is the caller's (frontend's)
/// responsibility. This command is a thin platform-dispatch wrapper.
#[tauri::command]
pub fn notification_show(title: String, body: String, app: AppHandle) {
    notifications::show(&app, &title, &body);
}

// ---------------------------------------------------------------------------
// CMD-09 — Diagnostic log commands
// ---------------------------------------------------------------------------

/// Open the application log directory in the OS file manager.
#[tauri::command]
pub fn open_log_dir(app: AppHandle) {
    match app.path().app_log_dir() {
        Ok(log_dir) => {
            if let Err(e) = tauri_plugin_opener::open_path(&log_dir, None::<&str>) {
                log::warn!("[log] failed to open log dir {}: {e}", log_dir.display());
            }
        }
        Err(e) => log::warn!("[log] failed to resolve log dir: {e}"),
    }
}

/// Return the compile-time build version string.
#[tauri::command]
pub fn app_version() -> &'static str {
    env!("APP_BUILD_VERSION")
}

// ---------------------------------------------------------------------------
// CMD-10 — Platform commands
// ---------------------------------------------------------------------------

/// Returns whether the app has macOS Accessibility permission.
/// On macOS, calls AXIsProcessTrusted() from the ApplicationServices framework.
/// On all other platforms, always returns true.
#[tauri::command]
pub fn accessibility_trusted() -> bool {
    #[cfg(target_os = "macos")]
    {
        #[link(name = "ApplicationServices", kind = "framework")]
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

/// Returns whether the system tray is supported on this platform/install.
/// On Linux, probes for libayatana-appindicator3 / libappindicator3 at runtime.
/// On macOS and Windows, always returns true.
#[tauri::command]
pub fn tray_supported() -> bool {
    #[cfg(target_os = "linux")]
    {
        tray::appindicator_available()
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

/// Return the application log directory path as a string.
#[tauri::command]
pub fn get_log_dir(app: AppHandle) -> Result<String, String> {
    app.path()
        .app_log_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| {
            log::warn!("[log] failed to resolve log dir: {e}");
            e.to_string()
        })
}

// ---------------------------------------------------------------------------
// CMD-11 — Updater commands
// ---------------------------------------------------------------------------

/// Information about an available update returned to the frontend.
#[derive(serde::Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub body: Option<String>,
    pub date: Option<String>,
}

/// Check whether a newer version is available.
/// Returns `Some(UpdateInfo)` when an update is available, or `None` when
/// the running version is already the latest.
/// Errors (e.g. network failure) are surfaced as a string so the frontend
/// can display a non-blocking message.
#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    use tauri_plugin_updater::UpdaterExt;
    log::info!("[updater] checking for updates");
    let updater = app.updater().map_err(|e| {
        log::error!("[updater] failed to build updater: {e}");
        e.to_string()
    })?;
    match updater.check().await {
        Ok(Some(update)) => {
            log::info!("[updater] update available: v{}", update.version);
            Ok(Some(UpdateInfo {
                version: update.version.clone(),
                body: update.body.clone(),
                date: update.date.map(|d| d.to_string()),
            }))
        }
        Ok(None) => {
            log::info!("[updater] already up to date");
            Ok(None)
        }
        Err(e) => {
            log::warn!("[updater] update check failed: {e}");
            Err(e.to_string())
        }
    }
}

/// Download, verify, and install the pending update, then relaunch immediately.
/// Should only be called after `check_update` has returned `Some(UpdateInfo)`.
#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;
    log::info!("[updater] install requested — checking for update");
    let updater = app.updater().map_err(|e| {
        log::error!("[updater] failed to build updater: {e}");
        e.to_string()
    })?;
    let update = updater
        .check()
        .await
        .map_err(|e| {
            log::error!("[updater] update check failed during install: {e}");
            e.to_string()
        })?
        .ok_or_else(|| {
            log::warn!("[updater] install_update called but no update is available");
            "No update available".to_string()
        })?;
    log::info!("[updater] downloading and installing v{}", update.version);
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| {
            log::error!("[updater] download/install failed: {e}");
            e.to_string()
        })?;
    log::info!("[updater] install complete — relaunching");
    app.restart();
}

fn cue_to_stem(cue: &str) -> Result<&'static str, String> {
    match cue {
        "work_alert" => Ok(audio::STEM_WORK),
        "short_break_alert" => Ok(audio::STEM_SHORT),
        "long_break_alert" => Ok(audio::STEM_LONG),
        _ => Err(format!("unknown audio cue: '{cue}'")),
    }
}

fn cue_to_name_key(cue: &str) -> Result<&'static str, String> {
    match cue {
        "work_alert" => Ok("custom_work_alert_name"),
        "short_break_alert" => Ok("custom_short_break_alert_name"),
        "long_break_alert" => Ok("custom_long_break_alert_name"),
        _ => Err(format!("unknown audio cue: '{cue}'")),
    }
}

// ---------------------------------------------------------------------------
// Stats payload types
// ---------------------------------------------------------------------------

/// Batched payload for Today + This Week tabs.
#[derive(serde::Serialize)]
pub struct DetailedStats {
    pub today: queries::DailyStats,
    pub week: Vec<queries::DayStat>,
    pub streak: queries::StreakInfo,
}

/// Payload for the All Time tab.
#[derive(serde::Serialize)]
pub struct HeatmapStats {
    pub entries: Vec<queries::HeatmapEntry>,
    pub total_rounds: u32,
    pub total_hours: u32,
    pub longest_streak: u32,
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use crate::db::migrations;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        conn
    }

    fn seed_sessions(conn: &Connection) {
        conn.execute_batch("
            INSERT INTO sessions (started_at, ended_at, round_type, duration_secs, completed)
            VALUES (1000, 1060, 'work', 60, 1),
                   (2000, 2300, 'short-break', 300, 1);
        ").unwrap();
    }

    #[test]
    fn sessions_clear_removes_all_rows() {
        let conn = setup();
        seed_sessions(&conn);
        let before: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0)).unwrap();
        assert_eq!(before, 2);

        let n = conn.execute("DELETE FROM sessions", []).unwrap();
        assert_eq!(n, 2);

        let after: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0)).unwrap();
        assert_eq!(after, 0);
    }

    #[test]
    fn sessions_clear_on_empty_table_returns_zero() {
        let conn = setup();
        let n = conn.execute("DELETE FROM sessions", []).unwrap();
        assert_eq!(n, 0);
    }
}

