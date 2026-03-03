## 1. DB Migration (Rust)

- [x] 1.1 Add `MIGRATION_2` to `src-tauri/src/db/migrations.rs`: INSERT `time_work_secs`, `time_short_break_secs`, `time_long_break_secs` rows from existing `*_mins` values × 60, then DELETE the `*_mins` rows, then INSERT schema_version 2
- [x] 1.2 Add `if version < 2 { ... }` block to `run()` in `migrations.rs`
- [x] 1.3 Update `src-tauri/src/settings/defaults.rs`: replace the three `*_mins` entries with `time_work_secs = "1500"`, `time_short_break_secs = "300"`, `time_long_break_secs = "900"`
- [x] 1.4 Update `src-tauri/src/settings/mod.rs` `load()`: read from `time_work_secs`, `time_short_break_secs`, `time_long_break_secs` directly (remove `* 60` multiplication)
- [x] 1.5 Update `src-tauri/src/settings/mod.rs` tests: fix `save_and_reload_time` and `reset_defaults_restores_all_settings` to use new `*_secs` key names and second values; add a migration test asserting `*_mins` rows are absent after MIGRATION_2

## 2. Frontend IPC Update

- [x] 2.1 Update `TimerSection.svelte`: change `handleChange` calls from `'time_work_mins'` / `'time_short_break_mins'` / `'time_long_break_mins'` to `'time_work_secs'` / `'time_short_break_secs'` / `'time_long_break_secs'`
- [x] 2.2 Update the slider `oninput` handlers to pass `valueAsNumber * 60` (seconds) instead of bare minutes

## 3. Editable Badge Component

- [x] 3.1 Add a `parseMMSS(input: string): number | null` utility in `TimerSection.svelte` (or a shared util): accept `MM:SS` and bare integer-minutes; return total seconds or `null` on invalid input
- [x] 3.2 Add a `formatMMSS(totalSecs: number): string` utility: formats as `M:SS` or `MM:SS`
- [x] 3.3 Replace the `.slider-value` `<span>` elements for the three timer rows with `<input type="text">` elements bound to local editable state; display `formatMMSS(currentSecs)` when not editing
- [x] 3.4 Implement `oncommit` logic: on Enter / Tab / blur, call `parseMMSS`, clamp to [60, 5400], save via `handleChange`, update slider position to `Math.round(secs / 60)`; revert to previous display value on null parse
- [x] 3.5 Implement `onfocus` / click handler to select all text in the input for easy overwrite
- [x] 3.6 Style the editable badge to be visually identical to the current static badge when unfocused; show a subtle focus ring or border change when active

## 4. Statistics Rounding Fix (Rust)

- [x] 4.1 Update `focus_mins` calculation in `src-tauri/src/db/queries.rs` `get_daily_stats()`: change `(focus_secs / 60) as u32` to `((focus_secs + 30) / 60) as u32` to round to nearest minute
- [x] 4.2 Update the `focus_mins` Rust test in `queries.rs` to add a case asserting 339 s → 6 min and 324 s → 5 min

## 5. Verification

- [x] 5.1 Run `cargo test --manifest-path src-tauri/Cargo.toml` — all Rust tests pass
- [x] 5.2 Run `npm run check` — zero svelte-check errors
- [x] 5.3 Manual smoke test: enter `5:39` for Focus, restart app, confirm timer starts at 5:39
- [x] 5.4 Manual smoke test: drag slider to 30, confirm badge shows `30:00` and timer reflects 30 min
- [x] 5.5 Manual smoke test: enter `0:10` (below min), confirm it clamps to `1:00`
- [x] 5.6 Manual smoke test: enter `abc`, confirm badge reverts to previous value
- [x] 5.7 Manual smoke test: open fresh install (or delete DB), confirm default values are 25:00 / 5:00 / 15:00
- [x] 5.8 Manual smoke test: change Focus duration while timer is idle, confirm main window dial updates immediately
- [x] 5.9 Manual smoke test: complete a 5:39 Focus session, open Statistics, confirm focus time shows `6m`
