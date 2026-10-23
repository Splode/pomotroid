/// Pomodoro round sequencing: work → short-break → work → … → long-break → work (cycle).
///
/// Mirrors the original app's behaviour:
/// - After each completed work round, check if `work_round_number >= work_rounds_total`.
///   If yes → long break; otherwise → short break.
/// - After short break → advance work_round_number, next round is Work.
/// - After long break → reset work_round_number to 1, next round is Work.
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

// ---------------------------------------------------------------------------
// Round type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RoundType {
    Work,
    ShortBreak,
    LongBreak,
}

impl RoundType {
    pub fn as_str(self) -> &'static str {
        match self {
            RoundType::Work => "work",
            RoundType::ShortBreak => "short-break",
            RoundType::LongBreak => "long-break",
        }
    }
}

// ---------------------------------------------------------------------------
// Sequence state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct SequenceState {
    pub current_round: RoundType,
    /// The round type that was active before `advance()` was last called.
    /// `None` on the very first round (no preceding round exists).
    pub previous_round: Option<RoundType>,
    /// Which work round we're currently in (1-based). Displayed to the user.
    pub work_round_number: u32,
    /// Total work rounds before a long break (from settings).
    pub work_rounds_total: u32,
    /// Monotonically-increasing count of work rounds since the last reset.
    /// Unlike `work_round_number` this never resets at cycle boundaries,
    /// so it can be used as a session counter when long breaks are disabled.
    pub session_work_count: u32,
    /// Work rounds finished since the current ladder began. Drives the
    /// incremental focus mode. It restarts at the base duration whenever a
    /// ladder reset fires — the start of a long break, a new calendar day, or a
    /// manual reset — unless the matching setting turns that trigger off, in
    /// which case the ladder keeps climbing across cycles and days.
    pub work_rounds_completed: u32,
    /// Local calendar date (`"YYYY-MM-DD"`) the ladder was last touched on, used
    /// to detect a day rollover. `None` until the first rollover check.
    pub ladder_day: Option<String>,
}

/// Result of a day-rollover check (see [`SequenceState::check_day_rollover`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DayRollover {
    /// The calendar day changed since the ladder was last touched.
    pub day_changed: bool,
    /// The ladder restarted from the base duration as a result.
    pub reset: bool,
}

impl SequenceState {
    pub fn new(work_rounds_total: u32) -> Self {
        Self {
            current_round: RoundType::Work,
            previous_round: None,
            work_round_number: 1,
            work_rounds_total,
            session_work_count: 1,
            work_rounds_completed: 0,
            ladder_day: None,
        }
    }

    /// Duration of the current round in seconds, taken from settings.
    ///
    /// In incremental focus mode a work round is lengthened by
    /// `time_work_increment_secs` for every work round already completed in the
    /// current ladder, capped at `time_work_max_secs` and never shorter than the
    /// configured base duration. Break durations are never escalated.
    pub fn current_duration_secs(&self, settings: &Settings) -> u32 {
        match self.current_round {
            RoundType::Work => self.work_duration_secs(settings),
            RoundType::ShortBreak => settings.time_short_break_secs,
            RoundType::LongBreak => settings.time_long_break_secs,
        }
    }

    /// The work duration that applies to the current ladder position.
    pub fn work_duration_secs(&self, settings: &Settings) -> u32 {
        let base = settings.time_work_secs;
        if !settings.incremental_work_enabled || settings.time_work_increment_secs == 0 {
            return base;
        }
        let step = settings
            .time_work_increment_secs
            .saturating_mul(self.work_rounds_completed);
        base.saturating_add(step).min(settings.time_work_max_secs.max(base))
    }

    /// How many increments have been applied to the work duration right now.
    /// Used by the frontend to show the current step on the ladder.
    pub fn work_increment_steps(&self, settings: &Settings) -> u32 {
        if !settings.incremental_work_enabled || settings.time_work_increment_secs == 0 {
            return 0;
        }
        self.work_rounds_completed
    }

    /// True when the current work duration has reached the configured ceiling
    /// and further rounds will no longer grow.
    pub fn work_duration_at_cap(&self, settings: &Settings) -> bool {
        if !settings.incremental_work_enabled || settings.time_work_increment_secs == 0 {
            return false;
        }
        let cap = settings.time_work_max_secs.max(settings.time_work_secs);
        let uncapped = settings
            .time_work_secs
            .saturating_add(settings.time_work_increment_secs.saturating_mul(self.work_rounds_completed));
        uncapped >= cap
    }

