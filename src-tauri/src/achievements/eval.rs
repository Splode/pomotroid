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

    // The Completionist is checked after every unlock, not on a specific trigger.
    if !newly_unlocked.is_empty()
        && !earned.contains("the_completionist")
        && criteria_met("the_completionist", conn, app)
    {
        if let Err(e) = queries::insert_achievement(conn, "the_completionist", now) {
            log::warn!("[achievements] failed to insert the_completionist: {e}");
        } else {
            log::info!("[achievements] unlocked: the_completionist");
            newly_unlocked.push("the_completionist".to_string());
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
// Bus subscriber — single registration point for all achievement side-effects
// ---------------------------------------------------------------------------

/// At first launch after the achievements system is introduced, synthesize
/// `THEME_APPLIED` events for users who already had a non-default theme set.
/// Idempotent: does nothing if any `THEME_APPLIED` event already exists.
fn backfill_inferred_events(conn: &Connection) {
    use crate::achievements::event;

    if queries::count_events(conn, event::THEME_APPLIED) > 0 {
        return;
    }

    const DEFAULT_LIGHT: &str = "Pomotroid Light";
    const DEFAULT_DARK: &str = "Pomotroid";

    let light = crate::settings::get_setting(conn, "theme_light").unwrap_or_default();
    let dark  = crate::settings::get_setting(conn, "theme_dark").unwrap_or_default();

    let non_default = [
        (light.as_str(),  DEFAULT_LIGHT),
        (dark.as_str(),   DEFAULT_DARK),
    ]
    .iter()
    .find_map(|&(val, def)| (!val.is_empty() && val != def).then(|| val.to_string()));

    if let Some(theme_name) = non_default {
        match queries::insert_event(conn, event::THEME_APPLIED, Some(theme_name.as_str())) {
            Ok(_) => log::info!(
                "[achievements] backfill: synthesized THEME_APPLIED for '{theme_name}'"
            ),
            Err(e) => log::warn!(
                "[achievements] backfill: failed to insert THEME_APPLIED: {e}"
            ),
        }
    }
}

/// Delete achievements that are exclusively driven by `SESSION_COMPLETED` events.
/// Called by the bus subscriber when `SessionsCleared` is received.
pub fn cleanup_session_achievements(conn: &Connection) {
    use crate::achievements::event;
    let mut count = 0usize;
    for def in crate::achievements::ACHIEVEMENTS {
        if def.triggers.iter().all(|&t| t == event::SESSION_COMPLETED) {
            count += conn
                .execute(
                    "DELETE FROM achievements WHERE id = ?1",
                    rusqlite::params![def.id],
                )
                .unwrap_or(0);
        }
    }
    log::info!("[achievements] removed {count} session-based achievement rows on sessions_clear");
}

/// Build the achievement handler to register with the `EventBus`.
///
/// The returned closure:
/// 1. Acquires the DB lock.
/// 2. Inserts event-log rows and evaluates any newly-earned achievements.
/// 3. **Drops the lock** before calling `notify_and_spawn_toast`, which
///    re-acquires the lock internally.
pub fn make_subscriber() -> impl Fn(&crate::bus::AppEvent, &AppHandle) + Send + Sync + 'static {
    move |event, app| {
        use crate::achievements::event as ev;
        use crate::bus::AppEvent;

        let Some(db) = app.try_state::<crate::db::DbState>() else { return };

        // ── DB lock scope ────────────────────────────────────────────────────
        // INVARIANT: `conn` must be dropped before `notify_and_spawn_toast()`
        // because that function re-acquires the DB lock.
        let newly = {
            let Ok(conn) = db.lock() else { return };

            match event {
                AppEvent::AppLaunched => {
                    // Retroactively award achievements that can be inferred from
                    // existing settings/data (silently — no toast for past actions).
                    backfill_inferred_events(&conn);
                    check_all_achievements(&conn, app);
                    record_event(&conn, app, ev::APP_LAUNCHED, None)
                }

                AppEvent::SessionCompleted {
                    round_type,
                    was_skipped,
                    elapsed_secs,
                    round_duration_secs,
                    in_tray,
                    always_on_top,
                    websocket_active,
                    silent,
                    compact,
                } => {
                    // Context flags are already pre-gated to false for non-work
                    // rounds at the publish site, so no round_type check needed here.
                    // Use record_event (not insert_event) so achievements triggered
                    // by these context events are evaluated immediately.
                    let mut newly = vec![];
                    if *in_tray {
                        newly.extend(record_event(&conn, app, ev::SESSION_TRAY, None));
                    }
                    if *always_on_top {
                        newly.extend(record_event(&conn, app, ev::SESSION_ALWAYS_ON_TOP, None));
                    }
                    if *websocket_active {
                        newly.extend(record_event(&conn, app, ev::SESSION_WEBSOCKET_ACTIVE, None));
                    }
                    if *silent {
                        newly.extend(record_event(&conn, app, ev::SESSION_SILENT, None));
                    }
                    if *compact {
                        newly.extend(record_event(&conn, app, ev::SESSION_COMPACT, None));
                    }
                    if *was_skipped
                        && round_type == "work"
                        && round_duration_secs.saturating_sub(*elapsed_secs) < 60
                    {
                        newly.extend(record_event(&conn, app, ev::SESSION_SKIPPED_LATE, None));
                    }
                    newly.extend(record_event(&conn, app, ev::SESSION_COMPLETED, None));
                    newly
                }

                AppEvent::SessionsCleared => {
                    cleanup_session_achievements(&conn);
                    vec![]
                }

                AppEvent::SettingsSaved { key } =>
                    record_event(&conn, app, ev::SETTINGS_SAVED, Some(key.as_str())),
                AppEvent::ThemeApplied { name } =>
                    record_event(&conn, app, ev::THEME_APPLIED, Some(name.as_str())),
                AppEvent::LanguageChanged { language } =>
                    record_event(&conn, app, ev::LANGUAGE_CHANGED, Some(language.as_str())),
                AppEvent::WebSocketEnabled =>
                    record_event(&conn, app, ev::WEBSOCKET_ENABLED, None),
                AppEvent::WebSocketMessage { msg_type } =>
                    record_event(&conn, app, ev::WEBSOCKET_MESSAGE, Some(msg_type.as_str())),
                AppEvent::ShortcutUsed { action } =>
                    record_event(&conn, app, ev::SHORTCUT_USED, Some(action.as_str())),
                AppEvent::AudioCustomLoaded =>
                    record_event(&conn, app, ev::AUDIO_CUSTOM_LOADED, None),
                AppEvent::ThemeCreated =>
                    record_event(&conn, app, ev::THEME_CREATED, None),
            }
            // `conn` guard drops here — lock released before notify below.
        };
        // ── DB lock scope end ────────────────────────────────────────────────

        // Signal the stats window that achievement data may have changed
        // (progress counts update even when no achievement is newly unlocked).
        if !matches!(event, AppEvent::SessionsCleared) {
            let _ = app.emit("achievement:progress", ());
        }

        // Mute the toast chime when triggered by session completion — the round
        // completion sound already plays at this moment.
        let mute_chime = matches!(event, AppEvent::SessionCompleted { .. });

        // Suppress the toast for events that are not explicit in-app user actions
        // and can arrive while a work round is actively running (filesystem watcher,
        // external WebSocket polling).  The achievement is still recorded in the DB;
        // only the interrupting notification is withheld.
        let suppress_toast = matches!(
            event,
            AppEvent::ThemeCreated | AppEvent::WebSocketMessage { .. }
        ) && app
            .try_state::<crate::timer::TimerController>()
            .map(|t| {
                let snap = t.get_snapshot();
                snap.round_type == "work" && snap.is_running
            })
            .unwrap_or(false);

        if !newly.is_empty() && !suppress_toast {
            notify_and_spawn_toast(newly, app, mute_chime);
        }
    }
}

// ---------------------------------------------------------------------------
// Criteria — one arm per achievement, querying the DB directly
// ---------------------------------------------------------------------------

fn criteria_met(id: &str, conn: &Connection, app: &AppHandle) -> bool {
    use crate::achievements::event;
    match id {
        // --- Original Milestone ---
        "the_seed" => queries::get_all_time_stats(conn)
            .map(|s| s.completed_work_sessions >= 1).unwrap_or(false),
        "hat_trick" => queries::get_hat_trick_max_day(conn)
            .map(|n| n >= 3).unwrap_or(false),
        "the_centurion" => queries::get_all_time_stats(conn)
            .map(|s| s.completed_work_sessions >= 100).unwrap_or(false),
        "tomato_baron" => queries::get_all_time_stats(conn)
            .map(|s| s.completed_work_sessions >= 500).unwrap_or(false),
        "tomato_tycoon" => queries::get_all_time_stats(conn)
            .map(|s| s.completed_work_sessions >= 1000).unwrap_or(false),
        "time_lord" => queries::get_all_time_stats(conn)
            .map(|s| s.total_work_secs >= 360_000).unwrap_or(false),
        // --- Original Habit ---
        "on_a_roll"    => queries::get_streak(conn).map(|s| s.longest >= 3).unwrap_or(false),
        "week_warrior" => queries::get_streak(conn).map(|s| s.longest >= 7).unwrap_or(false),
        "month_of_zen" => queries::get_streak(conn).map(|s| s.longest >= 30).unwrap_or(false),
        "comeback_kid" => queries::get_comeback_kid(conn).unwrap_or(false),
        // --- Original Discovery ---
        "early_bird"        => queries::get_early_sessions(conn).unwrap_or(false),
        "midnight_oil"      => queries::get_midnight_sessions(conn).unwrap_or(false),
        "perfect_day"       => queries::get_perfect_day_exists(conn).unwrap_or(false),
        "the_long_haul"     => queries::get_long_break_completed(conn).unwrap_or(false),
        "creature_of_habit" => queries::get_creature_of_habit(conn).unwrap_or(false),
        "theme_artist" => {
            if queries::count_events(conn, event::THEME_CREATED) > 0 { return true; }
            app.path().app_data_dir().map(|dir| {
                !crate::themes::load_custom(&dir.join("themes")).is_empty()
            }).unwrap_or(false)
        }

        // --- New Habit ---
        "showing_up"     => queries::event_current_streak(conn, event::APP_LAUNCHED) >= 7,
        "daily_devotee"  => queries::event_current_streak(conn, event::APP_LAUNCHED) >= 30,
        "the_long_game"  => queries::get_streak(conn).map(|s| s.longest >= 100).unwrap_or(false),
        "morning_ritual" => queries::get_morning_ritual_streak(conn) >= 5,
        "balanced"       => queries::get_balanced_week(conn),
        "stretch_break"  => queries::get_stretch_break(conn),
        "weekly_review"  => queries::get_stats_weekly_streak(conn) >= 4,

        // --- New Milestone ---
        "flow_state"      => queries::get_flow_state(conn) >= 4,
        "heatmap_inferno" => queries::get_heatmap_inferno_streak(conn) >= 7,
        "full_palette"    => queries::count_distinct_event_payloads(conn, event::THEME_APPLIED) >= 10,
        "front_and_center" => queries::count_events(conn, event::SESSION_ALWAYS_ON_TOP) >= 4,
        "compact_champion" => queries::count_events(conn, event::SESSION_COMPACT) >= 5,
        "power_user"       => queries::count_events(conn, event::SHORTCUT_USED) >= 50,

        // --- New Discovery ---
        "rest_is_productive"  => queries::get_rest_is_productive(conn),
        "first_impression"    => queries::count_events(conn, event::THEME_APPLIED) > 0,
        "by_the_numbers"      => queries::count_events(conn, event::STATS_OPENED) > 0,
        "your_rules" => {
            // Fires when time_work_secs key is saved; payload is the key name.
            queries::count_events_with_payload(conn, event::SETTINGS_SAVED, "time_work_secs") > 0
        }
        "sound_check"         => queries::count_events(conn, event::AUDIO_CUSTOM_LOADED) > 0,
        "lost_in_translation" => queries::count_events(conn, event::LANGUAGE_CHANGED) > 0,
        "background_worker"   => queries::count_events(conn, event::SESSION_TRAY) > 0,

        // --- New Secret ---
        "too_eager"      => queries::get_too_eager_count(conn) >= 3,
        "slow_and_steady" => queries::get_slow_and_steady_streak(conn) >= 7,
        "obsessive_saver" => queries::get_obsessive_saver(conn),
        "rebel"           => queries::count_events(conn, event::SESSION_SKIPPED_LATE) >= 3,
        "holiday_focus"   => queries::get_session_on_month_day(conn, 12, 25),
        "new_year_focus"  => queries::get_session_on_month_day(conn, 1, 1),
        "self_love"       => queries::get_session_on_month_day(conn, 2, 14),
        "lucky_streak"    => queries::get_exact_daily_work_count(conn, 7),
        "perfect_ten"     => queries::get_exact_daily_work_count(conn, 10),
        "ghost_mode"      => queries::get_ghost_mode_streak(conn),
        "the_completionist" => {
            // All non-secret achievements must be earned.
            let earned = match queries::get_earned_achievement_ids(conn) {
                Ok(s) => s,
                Err(_) => return false,
            };
            crate::achievements::ACHIEVEMENTS.iter()
                .filter(|d| !d.secret)
                .all(|d| earned.contains(d.id))
        }
        "wired_in"        => queries::count_events(conn, event::WEBSOCKET_ENABLED) > 0,
        "streaming_live"  => queries::count_events(conn, event::SESSION_WEBSOCKET_ACTIVE) >= 5,
        "automated"       => queries::count_events(conn, event::WEBSOCKET_MESSAGE) > 0,
        "deep_dive"       => queries::count_events(conn, event::STATS_LONG_VIEW) > 0,
        "history_buff"    => queries::get_event_on_dec31(conn, event::STATS_OPENED),
        "tres_bien" => queries::get_tres_bien_today(conn),
        "chromesthete" => {
            app.path().app_data_dir().map(|dir| {
                crate::themes::load_custom(&dir.join("themes")).len() >= 3
            }).unwrap_or(false)
        }
        "marathon"   => queries::get_marathon(conn),
        "baby_steps" => queries::get_baby_steps(conn),
        "no_rest" => queries::get_no_rest_criteria(conn),
        "cold_spell"   => queries::get_cold_spell(conn),
        "library_mode" => queries::count_events(conn, event::SESSION_SILENT) >= 4,

        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Progress for unearned milestone / count achievements
// ---------------------------------------------------------------------------

pub fn progress_for(id: &str, conn: &Connection) -> Option<u32> {
    use crate::achievements::event;
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
        "showing_up"    => Some(queries::event_current_streak(conn, event::APP_LAUNCHED).min(7) as u32),
        "daily_devotee" => Some(queries::event_current_streak(conn, event::APP_LAUNCHED).min(30) as u32),
        "the_long_game" => queries::get_streak(conn).ok().map(|s| s.longest.min(100)),
        "flow_state"       => Some(queries::get_flow_state(conn).min(4)),
        "full_palette"     => Some(queries::count_distinct_event_payloads(conn, event::THEME_APPLIED).min(10) as u32),
        "front_and_center" => Some(queries::count_events(conn, event::SESSION_ALWAYS_ON_TOP).min(4) as u32),
        "compact_champion" => Some(queries::count_events(conn, event::SESSION_COMPACT).min(5) as u32),
        "power_user"       => Some(queries::count_events(conn, event::SHORTCUT_USED).min(50) as u32),
        "streaming_live"   => Some(queries::count_events(conn, event::SESSION_WEBSOCKET_ACTIVE).min(5) as u32),
        "library_mode"     => Some(queries::count_events(conn, event::SESSION_SILENT).min(4) as u32),
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
/// `mute_chime` suppresses the bell sound in the toast (e.g. when a round
/// completion sound is already playing).
pub fn notify_and_spawn_toast(newly_unlocked: Vec<String>, app: &AppHandle, mute_chime: bool) {
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
        if !mute_chime {
            if let Some(audio) = app.try_state::<std::sync::Arc<crate::audio::AudioManager>>() {
                audio.play_cue(crate::audio::AudioCue::Achievement);
            }
        }
        crate::achievements::toast::spawn_toast_window(app, &newly_unlocked, count);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{migrations, queries};
    use rusqlite::{Connection, params};

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        conn
    }

    fn now_secs() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }

    /// Insert a session with explicit timestamps (bypasses unix_now()).
    fn insert_session_at(
        conn: &Connection,
        round_type: &str,
        duration_secs: u32,
        started_at: i64,
        completed: bool,
    ) {
        let ended_at = started_at + duration_secs as i64;
        conn.execute(
            "INSERT INTO sessions (started_at, ended_at, round_type, duration_secs, completed)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![started_at, ended_at, round_type, duration_secs, completed as i64],
        )
        .unwrap();
    }

    /// Insert an achievement row directly (simulates a prior unlock).
    fn unlock(conn: &Connection, id: &str) {
        queries::insert_achievement(conn, id, now_secs()).unwrap();
    }

    // -------------------------------------------------------------------------
    // cleanup_session_achievements
    // -------------------------------------------------------------------------

    #[test]
    fn cleanup_empty_db_no_panic() {
        let conn = setup();
        // Should complete without panicking or erroring on an empty DB
        cleanup_session_achievements(&conn);
        let earned = queries::get_earned_achievement_ids(&conn).unwrap();
        assert!(earned.is_empty());
    }

    #[test]
    fn cleanup_removes_session_only_achievements() {
        let conn = setup();
        // Unlock one SESSION_COMPLETED-only achievement and one non-session achievement
        unlock(&conn, "the_seed");          // triggers: [SESSION_COMPLETED]
        unlock(&conn, "first_impression"); // triggers: [THEME_APPLIED]

        cleanup_session_achievements(&conn);

        let earned = queries::get_earned_achievement_ids(&conn).unwrap();
        assert!(
            !earned.contains("the_seed"),
            "the_seed should be removed (session-only trigger)"
        );
        assert!(
            earned.contains("first_impression"),
            "first_impression should be kept (theme trigger)"
        );
    }

    #[test]
    fn cleanup_removes_multiple_session_achievements() {
        let conn = setup();
        unlock(&conn, "the_seed");
        unlock(&conn, "hat_trick");
        unlock(&conn, "on_a_roll");
        unlock(&conn, "by_the_numbers"); // triggers: [STATS_OPENED] — must survive

        cleanup_session_achievements(&conn);

        let earned = queries::get_earned_achievement_ids(&conn).unwrap();
        assert!(!earned.contains("the_seed"));
        assert!(!earned.contains("hat_trick"));
        assert!(!earned.contains("on_a_roll"));
        assert!(earned.contains("by_the_numbers"));
    }

    // -------------------------------------------------------------------------
    // progress_for
    // -------------------------------------------------------------------------

    #[test]
    fn progress_for_the_seed_zero() {
        let conn = setup();
        assert_eq!(progress_for("the_seed", &conn), Some(0));
    }

    #[test]
    fn progress_for_the_seed_five_sessions() {
        let conn = setup();
        let t = now_secs() - 10 * 3600;
        for i in 0..5_i64 {
            insert_session_at(&conn, "work", 1500, t + i * 1800, true);
        }
        assert_eq!(progress_for("the_seed", &conn), Some(5));
    }

    #[test]
    fn progress_for_time_lord_in_hours() {
        let conn = setup();
        // 7200 s completed = 2 hours
        let t = now_secs() - 10 * 3600;
        insert_session_at(&conn, "work", 7200, t, true);
        assert_eq!(progress_for("time_lord", &conn), Some(2));
    }

    #[test]
    fn progress_for_on_a_roll_two_day_streak() {
        let conn = setup();
        let today_midnight = (now_secs() / 86_400) * 86_400;
        insert_session_at(&conn, "work", 1500, today_midnight - 86_400 + 36_000, true); // yesterday
        insert_session_at(&conn, "work", 1500, today_midnight + 36_000, true);          // today
        // Longest streak = 2
        assert_eq!(progress_for("on_a_roll", &conn), Some(2));
    }

    #[test]
    fn progress_for_unknown_id_returns_none() {
        let conn = setup();
        assert_eq!(progress_for("nonexistent_achievement", &conn), None);
    }

    // -------------------------------------------------------------------------
    // the_completionist
    // -------------------------------------------------------------------------

    #[test]
    fn the_completionist_all_non_secret_true() {
        let conn = setup();
        for def in ACHIEVEMENTS.iter().filter(|d| !d.secret) {
            unlock(&conn, def.id);
        }
        let earned = queries::get_earned_achievement_ids(&conn).unwrap();
        assert!(
            ACHIEVEMENTS.iter().filter(|d| !d.secret).all(|d| earned.contains(d.id)),
            "all non-secret achievements should be in the earned set"
        );
    }

    #[test]
    fn the_completionist_missing_one_false() {
        let conn = setup();
        let non_secret: Vec<_> = ACHIEVEMENTS.iter().filter(|d| !d.secret).collect();
        // Unlock all except the last non-secret achievement.
        for def in non_secret.iter().take(non_secret.len() - 1) {
            unlock(&conn, def.id);
        }
        let earned = queries::get_earned_achievement_ids(&conn).unwrap();
        assert!(
            !ACHIEVEMENTS.iter().filter(|d| !d.secret).all(|d| earned.contains(d.id)),
            "should not be complete when one non-secret achievement is missing"
        );
    }
}
