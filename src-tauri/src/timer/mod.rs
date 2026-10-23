pub mod engine;
pub mod sequence;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use crate::audio::{AudioCue, AudioManager};
use crate::db::{queries, DbState};
use crate::settings::{self, Settings};
use crate::tray::{self, TrayState};
use crate::websocket::{self, WsState};

use engine::{EngineHandle, TimerCommand, TimerEvent};
use sequence::{DayRollover, RoundType, SequenceState};

// ---------------------------------------------------------------------------
// Persisted incremental-focus ladder
// ---------------------------------------------------------------------------
//
// The ladder lives in `SequenceState`, but unlike the round counters it is
// expected to survive a restart: with the long-break/day reset triggers turned
// off it is supposed to keep climbing until the user resets it. It is stored in
// the settings key/value table (which already holds similar runtime state such
// as the last window position) under its own two keys.

/// Settings key holding the number of increments currently applied.
const KEY_LADDER_STEPS: &str = "ladder_steps";
/// Settings key holding the local date the ladder was last touched on.
const KEY_LADDER_DAY: &str = "ladder_day";

/// Today's local calendar date as `"YYYY-MM-DD"`, or `None` if the database
/// cannot answer. Read through SQLite so the ladder's notion of "today" matches
/// the statistics queries exactly.
fn today_local(db: &DbState) -> Option<String> {
    let conn = db.lock().ok()?;
    conn.query_row("SELECT date('now', 'localtime')", [], |row| row.get::<_, String>(0))
        .map_err(|e| log::warn!("[timer] could not read the local date: {e}"))
        .ok()
}

/// Copy the ladder's position out of the sequence state.
fn ladder_position(sequence: &Arc<Mutex<SequenceState>>) -> (u32, Option<String>) {
    let seq = sequence.lock().unwrap();
    (seq.work_rounds_completed, seq.ladder_day.clone())
}

/// Persist the ladder so it survives an app restart.
fn persist_ladder(db: &DbState, steps: u32, day: Option<&str>) {
    if let Ok(conn) = db.lock() {
        let _ = settings::save_setting(&conn, KEY_LADDER_STEPS, &steps.to_string());
        let _ = settings::save_setting(&conn, KEY_LADDER_DAY, day.unwrap_or_default());
    }
}

/// Restore a previously persisted ladder into `seq`. Must run before the first
/// duration is computed so a restart resumes at the right step.
fn restore_ladder(db: &DbState, seq: &mut SequenceState) {
    let Ok(conn) = db.lock() else { return };
    if let Some(steps) = settings::get_setting(&conn, KEY_LADDER_STEPS)
        .and_then(|v| v.parse::<u32>().ok())
    {
        seq.work_rounds_completed = steps;
    }
    if let Some(day) = settings::get_setting(&conn, KEY_LADDER_DAY).filter(|d| !d.is_empty()) {
        seq.ladder_day = Some(day);
    }
}

// ---------------------------------------------------------------------------
// Snapshot — serialized to JSON for the frontend
// ---------------------------------------------------------------------------

/// Full timer state snapshot. Sent as the payload of Tauri events and
/// returned by the `timer_get_state` IPC command.
#[derive(Debug, Clone, Serialize)]
pub struct TimerSnapshot {
    /// "work" | "short-break" | "long-break"
    pub round_type: String,
    /// Round type that was active before this one. Empty string on the first round of a session.
    pub previous_round_type: String,
    pub elapsed_secs: u32,
    pub total_secs: u32,
    pub is_running: bool,
    /// True if the timer has been started and then paused (elapsed > 0, not running).
    pub is_paused: bool,
    pub work_round_number: u32,
    pub work_rounds_total: u32,
    /// Monotonically-increasing focus round count since last reset. Used as a
    /// session counter when long breaks are disabled.
    pub session_work_count: u32,
    /// Incremental focus mode: whether the escalating ladder is active.
    pub incremental_work_enabled: bool,
    /// The base (un-escalated) work duration from settings, in seconds.
    pub base_work_secs: u32,
    /// Seconds added to the work duration per completed work round.
    pub work_increment_secs: u32,
    /// Escalation ceiling for the work duration in seconds.
    pub work_max_secs: u32,
    /// How many increments are currently applied to the work duration.
    pub increment_steps: u32,
    /// True when the work duration has reached the configured ceiling.
    pub at_increment_cap: bool,
}

