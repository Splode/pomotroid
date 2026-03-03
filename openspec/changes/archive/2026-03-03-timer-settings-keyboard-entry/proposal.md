## Why

The timer duration sliders in Settings → Timer only support whole-minute increments, preventing users from setting precise durations like 25:30 or 5:45. Keyboard entry with MM:SS format gives users full control over their Pomodoro rhythm without forcing them into one-minute steps.

## What Changes

- The time display badge next to each timer slider label (Focus, Short Break, Long Break) becomes an editable input that accepts MM:SS format
- Clicking the badge activates edit mode; pressing Enter or blurring the field commits the value
- Valid range is 1:00–90:00; values outside this range are clamped on commit
- Arbitrary MM:SS values (e.g. 5:39) are accepted — not just whole minutes
- Timer duration settings are stored in whole seconds in the database instead of whole minutes, enabling sub-minute precision
- A DB migration converts existing minute-based values to seconds

## Capabilities

### New Capabilities

- `timer-duration-keyboard-entry`: Inline editable MM:SS input replacing the static time badge in the Timer settings section, with validation, clamping, and commit-on-blur/Enter behaviour.

### Modified Capabilities

- `settings`: Timer duration DB keys change from minute-resolution integers (`time_work_mins`, `time_short_break_mins`, `time_long_break_mins`) to second-resolution integers (`time_work_secs`, `time_short_break_secs`, `time_long_break_secs`), requiring a migration for existing users.

## Impact

- `src/lib/components/settings/sections/TimerSection.svelte` — new editable badge component; slider `oninput` continues to save seconds
- `src-tauri/src/settings/mod.rs` — DB key name changes; Settings struct field types unchanged (already in seconds)
- `src-tauri/src/db/migrations.rs` — new migration converts existing `time_work_mins` etc. rows to second values under new keys
- `src/lib/ipc/index.ts` — DB key constants updated
- `src/lib/types.ts` — no type changes; fields already named `*_secs`