    /// Advance to the next round.  Returns `(next_round_type, duration_secs)`.
    ///
    /// Call this when the engine fires `TimerEvent::Complete`.
    pub fn advance(&mut self, settings: &Settings) -> (RoundType, u32) {
        self.previous_round = Some(self.current_round);
        let left_work_round = self.current_round == RoundType::Work;
        self.current_round = match self.current_round {
            RoundType::Work => {
                if self.work_round_number >= self.work_rounds_total {
                    // At the long-break point.
                    if settings.long_breaks_enabled {
                        RoundType::LongBreak
                    } else if settings.short_breaks_enabled {
                        // Substitute a short break; set to 0 so the ShortBreak→Work arm
                        // increments it to 1, preserving the cycle-reset invariant.
                        self.work_round_number = 0;
                        RoundType::ShortBreak
                    } else {
                        // Both breaks disabled: loop directly back to Work(1).
                        self.work_round_number = 1;
                        RoundType::Work
                    }
                } else if settings.short_breaks_enabled {
                    RoundType::ShortBreak
                } else {
                    // Short breaks disabled: skip directly to the next work round.
                    self.work_round_number += 1;
                    RoundType::Work
                }
            }
            RoundType::ShortBreak => {
                self.work_round_number += 1;
                RoundType::Work
            }
            RoundType::LongBreak => {
                self.work_round_number = 1;
                RoundType::Work
            }
        };

        // A completed work round extends the ladder for the rounds that follow.
        // Counted before the duration is computed so the very next work round
        // already reflects the increment.
        if left_work_round {
            self.work_rounds_completed = self.work_rounds_completed.saturating_add(1);
        }

        // A long break starting is the cycle boundary. By default it restarts
        // the ladder, so every cycle begins at the base duration. When the user
        // turns that off the ladder carries across cycles and keeps climbing.
        if self.current_round == RoundType::LongBreak && settings.incremental_reset_on_long_break {
            self.work_rounds_completed = 0;
        }

        // Increment the session counter every time we enter a new Work round.
        if self.current_round == RoundType::Work {
            self.session_work_count += 1;
        }

        let duration = self.current_duration_secs(settings);
        (self.current_round, duration)
    }

    /// Restart the ladder from the base duration, keeping the round and cycle
    /// counters untouched. Used by the manual "Reset Focus Ladder" action and by
    /// the long-break / day rollover triggers.
    ///
    /// `today` (when known) is recorded as the ladder's current day so a
    /// rollover is measured from now on. Returns true when the step counter
    /// actually changed.
    pub fn reset_ladder(&mut self, today: Option<&str>) -> bool {
        if let Some(day) = today {
            self.ladder_day = Some(day.to_string());
        }
        let changed = self.work_rounds_completed != 0;
        self.work_rounds_completed = 0;
        changed
    }

    /// Restart the ladder when the local calendar day changes.
    ///
    /// The current day is always recorded, so the next rollover is measured from
    /// today. When `incremental_reset_daily` is off the extra day is noted but
    /// the ladder is left climbing — turning the setting back on later will
    /// apply a reset on the next check.
    pub fn check_day_rollover(&mut self, settings: &Settings, today: &str) -> DayRollover {
        let previous = self.ladder_day.replace(today.to_string());
        match previous {
            Some(day) if day != today => DayRollover {
                day_changed: true,
                reset: settings.incremental_reset_daily && self.reset_ladder(Some(today)),
            },
            // First observation of a day, or still the same day: nothing to do.
            _ => DayRollover::default(),
        }
    }

    /// Reset the sequence to the initial state (used by the Reset command).
    pub fn reset(&mut self) {
        self.current_round = RoundType::Work;
        self.previous_round = None;
        self.work_round_number = 1;
        self.session_work_count = 1;
        self.work_rounds_completed = 0;
        self.ladder_day = None;
    }
}