// ---------------------------------------------------------------------------
// Shared mutable state between the controller and the event-listener thread
// ---------------------------------------------------------------------------

struct TimerShared {
    elapsed_secs: u32,
    is_running: bool,
}

// ---------------------------------------------------------------------------
// TimerController — public API registered as Tauri state
// ---------------------------------------------------------------------------

pub struct TimerController {
    engine: EngineHandle,
    sequence: Arc<Mutex<SequenceState>>,
    settings: Arc<Mutex<Settings>>,
    shared: Arc<Mutex<TimerShared>>,
    /// Used to persist the incremental ladder and to resolve the local date.
    db: DbState,
    /// Kept alive so TrayState is not dropped if lib.rs forgets its copy.
    #[allow(dead_code)]
    tray: Arc<TrayState>,
}

impl TimerController {
    /// Construct and start the background threads.
    /// Call once from `lib.rs` during Tauri `setup`.
    pub fn new(
        app: AppHandle,
        settings: Settings,
        tray: Arc<TrayState>,
        db: DbState,
    ) -> Self {
        let mut seq = SequenceState::new(settings.long_break_interval);
        // Resume a ladder that was left climbing by a previous run.
        restore_ladder(&db, &mut seq);
        let duration = seq.current_duration_secs(&settings);

        let (engine, event_rx) = engine::spawn(duration, Duration::from_secs(1));

        let sequence = Arc::new(Mutex::new(seq));
        let settings_arc = Arc::new(Mutex::new(settings));
        let shared = Arc::new(Mutex::new(TimerShared {
            elapsed_secs: 0,
            is_running: false,
        }));

        // Clone handles for the event-listener thread.
        let seq_thread = Arc::clone(&sequence);
        let settings_thread = Arc::clone(&settings_arc);
        let shared_thread = Arc::clone(&shared);
        let engine_thread = engine.clone();
        let tray_thread = Arc::clone(&tray);
        let db_thread = Arc::clone(&db);

        std::thread::Builder::new()
            .name("timer-events".to_string())
            .spawn(move || {
                listen_events(
                    app,
                    event_rx,
                    ListenContext {
                        sequence: seq_thread,
                        settings: settings_thread,
                        shared: shared_thread,
                        engine: engine_thread,
                        tray: tray_thread,
                        db: db_thread,
                    },
                );
            })
            .expect("failed to spawn timer event listener");

        Self {
            engine,
            sequence,
            settings: settings_arc,
            shared,
            db,
            tray,
        }
    }

    // --- Commands ---

    /// True when the timer sits at the start of a round: nothing running and
    /// nothing elapsed.
    fn is_idle(&self) -> bool {
        let s = self.shared.lock().unwrap();
        !s.is_running && s.elapsed_secs == 0
    }

    /// Persist the ladder's current position.
    fn save_ladder(&self) {
        let (steps, day) = ladder_position(&self.sequence);
        persist_ladder(&self.db, steps, day.as_deref());
    }

