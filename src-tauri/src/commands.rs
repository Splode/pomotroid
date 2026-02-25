/// All #[tauri::command] functions exposed to the Svelte frontend via Tauri IPC.
///
/// Commands are grouped by domain: Timer, Settings, Themes, Stats.
/// Each command returns `Result<T, String>` so errors surface cleanly in JS.
use tauri::{AppHandle, Emitter, Manager, State};

use std::sync::Arc;

use crate::audio::AudioManager;
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
    settings::load(&conn).map_err(|e| e.to_string())
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
    let new_settings = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        settings::save_setting(&conn, &key, &value).map_err(|e| e.to_string())?;
        settings::load(&conn).map_err(|e| e.to_string())?
    };

    // Keep the timer engine in sync when time-related settings change.
    timer.apply_settings(new_settings.clone());

    // If the timer is idle, broadcast a reset snapshot immediately so the
    // frontend's dial and display reflect the new duration without requiring
    // the user to manually start/reset the timer.
    {
        let snap = timer.get_snapshot();
        if !snap.is_running && !snap.is_paused {
            app.emit("timer:reset", &snap).ok();
        }
    }

    // Propagate volume and tick-sound changes to the audio engine (optional state).
    if let Some(audio) = app.try_state::<Arc<AudioManager>>() {
        audio.apply_settings(&new_settings);
    }

    // Apply always-on-top window flag immediately when the setting changes.
    if matches!(key.as_str(), "always_on_top" | "break_always_on_top") {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.set_always_on_top(new_settings.always_on_top);
        }
    }

    // Create or destroy the tray based on the min_to_tray setting.
    if key == "min_to_tray" {
        if value == "true" {
            tray::create_tray(&app, &tray_state);
        } else {
            tray::destroy_tray(&tray_state);
        }
    }

    // Re-register global shortcuts when any shortcut key changes.
    if matches!(key.as_str(), "shortcut_toggle" | "shortcut_reset" | "shortcut_skip") {
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
    app: AppHandle,
) -> Result<Settings, String> {
    let new_settings = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        // Delete all rows so seed_defaults can insert fresh defaults.
        conn.execute("DELETE FROM settings", [])
            .map_err(|e| e.to_string())?;
        settings::seed_defaults(&conn).map_err(|e| e.to_string())?;
        settings::load(&conn).map_err(|e| e.to_string())?
    };

    timer.apply_settings(new_settings.clone());
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

/// Apply a theme by name: persist the selection and return the Theme object
/// so the frontend can immediately apply the CSS custom properties.
#[tauri::command]
pub fn theme_apply(
    name: String,
    app: AppHandle,
    db: State<'_, DbState>,
) -> Result<Theme, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let theme = themes::find(&data_dir, &name)
        .ok_or_else(|| format!("theme '{name}' not found"))?;

    // Persist the selection and load updated settings to broadcast.
    let updated_settings = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        settings::save_setting(&conn, "theme", &theme.name).map_err(|e| e.to_string())?;
        settings::load(&conn).map_err(|e| e.to_string())?
    };

    // Notify all windows so they can re-apply the new theme CSS.
    app.emit("settings:changed", &updated_settings).ok();

    Ok(theme)
}

// ---------------------------------------------------------------------------
// CMD-04 — Stats commands
// ---------------------------------------------------------------------------

/// All-time statistics aggregated from the sessions table.
#[tauri::command]
pub fn stats_get_all_time(db: State<'_, DbState>) -> Result<AllTimeStats, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let raw = queries::get_all_time_stats(&conn).map_err(|e| e.to_string())?;
    Ok(AllTimeStats {
        total_work_rounds: raw.completed_work_sessions as u32,
        total_work_minutes: (raw.total_work_secs / 60) as u32,
    })
}

/// Current-session statistics from the timer controller.
#[tauri::command]
pub fn stats_get_session(timer: State<'_, TimerController>) -> SessionStats {
    let snap = timer.get_snapshot();
    SessionStats {
        session_work_rounds: snap.work_round_number,
    }
}

// ---------------------------------------------------------------------------
// CMD-05 — Window commands
// ---------------------------------------------------------------------------

/// Show or hide the main window.
#[tauri::command]
pub fn window_set_visibility(visible: bool, app: AppHandle) -> Result<(), String> {
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

/// Set or clear the always-on-top flag for the main window.
#[tauri::command]
pub fn window_set_always_on_top(on_top: bool, app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    window.set_always_on_top(on_top).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Stats payload types
// ---------------------------------------------------------------------------

#[derive(serde::Serialize)]
pub struct AllTimeStats {
    pub total_work_rounds: u32,
    pub total_work_minutes: u32,
}

#[derive(serde::Serialize)]
pub struct SessionStats {
    /// Work rounds completed in this session (current round_number).
    pub session_work_rounds: u32,
}
