use rusqlite::Connection;
use tauri::{AppHandle, Emitter, Manager};

use crate::db::queries;
use crate::achievements::{build_view, AchievementView, ProgressKind, ACHIEVEMENTS};
use crate::settings;

// ---------------------------------------------------------------------------
// Unlock event payload
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, Clone)]
pub struct AchievementUnlockedPayload {
    pub ids: Vec<String>,
    pub count: u32,
}

// ---------------------------------------------------------------------------
// Public entry point — record an event and evaluate relevant achievements
// ---------------------------------------------------------------------------

/// Record a named event in the log, then check every achievement that listens
/// to that event.  Returns the IDs of any newly-unlocked achievements.
///
/// The DB lock must be held by the caller.  Do NOT call
/// `notify_and_spawn_toast` while the lock is held — release it first, then
/// call `notify_and_spawn_toast` with the returned IDs.
pub fn record_event(
    conn: &Connection,
    app: &AppHandle,
    name: &str,
    payload: Option<&str>,
) -> Vec<String> {
    if let Err(e) = queries::insert_event(conn, name, payload) {
        log::warn!("[achievements] failed to insert event '{name}': {e}");
    }
    on_event(name, conn, app)
}

/// Evaluate every achievement whose trigger list includes `event_name`.
/// Inserts newly-earned achievements into the DB and returns their IDs.
pub fn on_event(event_name: &str, conn: &Connection, app: &AppHandle) -> Vec<String> {
    let earned = match queries::get_earned_achievement_ids(conn) {
        Ok(set) => set,
        Err(e) => {
            log::warn!("[achievements] failed to load earned ids: {e}");
            return vec![];
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let mut newly_unlocked = vec![];

    for def in ACHIEVEMENTS {
        if !def.triggers.contains(&event_name) { continue; }
        if earned.contains(def.id) { continue; }

        if criteria_met(def.id, conn, app) {
            if let Err(e) = queries::insert_achievement(conn, def.id, now) {
                log::warn!("[achievements] failed to insert {}: {e}", def.id);
            } else {
                log::info!("[achievements] unlocked: {}", def.id);
                newly_unlocked.push(def.id.to_string());
            }
        }
    }

    newly_unlocked
}

/// Evaluate ALL achievements regardless of trigger — used for retroactive
/// catch-up when opening the achievements gallery.  Never fires a toast.
pub fn check_all_achievements(conn: &Connection, app: &AppHandle) {
    let earned = match queries::get_earned_achievement_ids(conn) {
        Ok(set) => set,
        Err(e) => {
            log::warn!("[achievements] retroactive check failed to load earned ids: {e}");
            return;
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for def in ACHIEVEMENTS {
        if earned.contains(def.id) { continue; }
        if criteria_met(def.id, conn, app) {
            let _ = queries::insert_achievement(conn, def.id, now);
        }
    }
}

// ---------------------------------------------------------------------------
// Criteria — one arm per achievement, querying the DB directly
// ---------------------------------------------------------------------------

fn criteria_met(id: &str, conn: &Connection, app: &AppHandle) -> bool {
    match id {
        // --- Milestone ---
        "the_seed" => {
            queries::get_all_time_stats(conn)
                .map(|s| s.completed_work_sessions >= 1)
                .unwrap_or(false)
        }
        "hat_trick" => {
            queries::get_hat_trick_max_day(conn)
                .map(|n| n >= 3)
                .unwrap_or(false)
        }
        "the_centurion" => {
            queries::get_all_time_stats(conn)
                .map(|s| s.completed_work_sessions >= 100)
                .unwrap_or(false)
        }
        "tomato_baron" => {
            queries::get_all_time_stats(conn)
                .map(|s| s.completed_work_sessions >= 500)
                .unwrap_or(false)
        }
        "tomato_tycoon" => {
            queries::get_all_time_stats(conn)
                .map(|s| s.completed_work_sessions >= 1000)
                .unwrap_or(false)
        }
        "time_lord" => {
            queries::get_all_time_stats(conn)
                .map(|s| s.total_work_secs >= 360_000)
                .unwrap_or(false)
        }
        // --- Habit ---
        "on_a_roll" => {
            queries::get_streak(conn)
                .map(|s| s.longest >= 3)
                .unwrap_or(false)
        }
        "week_warrior" => {
            queries::get_streak(conn)
                .map(|s| s.longest >= 7)
                .unwrap_or(false)
        }
        "month_of_zen" => {
            queries::get_streak(conn)
                .map(|s| s.longest >= 30)
                .unwrap_or(false)
        }
        "comeback_kid" => queries::get_comeback_kid(conn).unwrap_or(false),
        // --- Discovery ---
        "early_bird"        => queries::get_early_sessions(conn).unwrap_or(false),
        "midnight_oil"      => queries::get_midnight_sessions(conn).unwrap_or(false),
        "perfect_day"       => queries::get_perfect_day_exists(conn).unwrap_or(false),
        "the_long_haul"     => queries::get_long_break_completed(conn).unwrap_or(false),
        "creature_of_habit" => queries::get_creature_of_habit(conn).unwrap_or(false),
        "theme_artist" => {
            // Check the events log first (fast path); fall back to filesystem.
            if queries::count_events(conn, crate::achievements::event::THEME_CREATED) > 0 {
                return true;
            }
            app.path().app_data_dir().map(|dir| {
                !crate::themes::load_custom(&dir.join("themes")).is_empty()
            }).unwrap_or(false)
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Progress for unearned milestone / count achievements
// ---------------------------------------------------------------------------

pub fn progress_for(id: &str, conn: &Connection) -> Option<u32> {
    match id {
        "the_seed" | "hat_trick" | "the_centurion" | "tomato_baron" | "tomato_tycoon" => {
            queries::get_all_time_stats(conn).ok()
                .map(|s| s.completed_work_sessions as u32)
        }
        "time_lord" => {
            queries::get_all_time_stats(conn).ok()
                .map(|s| (s.total_work_secs / 3600) as u32)
        }
        "on_a_roll" | "week_warrior" | "month_of_zen" => {
            queries::get_streak(conn).ok().map(|s| s.longest)
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Build full view list for the gallery command
// ---------------------------------------------------------------------------

pub fn build_all_views(conn: &Connection, _app: &AppHandle) -> Vec<AchievementView> {
    ACHIEVEMENTS
        .iter()
        .map(|def| {
            let unlocked_at = queries::get_achievement_unlocked_at(conn, def.id)
                .unwrap_or(None);
            let progress_current = if unlocked_at.is_none() {
                match def.progress {
                    ProgressKind::Count { .. } => progress_for(def.id, conn),
                    ProgressKind::Binary => None,
                }
            } else {
                None
            };
            build_view(def, unlocked_at, progress_current)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Notification + toast
// ---------------------------------------------------------------------------

/// Emit the `achievement:unlocked` event and optionally spawn the toast window.
/// Call this AFTER releasing the DB lock (toast spawn re-acquires the lock).
pub fn notify_and_spawn_toast(newly_unlocked: Vec<String>, app: &AppHandle) {
    let count = newly_unlocked.len() as u32;
    let payload = AchievementUnlockedPayload {
        ids: newly_unlocked.clone(),
        count,
    };
    if let Err(e) = app.emit("achievement:unlocked", &payload) {
        log::warn!("[achievements] failed to emit event: {e}");
    }

    let notifications_on = app
        .try_state::<crate::db::DbState>()
        .and_then(|db| db.lock().ok().map(|conn| {
            settings::get_setting(&conn, "achievement_notifications")
                .map(|v| v == "true")
                .unwrap_or(true)
        }))
        .unwrap_or(true);

    if notifications_on {
        crate::achievements::toast::spawn_toast_window(app, &newly_unlocked, count);
    }
}