    /// Restart the ladder when the local calendar day has changed.
    ///
    /// Only fires while the timer is idle: a round that is already running or
    /// paused keeps the duration it was primed with, and the rollover is picked
    /// up at the next round boundary instead (see the `Complete` handler).
    /// Returns true when the ladder actually restarted.
    fn roll_ladder_day(&self) -> bool {
        if !self.is_idle() {
            return false;
        }
        let Some(today) = today_local(&self.db) else {
            return false;
        };
        let rollover = {
            let mut seq = self.sequence.lock().unwrap();
            let s = self.settings.lock().unwrap();
            seq.check_day_rollover(&s, &today)
        };
        if !rollover.day_changed {
            return false;
        }
        // The day moved on even when the reset itself is disabled.
        self.save_ladder();
        if rollover.reset {
            log::info!("[timer] incremental focus ladder restarted for a new day");
            self.reconfigure();
        }
        rollover.reset
    }

    /// Toggle: start a fresh timer if idle, resume if paused, pause if running.
    pub fn toggle(&self) {
        self.roll_ladder_day();
        let s = self.shared.lock().unwrap();
        if s.is_running {
            log::info!("[timer] pause");
            self.engine.send(TimerCommand::Pause);
        } else if s.elapsed_secs > 0 {
            log::info!("[timer] resume");
            self.engine.send(TimerCommand::Resume);
        } else {
            log::info!("[timer] start");
            self.engine.send(TimerCommand::Start);
        }
    }

    pub fn reset(&self) {
        log::info!("[timer] reset");
        self.sequence.lock().unwrap().reset();
        self.save_ladder();
        // Send only Reset — the event listener's Reset handler will follow up
        // with Prime once the engine is confirmed Idle. Sending a duration
        // update here first would race the Reset and can leave the UI stale.
        self.engine.send(TimerCommand::Reset);
    }

    /// Restart the incremental-focus ladder from the base duration.
    ///
    /// Manual reset trigger: the round, cycle and session counters are left
    /// alone, and a running countdown is not interrupted — only the rounds that
    /// follow are shortened back to the base focus duration.
    pub fn reset_increment_ladder(&self) {
        let today = today_local(&self.db);
        let changed = self
            .sequence
            .lock()
            .unwrap()
            .reset_ladder(today.as_deref());
        log::info!("[timer] incremental focus ladder reset (cleared {changed})");
        self.save_ladder();
        if self.is_idle() {
            self.reconfigure();
        }
    }

    /// Restart only the current round's timer without touching the sequence.
    /// Round type, round number, and position in the work/break cycle are all
    /// preserved — only the elapsed time is zeroed.
    pub fn restart_round(&self) {
        log::info!("[timer] restart round");
        self.engine.send(TimerCommand::Reset);
    }

    pub fn skip(&self) {
        log::info!("[timer] skip");
        self.engine.send(TimerCommand::Skip);
    }

    pub fn suspend(&self) {
        self.engine.send(TimerCommand::Suspend);
    }

    pub fn wake_resume(&self) {
        self.engine.send(TimerCommand::WakeResume);
    }

    /// Update the duration for the current round when settings change.
    /// Only takes effect after the next Start/Resume (current countdown is not interrupted).
    pub fn reconfigure(&self) {
        let duration = {
            let seq = self.sequence.lock().unwrap();
            let settings = self.settings.lock().unwrap();
            seq.current_duration_secs(&settings)
        };
        self.engine.send(TimerCommand::Reconfigure { duration_secs: duration });
    }

    // --- Query ---

    pub fn get_snapshot(&self) -> TimerSnapshot {
        // A snapshot is the main way the ladder becomes visible, so it is also
        // where a day rollover is noticed while the app sits idle.
        self.roll_ladder_day();

        let seq = self.sequence.lock().unwrap();
        let settings = self.settings.lock().unwrap();
        let shared = self.shared.lock().unwrap();

        TimerSnapshot {
            round_type: seq.current_round.as_str().to_string(),
            previous_round_type: seq.previous_round.map(|r| r.as_str().to_string()).unwrap_or_default(),
            elapsed_secs: shared.elapsed_secs,
            total_secs: seq.current_duration_secs(&settings),
            is_running: shared.is_running,
            is_paused: !shared.is_running && shared.elapsed_secs > 0,
            work_round_number: seq.work_round_number,
            work_rounds_total: seq.work_rounds_total,
            session_work_count: seq.session_work_count,
            incremental_work_enabled: settings.incremental_work_enabled,
            base_work_secs: settings.time_work_secs,
            work_increment_secs: settings.time_work_increment_secs,
            work_max_secs: settings.time_work_max_secs,
            increment_steps: seq.work_increment_steps(&settings),
            at_increment_cap: seq.work_duration_at_cap(&settings),
        }
    }

