## Why

The Pomodoro technique prescribes fixed work/break cycles, but real workflows vary. Some users prefer to chain work rounds without interruption and take breaks manually; others want short breaks but find long breaks disruptive to their flow. Currently every break type fires unconditionally — there is no way to disable short or long breaks without deleting the app entirely or using an obscure workaround. This is a persistent workflow customization, not a one-time skip.

## What Changes

- Add two boolean settings: `short_breaks_enabled` (default `true`) and `long_breaks_enabled` (default `true`).
- When short breaks are disabled, the sequence skips ShortBreak entirely; work rounds advance directly to the next work round (counter increments normally).
- When long breaks are disabled, the sequence substitutes a ShortBreak at the long-break point (if short breaks are enabled) or loops directly back to Work(1) (if both are disabled). The round counter resets as it would have at the long-break point either way.
- Settings → Timer gains two "Disable" toggles — one above the Short Break duration slider, one above the Long Break duration slider. Toggles are off by default (breaks are enabled); turning one on opts out of that break type. When a break type is disabled, its duration slider (and for long breaks, the Rounds until Long Break slider) is visually dimmed and non-interactive.
- The round counter in the timer footer (`X / Y`) only makes sense when long breaks are active. When long breaks are disabled, it switches to a session counter that reads "round N" — a rolling count of focus rounds completed since the last reset.

## Capabilities

### New Capabilities

- none

### Modified Capabilities

- `timer-sequence`: Add requirement that short breaks and long breaks can each be independently disabled; document the resulting cycle behaviour for all four combinations.
- `settings`: Add `short_breaks_enabled` and `long_breaks_enabled` setting keys, both defaulting to `true`.

## Impact

- **Rust**: `src-tauri/src/timer/sequence.rs` — `advance()` receives `&Settings` already; add two flag checks to skip or substitute break types.
- **Rust**: `src-tauri/src/settings/mod.rs` — add `short_breaks_enabled: bool` and `long_breaks_enabled: bool` fields and defaults.
- **Rust**: `src-tauri/src/settings/defaults.rs` — add both keys with value `"true"`.
- **Rust**: `src-tauri/src/db/migrations.rs` — migration 5 seeds both new keys via `INSERT OR IGNORE`.
- **Frontend**: `src/lib/types.ts` — add both fields to `Settings` interface.
- **Frontend**: `src/lib/stores/settings.ts` — add both fields to the defaults object.
- **Frontend**: `src/lib/components/settings/sections/TimerSection.svelte` — add two `SettingsToggle` rows and wrap dependent sliders in a `disabled` container.
- **Frontend**: `src/messages/*.json` — add i18n keys for both toggles and the session round label (`timer_session_round`) across all 8 locales.
- **Rust**: `src-tauri/src/timer/sequence.rs` — add `session_work_count` field to `SequenceState`; exposed via `TimerSnapshot` for the frontend.
- **Frontend**: `src/lib/components/TimerFooter.svelte` — switch round counter display based on `long_breaks_enabled`.