// ---------------------------------------------------------------------------
// Tests (TIMER-02 acceptance: full cycles with various work_rounds values)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal `Settings` with the given durations.
    fn settings(work: u32, short: u32, long: u32) -> Settings {
        Settings {
            time_work_secs: work,
            time_short_break_secs: short,
            time_long_break_secs: long,
            long_break_interval: 4,
            ..Settings::default()
        }
    }

    /// Build settings with break-enable flags set explicitly.
    fn settings_with_flags(short_breaks_enabled: bool, long_breaks_enabled: bool) -> Settings {
        Settings {
            time_work_secs: 1500,
            time_short_break_secs: 300,
            time_long_break_secs: 900,
            long_break_interval: 4,
            short_breaks_enabled,
            long_breaks_enabled,
            ..Settings::default()
        }
    }

    /// Simulate `n` full cycles (each cycle = work_rounds × work + breaks + long break)
    /// and return a flat list of (round_type, duration) pairs.
    fn simulate_cycle(work_rounds: u32, cycles: u32) -> Vec<(RoundType, u32)> {
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(work_rounds);
        let mut result = Vec::new();

        // Record initial state.
        result.push((seq.current_round, seq.current_duration_secs(&s)));

        let total_rounds_per_cycle = work_rounds * 2; // work + break per work session, then long
        let steps = total_rounds_per_cycle * cycles;

        for _ in 0..steps {
            let (rt, dur) = seq.advance(&s);
            result.push((rt, dur));
        }
        result
    }

    #[test]
    fn single_work_round_cycle() {
        // work_rounds=1: Work → LongBreak → Work → LongBreak → …
        let rounds = simulate_cycle(1, 3);
        let types: Vec<_> = rounds.iter().map(|(rt, _)| *rt).collect();
        assert_eq!(
            types,
            vec![
                RoundType::Work,
                RoundType::LongBreak,
                RoundType::Work,
                RoundType::LongBreak,
                RoundType::Work,
                RoundType::LongBreak,
                RoundType::Work,
            ]
        );
    }

    #[test]
    fn two_work_rounds_cycle() {
        // work_rounds=2: Work → Short → Work → Long → Work → Short → …
        let rounds = simulate_cycle(2, 2);
        let types: Vec<_> = rounds.iter().map(|(rt, _)| *rt).collect();
        assert_eq!(
            types,
            vec![
                RoundType::Work,
                RoundType::ShortBreak,
                RoundType::Work,
                RoundType::LongBreak,
                RoundType::Work,
                RoundType::ShortBreak,
                RoundType::Work,
                RoundType::LongBreak,
                RoundType::Work,
            ]
        );
    }

    #[test]
    fn four_work_rounds_cycle() {
        // The default work_rounds=4 cycle.
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(4);

        // Initial state check (before any advance).
        assert_eq!(seq.current_round, RoundType::Work);
        assert_eq!(seq.current_duration_secs(&s), 1500);

        // Expected results of successive advance() calls.
        let expected = vec![
            (RoundType::ShortBreak, 300u32), // Work(1) → ShortBreak
            (RoundType::Work, 1500),          // ShortBreak → Work(2)
            (RoundType::ShortBreak, 300),     // Work(2) → ShortBreak
            (RoundType::Work, 1500),          // ShortBreak → Work(3)
            (RoundType::ShortBreak, 300),     // Work(3) → ShortBreak
            (RoundType::Work, 1500),          // ShortBreak → Work(4)
            (RoundType::LongBreak, 900),      // Work(4) → LongBreak (4 == total)
            (RoundType::Work, 1500),          // LongBreak → Work(1) — cycle 2
            (RoundType::ShortBreak, 300),     // Work(1) → ShortBreak
        ];

        for (i, (exp_type, exp_dur)) in expected.iter().enumerate() {
            let (rt, dur) = seq.advance(&s);
            assert_eq!(
                rt, *exp_type,
                "step {i}: expected {exp_type:?}, got {rt:?}"
            );
            assert_eq!(
                dur, *exp_dur,
                "step {i}: expected duration {exp_dur}, got {dur}"
            );
        }
    }

    #[test]
    fn twelve_work_rounds_cycle() {
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(12);

        // Simulate one full cycle: 12 work + 11 short + 1 long = 24 advances.
        let mut work_count = 0;
        let mut short_count = 0;
        let mut long_count = 0;

        for _ in 0..24 {
            let (rt, _) = seq.advance(&s);
            match rt {
                RoundType::Work => work_count += 1,
                RoundType::ShortBreak => short_count += 1,
                RoundType::LongBreak => long_count += 1,
            }
        }

        // After 24 advances from the initial Work state we should have:
        // 11 short breaks, 1 long break, and 12 work rounds.
        assert_eq!(short_count, 11, "12-round cycle should have 11 short breaks");
        assert_eq!(long_count, 1, "12-round cycle should have 1 long break");
        assert_eq!(work_count, 12, "12-round cycle should have 12 work rounds");
    }

    #[test]
    fn work_round_number_resets_after_long_break() {
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(2);

        seq.advance(&s); // → ShortBreak
        assert_eq!(seq.work_round_number, 1);

        seq.advance(&s); // → Work(2)
        assert_eq!(seq.work_round_number, 2);

        seq.advance(&s); // → LongBreak
        assert_eq!(seq.work_round_number, 2, "number stays during long break");

        seq.advance(&s); // → Work(1) — cycle reset
        assert_eq!(seq.work_round_number, 1, "number must reset to 1 after long break");
    }

    #[test]
    fn reset_returns_to_initial_state() {
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(4);

        seq.advance(&s);
        seq.advance(&s);
        seq.reset();

        assert_eq!(seq.current_round, RoundType::Work);
        assert_eq!(seq.work_round_number, 1);
        assert_eq!(seq.current_duration_secs(&s), 1500);
    }

    #[test]
    fn work_round_number_increments_on_each_work_completion() {
        // Verifies that work_round_number advances by 1 after each Work→Break→Work
        // transition and resets to 1 after a long break.
        let s = settings(1500, 300, 900);
        let mut seq = SequenceState::new(4);

        assert_eq!(seq.work_round_number, 1, "initial work_round_number is 1");

        // Complete work rounds 1→2→3→4.
        for expected in 2..=4u32 {
            seq.advance(&s); // Work(n) → ShortBreak
            seq.advance(&s); // ShortBreak → Work(n+1)
            assert_eq!(
                seq.work_round_number, expected,
                "work_round_number should be {expected} after completing round {}",
                expected - 1
            );
        }

        // Work(4) → LongBreak → Work(1).
        seq.advance(&s); // → LongBreak
        seq.advance(&s); // → Work(1)
        assert_eq!(
            seq.work_round_number, 1,
            "work_round_number must reset to 1 after long break"
        );
    }

    // -----------------------------------------------------------------------
    // Optional-breaks tests
    // -----------------------------------------------------------------------

    #[test]
    fn short_breaks_disabled_chains_work_rounds() {
        // short=false, long=true: Work rounds chain directly; long break still fires.
        let s = settings_with_flags(false, true);
        let mut seq = SequenceState::new(4);

        // Work(1) → Work(2) → Work(3) → Work(4) → LongBreak → Work(1)
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 2);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 3);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 4);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::LongBreak, "long break must still fire at round 4");

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 1, "counter must reset to 1 after long break");
    }

    #[test]
    fn long_breaks_disabled_substitutes_short_break() {
        // short=true, long=false: short break substituted at the long-break point.
        let s = settings_with_flags(true, false);
        let mut seq = SequenceState::new(2);

        // Work(1) → ShortBreak (normal) → Work(2) → ShortBreak (substituted) → Work(1)
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak, "normal short break before long-break point");

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 2);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak, "short break substituted at long-break point");

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 1, "counter must reset to 1 after substituted short break");
    }

    #[test]
    fn both_breaks_disabled_pure_work_loop() {
        // short=false, long=false: pure work loop; counter increments and resets.
        let s = settings_with_flags(false, false);
        let mut seq = SequenceState::new(3);

        // Work(1) → Work(2) → Work(3) → Work(1) — cycle
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 2);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 3);

        // At long-break point with both disabled → Work(1)
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 1, "counter must reset to 1 at cycle boundary");

        // Continues correctly in the next cycle.
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 2);
    }

    #[test]
    fn long_breaks_disabled_short_breaks_fire_normally() {
        // short=true, long=false: short breaks still fire before the long-break point.
        let s = settings_with_flags(true, false);
        let mut seq = SequenceState::new(3);

        // Work(1) → ShortBreak → Work(2) → ShortBreak → Work(3) → ShortBreak* → Work(1)
        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak, "short break fires at round 1 (before long-break point)");

        seq.advance(&s); // → Work(2)

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak, "short break fires at round 2 (before long-break point)");

        seq.advance(&s); // → Work(3)

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak, "short break substituted at long-break point when long=false");

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(seq.work_round_number, 1, "counter resets to 1");
    }

    // -----------------------------------------------------------------------
    // Incremental focus mode
    // -----------------------------------------------------------------------

    /// Settings with incremental focus enabled: 5 min base, +5 min per round, 20 min cap.
    fn incremental_settings() -> Settings {
        Settings {
            time_work_secs: 5 * 60,
            time_short_break_secs: 5 * 60,
            time_long_break_secs: 15 * 60,
            long_break_interval: 4,
            incremental_work_enabled: true,
            time_work_increment_secs: 5 * 60,
            time_work_max_secs: 20 * 60,
            ..Settings::default()
        }
    }

    #[test]
    fn incremental_disabled_keeps_flat_duration() {
        let s = Settings {
            time_work_secs: 5 * 60,
            time_short_break_secs: 5 * 60,
            time_long_break_secs: 15 * 60,
            long_break_interval: 2,
            incremental_work_enabled: false,
            time_work_increment_secs: 5 * 60,
            time_work_max_secs: 20 * 60,
            ..Settings::default()
        };
        let mut seq = SequenceState::new(2);

        assert_eq!(seq.current_duration_secs(&s), 5 * 60);
        for _ in 0..6 {
            let (rt, dur) = seq.advance(&s);
            if rt == RoundType::Work {
                assert_eq!(dur, 5 * 60, "work duration must stay flat when the feature is off");
            }
        }
    }

    #[test]
    fn incremental_escalates_work_rounds_and_caps() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(6);

        // Round 1 starts at the base duration.
        assert_eq!(seq.current_duration_secs(&s), 300, "first round is the base duration");

        // Each Work round is 5 min longer than the previous, until the 20 min cap.
        let expected_work = [300u32, 600, 900, 1200, 1200, 1200];
        for (i, want) in expected_work.iter().enumerate() {
            if i > 0 {
                // Advance Work → ShortBreak → Work.
                let (rt, dur) = seq.advance(&s);
                assert_eq!(rt, RoundType::ShortBreak);
                assert_eq!(dur, 300, "break durations are never escalated");
                seq.advance(&s);
            }
            assert_eq!(
                seq.current_round,
                RoundType::Work,
                "step {i}: expected to be on a work round"
            );
            assert_eq!(
                seq.current_duration_secs(&s),
                *want,
                "step {i}: unexpected work duration"
            );
        }
    }

    #[test]
    fn incremental_ladder_resets_after_long_break() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(2);

        assert_eq!(seq.current_duration_secs(&s), 300);

        seq.advance(&s); // → ShortBreak
        let (rt, dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(dur, 600, "second work round gains one increment");

        let (rt, _dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::LongBreak, "long break at the cycle boundary");
        assert_eq!(
            seq.work_rounds_completed, 0,
            "the ladder restarts as soon as the long break begins"
        );

        let (rt, dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(dur, 300, "a new cycle restarts at the base duration");
        assert_eq!(seq.work_rounds_completed, 0, "ladder stays at the base");
    }

    #[test]
    fn incremental_ladder_survives_long_breaks_when_reset_is_off() {
        let s = Settings {
            incremental_reset_on_long_break: false,
            ..incremental_settings()
        };
        let mut seq = SequenceState::new(2);

        seq.advance(&s); // Work(1) → ShortBreak
        let (rt, dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(dur, 600);

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::LongBreak);
        assert_eq!(
            seq.work_rounds_completed, 2,
            "the ladder keeps its steps across the long break"
        );

        // Leaving the long break continues the ladder instead of restarting it.
        let (rt, dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(dur, 900, "the next cycle continues climbing");
        assert_eq!(seq.work_rounds_completed, 2);
    }

    #[test]
    fn incremental_ladder_climbs_across_cycles_until_the_cap() {
        let s = Settings {
            incremental_reset_on_long_break: false,
            ..incremental_settings()
        };
        let mut seq = SequenceState::new(1); // every work round ends in a long break

        // 5 → 10 → 15 → 20 (cap) → 20 → 20 …
        let expected = [300u32, 600, 900, 1200, 1200, 1200];
        for (i, want) in expected.iter().enumerate() {
            if i > 0 {
                seq.advance(&s); // → LongBreak
                let (rt, dur) = seq.advance(&s);
                assert_eq!(rt, RoundType::Work);
                assert_ne!(dur, 0);
            }
            assert_eq!(
                seq.current_duration_secs(&s),
                *want,
                "step {i}: the ladder must keep climbing across cycles"
            );
        }
        assert!(seq.work_duration_at_cap(&s));
        assert_eq!(seq.work_increment_steps(&s), 5, "capped rounds still count steps");
    }

    #[test]
    fn incremental_ladder_resets_on_manual_reset() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(8);

        seq.advance(&s); // → ShortBreak
        seq.advance(&s); // → Work (600s)
        assert_eq!(seq.current_duration_secs(&s), 600);

        seq.reset();
        assert_eq!(seq.work_rounds_completed, 0);
        assert_eq!(seq.current_duration_secs(&s), 300, "reset returns to the base duration");
    }

    #[test]
    fn reset_ladder_keeps_the_round_counters() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(8);

        seq.advance(&s); // → ShortBreak
        seq.advance(&s); // → Work(2), ladder at 600s
        assert_eq!(seq.work_round_number, 2);
        assert_eq!(seq.session_work_count, 2);
        assert_eq!(seq.work_rounds_completed, 1);

        assert!(seq.reset_ladder(Some("2026-05-01")), "the ladder had steps to clear");

        assert_eq!(seq.work_rounds_completed, 0, "ladder restarts at the base");
        assert_eq!(seq.current_duration_secs(&s), 300);
        assert_eq!(seq.work_round_number, 2, "round counter is untouched");
        assert_eq!(seq.session_work_count, 2, "session counter is untouched");
        assert_eq!(seq.current_round, RoundType::Work, "round type is untouched");
        assert_eq!(seq.ladder_day.as_deref(), Some("2026-05-01"));

        assert!(!seq.reset_ladder(Some("2026-05-01")), "nothing left to reset");
    }

    #[test]
    fn day_rollover_resets_the_ladder_when_enabled() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(8);

        seq.advance(&s); // → ShortBreak
        seq.advance(&s); // → Work(2), ladder at 600s
        assert_eq!(seq.work_rounds_completed, 1);

        // Same day: no change.
        let first = seq.check_day_rollover(&s, "2026-05-01");
        assert_eq!(first, DayRollover { day_changed: false, reset: false });
        assert_eq!(seq.work_rounds_completed, 1);

        // A new day restarts the ladder.
        let rollover = seq.check_day_rollover(&s, "2026-05-02");
        assert_eq!(rollover, DayRollover { day_changed: true, reset: true });
        assert_eq!(seq.work_rounds_completed, 0, "a new day starts at the base duration");
        assert_eq!(seq.current_duration_secs(&s), 300);
        assert_eq!(seq.ladder_day.as_deref(), Some("2026-05-02"));

        // And the new day is remembered.
        let again = seq.check_day_rollover(&s, "2026-05-02");
        assert_eq!(again, DayRollover { day_changed: false, reset: false });
    }

    #[test]
    fn day_rollover_keeps_the_ladder_when_daily_reset_is_off() {
        let s = Settings {
            incremental_reset_daily: false,
            ..incremental_settings()
        };
        let mut seq = SequenceState::new(8);
        // The ladder is tagged with today on its first check.
        let _ = seq.check_day_rollover(&s, "2026-05-01");

        seq.advance(&s); // → ShortBreak
        seq.advance(&s); // → Work(2), ladder at 600s

        let rollover = seq.check_day_rollover(&s, "2026-05-02");
        assert_eq!(
            rollover,
            DayRollover { day_changed: true, reset: false },
            "the day is recorded but the ladder is left alone"
        );
        assert_eq!(seq.work_rounds_completed, 1, "the ladder keeps climbing");
        assert_eq!(seq.current_duration_secs(&s), 600);
        assert_eq!(seq.ladder_day.as_deref(), Some("2026-05-02"));

        // Turning the setting back on applies the missed reset on the next day.
        let s2 = incremental_settings();
        let rollover = seq.check_day_rollover(&s2, "2026-05-03");
        assert_eq!(rollover, DayRollover { day_changed: true, reset: true });
        assert_eq!(seq.work_rounds_completed, 0);
    }

    #[test]
    fn day_rollover_ignores_the_ladder_when_it_is_already_at_zero() {        let s = incremental_settings();
        let mut seq = SequenceState::new(8);
        let _ = seq.check_day_rollover(&s, "2026-05-01");

        let rollover = seq.check_day_rollover(&s, "2026-05-02");
        assert_eq!(
            rollover,
            DayRollover { day_changed: true, reset: false },
            "an untouched ladder has nothing to restart"
        );
        assert_eq!(seq.ladder_day.as_deref(), Some("2026-05-02"));
    }

    #[test]
    fn incremental_never_shrinks_below_base_when_cap_is_smaller() {
        let s = Settings {
            time_work_secs: 25 * 60,
            incremental_work_enabled: true,
            time_work_increment_secs: 5 * 60,
            time_work_max_secs: 10 * 60, // misconfigured: cap below the base
            long_break_interval: 4,
            ..Settings::default()
        };
        let seq = SequenceState::new(4);
        assert_eq!(
            seq.current_duration_secs(&s),
            25 * 60,
            "cap below the base duration must not shorten the work round"
        );
    }

    #[test]
    fn incremental_steps_and_cap_flags_track_the_ladder() {
        let s = incremental_settings();
        let mut seq = SequenceState::new(5);

        assert_eq!(seq.work_increment_steps(&s), 0);
        assert!(!seq.work_duration_at_cap(&s));

        // Work(1)→SB→Work(2)→SB→Work(3)→SB→Work(4)→SB→Work(5)→Long → Work(1)
        // — a full cycle, which restarts the ladder.
        for _ in 0..10 {
            seq.advance(&s);
        }
        assert_eq!(seq.current_round, RoundType::Work);
        assert_eq!(seq.work_round_number, 1, "back at the start of a new cycle");
        assert_eq!(seq.work_increment_steps(&s), 0, "ladder restarts each cycle");
        assert!(!seq.work_duration_at_cap(&s));
        assert_eq!(seq.current_duration_secs(&s), 300);

        // Walk part-way into the next cycle and confirm the cap has engaged.
        // Three work→break pairs from Work(1) land on Work(4).
        for _ in 0..3 {
            seq.advance(&s); // Work(n) → ShortBreak
            seq.advance(&s); // ShortBreak → Work(n+1)
        }
        assert_eq!(seq.current_round, RoundType::Work);
        assert_eq!(seq.work_round_number, 4, "three work rounds completed this cycle");
        assert_eq!(seq.work_increment_steps(&s), 3);
        assert!(seq.work_duration_at_cap(&s), "300 + 300×3 reaches the 1200 s cap");
        assert_eq!(seq.work_duration_secs(&s), 1200);
    }

    #[test]
    fn work_round_finishing_after_midnight_does_not_extend_the_new_ladder() {
        // Mirrors how the timer event listener composes these calls when a work
        // round that started yesterday completes just after midnight.
        let s = incremental_settings();
        let mut seq = SequenceState::new(4);
        let _ = seq.check_day_rollover(&s, "2026-05-01");

        seq.advance(&s); // Work(1) → ShortBreak
        seq.advance(&s); // → Work(2), ladder at 600 s
        assert_eq!(seq.work_rounds_completed, 1);

        // Midnight passes while the second work round is still running.
        let rollover = seq.check_day_rollover(&s, "2026-05-02");
        assert!(rollover.reset, "the new day restarts the ladder");

        let (rt, _) = seq.advance(&s);
        assert_eq!(rt, RoundType::ShortBreak);
        // The listener drops the step that the finished round had just added.
        seq.reset_ladder(Some("2026-05-02"));

        // So the first work round of the new day runs at the base duration.
        let (rt, dur) = seq.advance(&s);
        assert_eq!(rt, RoundType::Work);
        assert_eq!(dur, 300, "the new day starts at the base duration");
        assert_eq!(seq.work_increment_steps(&s), 0);
    }
}