    /// Apply new settings values. Updates the in-memory copy and, if the
    /// timer is idle (not running and no elapsed progress), reconfigures the
    /// engine so the next Start uses the new duration.
    ///
    /// When the timer is running or paused, the current countdown is left
    /// untouched; the new duration takes effect at the start of the next
    /// round or after a manual reset.  Sending Reconfigure to a running
    /// engine transitions it to Idle, which would freeze the timer.
    pub fn apply_settings(&self, new: Settings) {
        // Sync work_rounds_total so the round counter and advance() logic both
        // reflect the new long_break_interval immediately.
        self.sequence.lock().unwrap().work_rounds_total = new.long_break_interval;
        *self.settings.lock().unwrap() = new;
        // Picks up a day rollover that happened while nothing was running, so
        // the reconfigure below primes the corrected duration.
        self.roll_ladder_day();
        let s = self.shared.lock().unwrap();
        let is_idle = !s.is_running && s.elapsed_secs == 0;
        drop(s);
        if is_idle {
            self.reconfigure();
        }
    }
}

// ---------------------------------------------------------------------------
// Background event listener thread
// ---------------------------------------------------------------------------

struct ListenContext {
    sequence: Arc<Mutex<SequenceState>>,
    settings: Arc<Mutex<Settings>>,
    shared: Arc<Mutex<TimerShared>>,
    engine: EngineHandle,
    tray: Arc<TrayState>,
    db: DbState,
}

