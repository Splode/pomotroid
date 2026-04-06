## 1. Database Migration

- [x] 1.1 Add `MIGRATION_7` constant in `src-tauri/src/db/migrations.rs`: `ALTER TABLE sessions ADD COLUMN label TEXT`
- [x] 1.2 Increment schema version check in `migrations::run()` to include migration 7

## 2. Rust — Session Storage

- [x] 2.1 Update `complete_session()` in `src-tauri/src/db/queries.rs` to accept `label: Option<&str>` and bind it in the UPDATE statement
- [x] 2.2 Add `LabelStat` struct (`label: Option<String>`, `duration_mins: u32`) to `src-tauri/src/db/queries.rs`
- [x] 2.3 Add `get_label_breakdown(conn, period: &str)` query in `src-tauri/src/db/queries.rs` that groups completed work sessions by label, sums duration_secs, converts to minutes, and accepts `"today"`, `"week"`, or `"alltime"` as period

## 3. Rust — TimerController

- [x] 3.1 Add `current_label: Arc<Mutex<Option<String>>>` field to `TimerController` in `src-tauri/src/timer/mod.rs`, initialized to `None`
- [x] 3.2 Pass `current_label` into the timer event loop closure so `complete_session()` can read it at round completion
- [x] 3.3 Update the `TimerEvent::Complete` handler to read `current_label` under lock and pass the value to `complete_session()`

## 4. Rust — IPC Commands

- [x] 4.1 Add `timer_set_label(label: Option<String>, timer: State<TimerController>)` command in `src-tauri/src/commands.rs`; normalize empty string to `None` before storing
- [x] 4.2 Emit `label:clear` event from the `timer_reset` command after resetting the timer
- [x] 4.3 Emit `label:clear` event from `settings_reset_defaults` after emitting `timer:reset`
- [x] 4.4 Add `stats_get_label_breakdown(period: String, db: State<DbConn>)` command in `src-tauri/src/commands.rs` that calls `queries::get_label_breakdown()`
- [x] 4.5 Register `timer_set_label` and `stats_get_label_breakdown` in `tauri::Builder::invoke_handler()`

## 5. Frontend — IPC Layer

- [x] 5.1 Add `timerSetLabel(label: string | null): Promise<void>` wrapper in `src/lib/ipc/index.ts`
- [x] 5.2 Add `statsGetLabelBreakdown(period: 'today' | 'week' | 'alltime'): Promise<LabelStat[]>` wrapper in `src/lib/ipc/index.ts`
- [x] 5.3 Add `onLabelClear(cb: () => void)` event listener wrapper in `src/lib/ipc/index.ts`
- [x] 5.4 Add `LabelStat` type (`label: string | null, duration_mins: number`) to `src/lib/types.ts`

## 6. Frontend — Timer UI

- [x] 6.1 Create `src/lib/components/TaskLabelInput.svelte`: a text input with no visible border unfocused, subtle border on focus, max 48 chars, placeholder "what are you working on?", accepts `value` prop and `onchange` callback
- [x] 6.2 Add `taskLabel = $state('')` to `src/lib/components/Timer.svelte` (placed here rather than +page.svelte as it owns all timer IPC)
- [x] 6.3 Add `onLabelClear` listener in `Timer.svelte` `onMount` that sets `taskLabel = ''` and calls `timerSetLabel(null)`
- [x] 6.4 Render `<TaskLabelInput>` in `Timer.svelte` below the round label, conditionally: `!isCompact && state.round_type === 'work'`
- [x] 6.5 Wire `TaskLabelInput` `onchange` to debounce (~350ms) and call `timerSetLabel(value || null)`

## 7. Frontend — Stats: Label Breakdown Component

- [x] 7.1 Create `src/lib/components/stats/LabelBreakdown.svelte`: accepts `entries: LabelStat[]` and `variant: 'pie' | 'list'` props
- [x] 7.2 Implement pie/donut chart in `LabelBreakdown` using inline SVG: compute slice angles from `duration_mins`, cap at top 4 labels + "Other" for pie variant, render "(unlabeled)" slice last in muted color
- [x] 7.3 Implement ranked list in `LabelBreakdown`: render rows sorted by `duration_mins` descending, each row with label name, formatted duration, and proportional inline bar; "(unlabeled)" row rendered last

## 8. Frontend — Stats: Integrate Breakdowns

- [x] 8.1 Add `labelBreakdown = $state<LabelStat[]>([])` and load/refresh logic to `src/routes/stats/+page.svelte`; call `statsGetLabelBreakdown(activeTab period)` on tab switch and on `timer:round-change`
- [x] 8.2 Add `<LabelBreakdown entries={labelBreakdown} variant="pie" />` to `src/lib/components/stats/DailyView.svelte`, hidden when `entries` is empty
- [x] 8.3 Add `<LabelBreakdown entries={labelBreakdown} variant="list" />` to `src/lib/components/stats/WeeklyView.svelte`, hidden when `entries` is empty
- [x] 8.4 Add `<LabelBreakdown entries={labelBreakdown} variant="list" />` to `src/lib/components/stats/YearlyView.svelte`, hidden when `entries` is empty
