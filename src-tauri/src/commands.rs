/// All #[tauri::command] functions exposed to the Svelte frontend via Tauri IPC.
///
/// Commands are grouped by domain: Timer, Settings, Themes, Stats.
/// Each command returns `Result<T, String>` so errors surface cleanly in JS.
use tauri::{AppHandle, Emitter, Manager, State};

use std::sync::Arc;

use crate::audio::{self, AudioManager};
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
    tray_state: State<'_, Arc<TrayState>>,
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
    *tray_state.countdown_mode.lock().unwrap() = new_settings.dial_countdown;
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if let Some(theme) = themes::find(&data_dir, &new_settings.theme) {
        *tray_state.colors.lock().unwrap() = tray::TrayColors::from_colors_map(&theme.colors);
    }
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
    tray_state: State<'_, Arc<TrayState>>,
    timer: State<'_, TimerController>,
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

    // Update tray colors and immediately re-render the icon for the new theme.
    *tray_state.colors.lock().unwrap() = tray::TrayColors::from_colors_map(&theme.colors);
    let snap = timer.get_snapshot();
    let progress = if snap.total_secs > 0 {
        snap.elapsed_secs as f32 / snap.total_secs as f32
    } else {
        0.0
    };
    tray::update_icon(&tray_state, &snap.round_type, snap.is_paused, progress);

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
    Ok(display_name)
}

/// Restore the built-in sound for the given cue slot by deleting the custom file.
#[tauri::command]
pub fn audio_clear_custom(cue: String, app: AppHandle) -> Result<(), String> {
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
    Ok(())
}

/// Return the display names of any currently configured custom audio files.
/// Fields are `null` when the built-in sound is in use for that slot.
#[tauri::command]
pub fn audio_get_custom_info(app: AppHandle) -> Result<audio::CustomAudioInfo, String> {
    let audio_state = app
        .try_state::<Arc<AudioManager>>()
        .ok_or_else(|| "audio engine is not available".to_string())?;
    Ok(audio_state.get_custom_info())
}

fn cue_to_stem(cue: &str) -> Result<&'static str, String> {
    match cue {
        "work_alert" => Ok(audio::STEM_WORK),
        "short_break_alert" => Ok(audio::STEM_SHORT),
        "long_break_alert" => Ok(audio::STEM_LONG),
        _ => Err(format!("unknown audio cue: '{cue}'")),
    }
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