fn listen_events(
    app: AppHandle,
    event_rx: std::sync::mpsc::Receiver<TimerEvent>,
    ctx: ListenContext,
) {
    let ListenContext { sequence, settings, shared, engine, tray, db } = ctx;
    // Track last tray progress to throttle redraws to ≥ 1% delta.
    let mut last_tray_progress: f32 = -1.0;
    // Active session row ID for recording (None = not started yet).
    let mut current_session_id: Option<i64> = None;

    while let Ok(event) = event_rx.recv() {
        match event {
            TimerEvent::Started { total_secs } => {
                log::info!("[timer] started total={total_secs}s");
                shared.lock().unwrap().is_running = true;
                let _ = app.emit("timer:started", serde_json::json!({ "total_secs": total_secs }));
                if let Some(ws) = app.try_state::<Arc<WsState>>() {
                    websocket::broadcast_started(&ws, total_secs);
                }
                tray::update_menu_items(&tray, true, false);
            }

            TimerEvent::Tick { elapsed_secs, total_secs } => {
                {
                    let mut s = shared.lock().unwrap();
                    s.elapsed_secs = elapsed_secs;
                    s.is_running = true;
                }
                let _ = app.emit(
                    "timer:tick",
                    serde_json::json!({ "elapsed_secs": elapsed_secs, "total_secs": total_secs }),
                );

                // --- Session recording: start on first tick of a new round ---
                if elapsed_secs == 1 && current_session_id.is_none() {
                    let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                    let total = {
                        let seq = sequence.lock().unwrap();
                        let s = settings.lock().unwrap();
                        seq.current_duration_secs(&s)
                    };
                    if let Ok(conn) = db.lock() {
                        match queries::insert_session(&conn, &rt, total) {
                            Ok(id) => current_session_id = Some(id),
                            Err(e) => log::error!("[timer] failed to record session: {e}"),
                        }
                    }
                }

                // --- Tick sound ---
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                if let Some(audio) = app.try_state::<Arc<AudioManager>>() {
                    if audio.tick_enabled_for(&rt) {
                        audio.play_cue(AudioCue::Tick);
                    }
                }

                // Update tray arc — throttle to 1% visual change.
                let progress = if total_secs > 0 {
                    elapsed_secs as f32 / total_secs as f32
                } else {
                    0.0
                };
                if (progress - last_tray_progress).abs() >= 0.01 {
                    tray::update_icon(&tray, &rt, false, progress);
                    last_tray_progress = progress;
                }
            }

            TimerEvent::Complete { skipped: was_skipped } => {
                let completed_round = sequence.lock().unwrap().current_round.as_str().to_string();
                log::info!(
                    "[timer] round complete type={completed_round} skipped={was_skipped}"
                );

                // --- Session recording: mark the completed round ---
                if let Some(session_id) = current_session_id.take() {
                    if let Ok(conn) = db.lock() {
                        let _ = queries::complete_session(&conn, session_id, !was_skipped);
                    }
                }

                // Advance sequence. A new calendar day restarts the ladder
                // before the next round's duration is computed.
                let today = today_local(&db);
                let completed_work_round = completed_round == "work";
                let (next_round, next_duration, auto_start_work, auto_start_break) = {
                    let mut seq = sequence.lock().unwrap();
                    let s = settings.lock().unwrap();
                    let rollover = match today.as_deref() {
                        Some(today) => seq.check_day_rollover(&s, today),
                        None => DayRollover::default(),
                    };
                    let (rt, mut dur) = seq.advance(&s);
                    // A work round that finished just after midnight belongs to
                    // the day it started in, so it must not extend the new day's
                    // ladder: drop its step and re-derive the following duration.
                    if rollover.reset && completed_work_round {
                        seq.reset_ladder(today.as_deref());
                        dur = seq.current_duration_secs(&s);
                    }
                    (rt, dur, s.auto_start_work, s.auto_start_break)
                };

                // The ladder moved (or restarted) — keep the persisted copy in
                // step. Both locks are released before the database is touched.
                {
                    let (steps, day) = ladder_position(&sequence);
                    persist_ladder(&db, steps, day.as_deref());
                }

                // Reset shared state for the new round.
                {
                    let mut s = shared.lock().unwrap();
                    s.elapsed_secs = 0;
                    s.is_running = false;
                }

                // Arm the next round's duration without risking a late
                // reconfigure that kicks a freshly-started timer back to Idle.
                engine.send(TimerCommand::Prime {
                    duration_secs: next_duration,
                });

                // Emit round-change with the new snapshot.
                let snapshot = build_snapshot(&sequence, &settings, &shared);
                let _ = app.emit("timer:round-change", snapshot);

                // Desktop notifications are dispatched by the frontend via the
                // notification_show command after receiving the timer:round-change
                // event, so translated strings can be used.

                // Audio alert for the new round.
                if let Some(audio) = app.try_state::<Arc<AudioManager>>() {
                    let cue = match next_round {
                        RoundType::Work => AudioCue::WorkAlert,
                        RoundType::ShortBreak => AudioCue::ShortBreakAlert,
                        RoundType::LongBreak => AudioCue::LongBreakAlert,
                    };
                    audio.play_cue(cue);
                }

                // Lower-priority-during-breaks: when always_on_top is on and
                // break_always_on_top is enabled, disable always-on-top for
                // breaks and restore it when work resumes.
                let (always_on_top, break_always_on_top) = {
                    let s = settings.lock().unwrap();
                    (s.always_on_top, s.break_always_on_top)
                };
                if always_on_top {
                    if let Some(window) = app.get_webview_window("main") {
                        let is_break = next_round != RoundType::Work;
                        let _ = window.set_always_on_top(!(break_always_on_top && is_break));
                    }
                }

                // Update tray to reflect new round type and reset progress.
                // Use -1.0 (same as initialisation and Reset) so the very
                // first tick of the new round always passes the ≥1% threshold,
                // regardless of how long the round is.  Using 0.0 here caused
                // a ≥15-second blank period before the arc started animating.
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                tray::update_icon(&tray, &rt, false, 0.0);
                last_tray_progress = -1.0;

                // Broadcast round-change to any connected WebSocket clients.
                if let Some(ws) = app.try_state::<Arc<WsState>>() {
                    let snap = build_snapshot(&sequence, &settings, &shared);
                    websocket::broadcast_round_change(&ws, snap);
                }

                // Auto-start if configured.
                let should_auto = match next_round {
                    RoundType::Work => auto_start_work,
                    _ => auto_start_break,
                };
                if should_auto {
                    log::debug!("[timer] auto-starting {}", next_round.as_str());
                    engine.send(TimerCommand::Start);
                } else {
                    // Timer is idle waiting for the user to start the new round.
                    // Reset the tray menu to "Start" so it doesn't keep showing
                    // "Pause" from the round that just completed.
                    tray::update_menu_items(&tray, false, false);
                }
            }

            TimerEvent::Paused { elapsed_secs } => {
                log::info!("[timer] paused elapsed={elapsed_secs}s");
                shared.lock().unwrap().is_running = false;
                let _ = app.emit("timer:paused", serde_json::json!({ "elapsed_secs": elapsed_secs }));
                if let Some(ws) = app.try_state::<Arc<WsState>>() {
                    websocket::broadcast_paused(&ws, elapsed_secs);
                }

                // Show pause bars in tray.
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                let total = {
                    let seq = sequence.lock().unwrap();
                    let s = settings.lock().unwrap();
                    seq.current_duration_secs(&s)
                };
                let progress = if total > 0 { elapsed_secs as f32 / total as f32 } else { 0.0 };
                tray::update_icon(&tray, &rt, true, progress);
                tray::update_menu_items(&tray, false, true);
            }

            TimerEvent::Resumed { elapsed_secs } => {
                log::info!("[timer] resumed elapsed={elapsed_secs}s");
                shared.lock().unwrap().is_running = true;
                let _ = app.emit("timer:resumed", serde_json::json!({ "elapsed_secs": elapsed_secs }));
                if let Some(ws) = app.try_state::<Arc<WsState>>() {
                    websocket::broadcast_resumed(&ws, elapsed_secs);
                }

                // Restore arc in tray.
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                let total = {
                    let seq = sequence.lock().unwrap();
                    let s = settings.lock().unwrap();
                    seq.current_duration_secs(&s)
                };
                let progress = if total > 0 { elapsed_secs as f32 / total as f32 } else { 0.0 };
                tray::update_icon(&tray, &rt, false, progress);
                last_tray_progress = progress;
                tray::update_menu_items(&tray, true, false);
            }

            TimerEvent::Reset => {
                log::debug!("[timer] idle");
                // Abandon the active session (leave DB row as-is).
                current_session_id = None;

                {
                    let mut s = shared.lock().unwrap();
                    s.elapsed_secs = 0;
                    s.is_running = false;
                }
                let snapshot = build_snapshot(&sequence, &settings, &shared);
                let _ = app.emit("timer:reset", snapshot);
                if let Some(ws) = app.try_state::<Arc<WsState>>() {
                    websocket::broadcast_reset(&ws);
                }

                // Prime the engine with the current round's duration so the
                // next Start uses the correct (possibly settings-updated)
                // total. Using the lighter-weight command here avoids a race
                // where a fast user click on Start is immediately clobbered by
                // a late follow-up duration update.
                let duration = {
                    let seq = sequence.lock().unwrap();
                    let s = settings.lock().unwrap();
                    seq.current_duration_secs(&s)
                };
                engine.send(TimerCommand::Prime { duration_secs: duration });

                // Reset tray to idle (empty arc).
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                tray::update_icon(&tray, &rt, false, 0.0);
                last_tray_progress = -1.0;
                tray::update_menu_items(&tray, false, false);
            }

            TimerEvent::Suspended { elapsed_secs } => {
                log::info!("[timer] suspended by system elapsed={elapsed_secs}s");
                shared.lock().unwrap().is_running = false;
                let _ = app.emit(
                    "timer:suspended",
                    serde_json::json!({ "elapsed_secs": elapsed_secs }),
                );

                // Show pause bars while suspended.
                let rt = sequence.lock().unwrap().current_round.as_str().to_string();
                let total = {
                    let seq = sequence.lock().unwrap();
                    let s = settings.lock().unwrap();
                    seq.current_duration_secs(&s)
                };
                let progress = if total > 0 { elapsed_secs as f32 / total as f32 } else { 0.0 };
                tray::update_icon(&tray, &rt, true, progress);
            }
        }
    }
}

