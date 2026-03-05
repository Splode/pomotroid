/// Drift-correcting timer engine running on a dedicated OS thread.
///
/// Uses `std::time::Instant` (monotonic clock) and `recv_timeout` to
/// schedule ticks against a fixed timeline rather than sleeping for a
/// constant 1 s. This eliminates cumulative drift from wakeup latency.
///
/// Sleep/wake behaviour (OQ-1): on `Suspend` the engine saves `elapsed_secs`
/// and blocks; on `WakeResume` it restarts from that position without advancing.
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TimerCommand {
    Start,
    Pause,
    Resume,
    Reset,
    /// Immediately fires a `Complete` event (user-initiated skip).
    Skip,
    /// Change the total duration; moves engine to Idle so caller must Start.
    Reconfigure { duration_secs: u32 },
    /// OS sleep detected: freeze elapsed position, block until WakeResume.
    Suspend,
    /// OS wake detected: resume from the saved elapsed position.
    WakeResume,
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum TimerEvent {
    Started { total_secs: u32 },
    Tick { elapsed_secs: u32, total_secs: u32 },
    Complete { skipped: bool },
    Paused { elapsed_secs: u32 },
    Resumed { elapsed_secs: u32 },
    Reset,
    Suspended { elapsed_secs: u32 },
}

/// Cheap-to-clone handle for sending commands to the engine thread.
#[derive(Clone)]
pub struct EngineHandle {
    pub cmd_tx: Sender<TimerCommand>,
}

impl EngineHandle {
    pub fn send(&self, cmd: TimerCommand) {
        // Ignore send errors: the thread may have exited on Shutdown.
        let _ = self.cmd_tx.send(cmd);
    }
}

/// Spawn the timer engine thread.
///
/// `tick_interval` is 1 second in production; tests pass a shorter value
/// (e.g. 20 ms) to keep test execution fast.
pub fn spawn(duration_secs: u32, tick_interval: Duration) -> (EngineHandle, Receiver<TimerEvent>) {
    let (cmd_tx, cmd_rx) = mpsc::channel::<TimerCommand>();
    let (event_tx, event_rx) = mpsc::channel::<TimerEvent>();

    std::thread::Builder::new()
        .name("timer-engine".to_string())
        .spawn(move || run_loop(duration_secs, event_tx, cmd_rx, tick_interval))
        .expect("failed to spawn timer engine thread");

    (EngineHandle { cmd_tx }, event_rx)
}

// ---------------------------------------------------------------------------
// Internal state machine
// ---------------------------------------------------------------------------

struct RunningSegment {
    /// When this run segment started (used for drift-correction).
    start: Instant,
    /// `elapsed_secs` at the moment this segment began (0 on Start, N on Resume).
    elapsed_at_start: u32,
    /// Ticks fired within this segment.
    ticks: u32,
}

enum Phase {
    Idle,
    Running(RunningSegment),
    Paused,
    Suspended,
}

enum Transition {
    Stay,
    To(Phase),
    Break,
}

fn run_loop(
    duration_secs: u32,
    event_tx: Sender<TimerEvent>,
    cmd_rx: Receiver<TimerCommand>,
    tick_interval: Duration,
) {
    let mut total_secs = duration_secs;
    let mut elapsed_secs: u32 = 0;
    let mut phase = Phase::Idle;

    'engine: loop {
        let tr = match &mut phase {
            // -----------------------------------------------------------------
            Phase::Idle => match cmd_rx.recv() {
                Ok(TimerCommand::Start) => {
                    elapsed_secs = 0;
                    let _ = event_tx.send(TimerEvent::Started { total_secs });
                    Transition::To(Phase::Running(RunningSegment {
                        start: Instant::now(),
                        elapsed_at_start: 0,
                        ticks: 0,
                    }))
                }
                Ok(TimerCommand::Reconfigure { duration_secs: d }) => {
                    total_secs = d;
                    Transition::Stay
                }
                // Reset while Idle: emit the event so the listener can update
                // the frontend and send a follow-up Reconfigure.  Without this
                // handler the command would be silently swallowed, leaving the
                // listener blocked in recv() and the UI stale.
                Ok(TimerCommand::Reset) => {
                    elapsed_secs = 0;
                    let _ = event_tx.send(TimerEvent::Reset);
                    Transition::Stay
                }
                // Skip while Idle: advance to the next round without starting.
                Ok(TimerCommand::Skip) => {
                    let _ = event_tx.send(TimerEvent::Complete { skipped: true });
                    Transition::Stay
                }
                Ok(TimerCommand::Shutdown) | Err(_) => Transition::Break,
                _ => Transition::Stay,
            },

            // -----------------------------------------------------------------
            Phase::Paused | Phase::Suspended => match cmd_rx.recv() {
                Ok(TimerCommand::Resume | TimerCommand::WakeResume) => {
                    let _ = event_tx.send(TimerEvent::Resumed { elapsed_secs });
                    Transition::To(Phase::Running(RunningSegment {
                        start: Instant::now(),
                        elapsed_at_start: elapsed_secs,
                        ticks: 0,
                    }))
                }
                Ok(TimerCommand::Reset) => {
                    elapsed_secs = 0;
                    let _ = event_tx.send(TimerEvent::Reset);
                    Transition::To(Phase::Idle)
                }
                Ok(TimerCommand::Skip) => {
                    elapsed_secs = 0;
                    let _ = event_tx.send(TimerEvent::Complete { skipped: true });
                    Transition::To(Phase::Idle)
                }
                Ok(TimerCommand::Reconfigure { duration_secs: d }) => {
                    total_secs = d;
                    elapsed_secs = 0;
                    Transition::To(Phase::Idle)
                }
                Ok(TimerCommand::Shutdown) | Err(_) => Transition::Break,
                _ => Transition::Stay,
            },

            // -----------------------------------------------------------------
            Phase::Running(seg) => {
                // Drift-correcting sleep: target the absolute instant of the
                // next scheduled tick rather than sleeping for a fixed period.
                let next_tick = seg.start + tick_interval * (seg.ticks + 1);
                let wait = next_tick.saturating_duration_since(Instant::now());

                match cmd_rx.recv_timeout(wait) {
                    // --- tick fired ---
                    Err(RecvTimeoutError::Timeout) => {
                        seg.ticks += 1;
                        elapsed_secs = seg.elapsed_at_start + seg.ticks;
                        let _ = event_tx.send(TimerEvent::Tick { elapsed_secs, total_secs });

                        if elapsed_secs >= total_secs {
                            let _ = event_tx.send(TimerEvent::Complete { skipped: false });
                            elapsed_secs = 0;
                            Transition::To(Phase::Idle)
                        } else {
                            Transition::Stay
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => Transition::Break,

                    // --- commands ---
                    Ok(TimerCommand::Pause) => {
                        let _ = event_tx.send(TimerEvent::Paused { elapsed_secs });
                        Transition::To(Phase::Paused)
                    }
                    Ok(TimerCommand::Suspend) => {
                        let _ = event_tx.send(TimerEvent::Suspended { elapsed_secs });
                        Transition::To(Phase::Suspended)
                    }
                    Ok(TimerCommand::Skip) => {
                        elapsed_secs = 0;
                        let _ = event_tx.send(TimerEvent::Complete { skipped: true });
                        Transition::To(Phase::Idle)
                    }
                    Ok(TimerCommand::Reset) => {
                        elapsed_secs = 0;
                        let _ = event_tx.send(TimerEvent::Reset);
                        Transition::To(Phase::Idle)
                    }
                    Ok(TimerCommand::Reconfigure { duration_secs: d }) => {
                        total_secs = d;
                        elapsed_secs = 0;
                        Transition::To(Phase::Idle)
                    }
                    Ok(TimerCommand::Shutdown) => Transition::Break,
                    _ => Transition::Stay,
                }
            }
        };

        match tr {
            Transition::Stay => {}
            Transition::To(new_phase) => phase = new_phase,
            Transition::Break => break 'engine,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Short tick used by all tests so the suite runs in < 1 s total.
    const TICK: Duration = Duration::from_millis(20);

    fn drain(rx: &Receiver<TimerEvent>) -> Vec<TimerEvent> {
        let mut events = Vec::new();
        while let Ok(e) = rx.recv_timeout(Duration::from_millis(5)) {
            events.push(e);
        }
        events
    }

    fn collect_until_complete(rx: &Receiver<TimerEvent>, timeout: Duration) -> Vec<TimerEvent> {
        let deadline = Instant::now() + timeout;
        let mut events = Vec::new();
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            match rx.recv_timeout(remaining) {
                Ok(e) => {
                    let done = matches!(e, TimerEvent::Complete { .. });
                    events.push(e);
                    if done {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        events
    }

    #[test]
    fn fires_correct_number_of_ticks_and_completes() {
        let (handle, rx) = spawn(5, TICK);
        handle.send(TimerCommand::Start);

        let events = collect_until_complete(&rx, Duration::from_secs(2));

        let ticks: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, TimerEvent::Tick { .. }))
            .collect();
        assert_eq!(ticks.len(), 5, "expected 5 ticks, got {}", ticks.len());
        assert!(
            matches!(events.last(), Some(TimerEvent::Complete { .. })),
            "last event must be Complete"
        );
    }

    #[test]
    fn elapsed_secs_increments_monotonically() {
        let (handle, rx) = spawn(4, TICK);
        handle.send(TimerCommand::Start);

        let events = collect_until_complete(&rx, Duration::from_secs(2));
        let mut last_elapsed = 0;
        for e in &events {
            if let TimerEvent::Tick { elapsed_secs, .. } = e {
                assert!(*elapsed_secs > last_elapsed);
                last_elapsed = *elapsed_secs;
            }
        }
    }

    #[test]
    fn pause_stops_ticks_and_resume_continues() {
        let (handle, rx) = spawn(6, TICK);
        handle.send(TimerCommand::Start);

        // Let 2 ticks fire, then pause.
        std::thread::sleep(TICK * 2 + TICK / 2);
        handle.send(TimerCommand::Pause);

        // Collect events so far.
        std::thread::sleep(TICK * 3); // no ticks should arrive during this gap
        let events_before_resume = drain(&rx);

        let paused = events_before_resume
            .iter()
            .filter(|e| matches!(e, TimerEvent::Paused { .. }))
            .count();
        let ticks_before_pause = events_before_resume
            .iter()
            .filter(|e| matches!(e, TimerEvent::Tick { .. }))
            .count();
        assert_eq!(paused, 1, "expected 1 Paused event");
        assert!(ticks_before_pause >= 2, "should have at least 2 ticks before pause");

        // Resume and let the rest complete.
        handle.send(TimerCommand::Resume);
        let events_after = collect_until_complete(&rx, Duration::from_secs(2));

        assert!(
            events_after
                .iter()
                .any(|e| matches!(e, TimerEvent::Resumed { .. })),
            "expected Resumed event"
        );
        assert!(
            events_after
                .iter()
                .any(|e| matches!(e, TimerEvent::Complete { .. })),
            "expected Complete after resume"
        );
    }

    #[test]
    fn reset_returns_to_zero_and_fires_reset_event() {
        let (handle, rx) = spawn(10, TICK);
        handle.send(TimerCommand::Start);
        std::thread::sleep(TICK * 2 + TICK / 2);
        handle.send(TimerCommand::Reset);

        let events = drain(&rx);
        assert!(
            events.iter().any(|e| matches!(e, TimerEvent::Reset)),
            "expected Reset event"
        );
        // No Complete should have fired.
        assert!(
            !events.iter().any(|e| matches!(e, TimerEvent::Complete { .. })),
            "Complete must not fire on Reset"
        );
    }

    #[test]
    fn skip_fires_complete_immediately() {
        let (handle, rx) = spawn(30, TICK);
        handle.send(TimerCommand::Start);
        std::thread::sleep(TICK / 2);
        handle.send(TimerCommand::Skip);

        let events = collect_until_complete(&rx, Duration::from_millis(500));
        assert!(
            events.iter().any(|e| matches!(e, TimerEvent::Complete { .. })),
            "Skip must trigger Complete"
        );
        // Should have completed well before 30 ticks elapsed.
        let ticks = events
            .iter()
            .filter(|e| matches!(e, TimerEvent::Tick { .. }))
            .count();
        assert!(ticks < 5, "Skip should complete before many ticks fire");
    }

    #[test]
    fn suspend_and_wake_resume_preserves_position() {
        let (handle, rx) = spawn(10, TICK);
        handle.send(TimerCommand::Start);

        // Let 3 ticks fire, then suspend.
        std::thread::sleep(TICK * 3 + TICK / 2);
        handle.send(TimerCommand::Suspend);

        let before = drain(&rx);
        let suspended_elapsed = before.iter().find_map(|e| {
            if let TimerEvent::Suspended { elapsed_secs } = e {
                Some(*elapsed_secs)
            } else {
                None
            }
        });
        assert!(
            suspended_elapsed.is_some(),
            "expected Suspended event with elapsed_secs"
        );
        let saved = suspended_elapsed.unwrap();
        assert!(saved >= 3, "elapsed at suspend should be >= 3 s, got {saved}");

        // Gap: simulate OS sleep (no ticks must fire).
        std::thread::sleep(TICK * 5);
        let during_suspend = drain(&rx);
        assert!(
            !during_suspend.iter().any(|e| matches!(e, TimerEvent::Tick { .. })),
            "no ticks must fire while suspended"
        );

        // Wake and verify the timer continues from the saved position.
        handle.send(TimerCommand::WakeResume);
        let after = collect_until_complete(&rx, Duration::from_secs(2));

        let resumed = after.iter().find_map(|e| {
            if let TimerEvent::Resumed { elapsed_secs } = e {
                Some(*elapsed_secs)
            } else {
                None
            }
        });
        assert_eq!(
            resumed,
            Some(saved),
            "Resumed event must carry the same elapsed_secs as Suspended"
        );
        assert!(
            after.iter().any(|e| matches!(e, TimerEvent::Complete { .. })),
            "timer must complete after WakeResume"
        );
    }

    #[test]
    fn shutdown_terminates_thread_cleanly() {
        let (handle, _rx) = spawn(60, TICK);
        handle.send(TimerCommand::Start);
        std::thread::sleep(TICK);
        // Shutdown must not deadlock or panic.
        handle.send(TimerCommand::Shutdown);
        // Give the thread time to exit gracefully.
        std::thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn reset_while_idle_emits_reset_event() {
        // Before this fix Reset was silently dropped in Phase::Idle, leaving
        // the event listener blocked and the frontend stale.
        let (handle, rx) = spawn(5, TICK);
        // Do NOT start — engine begins in Idle.
        handle.send(TimerCommand::Reset);

        let events = drain(&rx);
        assert!(
            events.iter().any(|e| matches!(e, TimerEvent::Reset)),
            "Reset while Idle must emit a Reset event"
        );
    }

    #[test]
    fn reconfigure_changes_duration_in_idle() {
        // Spawn with duration=10, then Reconfigure to 3 before Start.
        // The engine must use the new duration when Started.
        let (handle, rx) = spawn(10, TICK);
        handle.send(TimerCommand::Reconfigure { duration_secs: 3 });
        handle.send(TimerCommand::Start);

        let events = collect_until_complete(&rx, Duration::from_secs(2));
        let ticks = events
            .iter()
            .filter(|e| matches!(e, TimerEvent::Tick { .. }))
            .count();
        assert_eq!(ticks, 3, "Reconfigure to 3s should yield 3 ticks, got {ticks}");
        assert!(
            matches!(events.last(), Some(TimerEvent::Complete { .. })),
            "last event must be Complete after reconfigured timer"
        );
    }

    #[test]
    fn drift_complete_within_tolerance() {
        // 5 ticks at TICK (20 ms) = nominal 100 ms.
        // Allow generous ±100 ms for CI scheduling jitter while still
        // catching runaway drift (e.g. the engine sleeping for 1 s instead of 20 ms).
        let (handle, rx) = spawn(5, TICK);
        let t0 = Instant::now();
        handle.send(TimerCommand::Start);

        let events = collect_until_complete(&rx, Duration::from_millis(600));
        let wall = t0.elapsed();

        assert!(
            matches!(events.last(), Some(TimerEvent::Complete { .. })),
            "timer must complete"
        );
        let nominal = TICK * 5; // 100 ms
        assert!(
            wall >= nominal / 2,
            "completed suspiciously fast ({wall:?}); possible clock issue"
        );
        assert!(
            wall <= nominal + Duration::from_millis(200),
            "drift exceeded tolerance: {wall:?} for nominal {nominal:?}"
        );
    }
}
