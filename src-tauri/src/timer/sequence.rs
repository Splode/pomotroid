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
}

impl SequenceState {
    pub fn new(work_rounds_total: u32) -> Self {
        Self {
            current_round: RoundType::Work,
            previous_round: None,
            work_round_number: 1,
            work_rounds_total,
            session_work_count: 1,
        }
    }

    /// Duration of the current round in seconds, taken from settings.
    pub fn current_duration_secs(&self, settings: &Settings) -> u32 {
        match self.current_round {
            RoundType::Work => settings.time_work_secs,
            RoundType::ShortBreak => settings.time_short_break_secs,
            RoundType::LongBreak => settings.time_long_break_secs,
        }
    }

    /// Advance to the next round.  Returns `(next_round_type, duration_secs)`.
    ///
    /// Call this when the engine fires `TimerEvent::Complete`.
    pub fn advance(&mut self, settings: &Settings) -> (RoundType, u32) {
        self.previous_round = Some(self.current_round);
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

        // Increment the session counter every time we enter a new Work round.
        if self.current_round == RoundType::Work {
            self.session_work_count += 1;
        }

        let duration = self.current_duration_secs(settings);
        (self.current_round, duration)
    }

    /// Reset the sequence to the initial state (used by the Reset command).
    pub fn reset(&mut self) {
        self.current_round = RoundType::Work;
        self.previous_round = None;
        self.work_round_number = 1;
        self.session_work_count = 1;
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
}