fn build_snapshot(
    sequence: &Arc<Mutex<SequenceState>>,
    settings: &Arc<Mutex<Settings>>,
    shared: &Arc<Mutex<TimerShared>>,
) -> TimerSnapshot {
    let seq = sequence.lock().unwrap();
    let s = settings.lock().unwrap();
    let sh = shared.lock().unwrap();

    TimerSnapshot {
        round_type: seq.current_round.as_str().to_string(),
        previous_round_type: seq.previous_round.map(|r| r.as_str().to_string()).unwrap_or_default(),
        elapsed_secs: sh.elapsed_secs,
        total_secs: seq.current_duration_secs(&s),
        is_running: sh.is_running,
        is_paused: !sh.is_running && sh.elapsed_secs > 0,
        work_round_number: seq.work_round_number,
        work_rounds_total: seq.work_rounds_total,
        session_work_count: seq.session_work_count,
        incremental_work_enabled: s.incremental_work_enabled,
        base_work_secs: s.time_work_secs,
        work_increment_secs: s.time_work_increment_secs,
        work_max_secs: s.time_work_max_secs,
        increment_steps: seq.work_increment_steps(&s),
        at_increment_cap: seq.work_duration_at_cap(&s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;

    fn test_db() -> DbState {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn persisted_ladder_survives_a_restart() {
        let db = test_db();
        persist_ladder(&db, 3, Some("2026-05-01"));

        // A restart builds a brand-new sequence; the ladder comes back.
        let mut seq = SequenceState::new(4);
        restore_ladder(&db, &mut seq);

        assert_eq!(seq.work_rounds_completed, 3);
        assert_eq!(seq.ladder_day.as_deref(), Some("2026-05-01"));
    }

    #[test]
    fn missing_ladder_state_restores_to_zero() {
        let db = test_db();
        let mut seq = SequenceState::new(4);
        restore_ladder(&db, &mut seq);

        assert_eq!(seq.work_rounds_completed, 0);
        assert_eq!(seq.ladder_day, None);
    }

    #[test]
    fn ladder_reset_persists_an_empty_day() {
        let db = test_db();
        persist_ladder(&db, 5, Some("2026-05-01"));
        // A manual reset clears the day, which round-trips as "unknown".
        persist_ladder(&db, 0, None);

        let mut seq = SequenceState::new(4);
        restore_ladder(&db, &mut seq);

        assert_eq!(seq.work_rounds_completed, 0);
        assert_eq!(seq.ladder_day, None, "an empty day must not become Some(\"\")");
    }

    #[test]
    fn today_local_returns_an_iso_date() {
        let db = test_db();
        let today = today_local(&db).expect("in-memory SQLite can report the date");

        assert_eq!(today.len(), 10, "expected YYYY-MM-DD, got {today}");
        assert_eq!(&today[4..5], "-");
        assert_eq!(&today[7..8], "-");
        assert!(today.chars().filter(|c| c.is_ascii_digit()).count() == 8);
    }
}
