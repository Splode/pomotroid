## Why

Users currently have no way to attribute completed Pomodoro rounds to specific tasks or activities, making statistics a raw count with no context. Adding a sticky task label to the timer lets users see time broken down by what they were actually working on.

## What Changes

- A text input appears below the round type label ("WORK #1") in the timer window, allowing users to name what they're working on
- The label is sticky — it persists across round transitions and breaks until explicitly cleared
- The label is hidden in compact mode (`w < 300 || h < 300`) and during break rounds
- Each completed work session stores the active label in the database
- The stats window gains label-based breakdowns: a pie chart in the daily view and ranked activity lists in weekly and all-time views
- A new `label:clear` Tauri event is emitted on explicit timer reset and "reset all to defaults"; settings changes (duration, rounds) do not clear the label

## Capabilities

### New Capabilities

- `task-labels`: Sticky task label input on the timer UI, per-session label storage in the DB, and label-based breakdown visualizations in the stats window

### Modified Capabilities

- `statistics`: Stats views gain label-breakdown sections (pie chart for daily, ranked list for weekly and all-time)
- `session-history-management`: Sessions table gains a nullable `label` column; session recording writes the active label at completion time

## Impact

- **DB**: New migration adds `label TEXT` (nullable) to `sessions` table
- **Rust**: `TimerController` gains a `current_label` field; `timer_reset` and `settings_reset_defaults` commands emit a new `label:clear` event; `complete_session` query gains a `label` parameter
- **IPC**: New `timer_set_label(label)` command; new `stats_get_label_breakdown(period)` command; `onLabelClear` event listener added to `ipc/index.ts`
- **Frontend**: `+page.svelte` gains label input state and renders `TaskLabelInput` component; `DailyView`, `WeeklyView`, `YearlyView` stats components gain label breakdown sections
- **No breaking changes**: `label` column is nullable; all existing stats queries are unaffected
