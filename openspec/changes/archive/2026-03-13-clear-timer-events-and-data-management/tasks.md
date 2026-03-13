## 1. Rust backend — sessions_clear command

- [x] 1.1 In `src-tauri/src/commands.rs`, add `sessions_clear` command: acquire DB connection, emit `log::info!("[sessions] clearing all session history")`, run `DELETE FROM sessions` in a transaction, emit `log::info!("[sessions] cleared {n} rows")` on success or `log::error!("[sessions] failed to clear history: {e}")` on failure, return `Result<(), String>`
- [x] 1.2 Register `sessions_clear` in the `tauri::Builder::invoke_handler` list in `src-tauri/src/lib.rs`
- [x] 1.3 In `src-tauri/capabilities/default.json`, add `"core:default"` allow entry for `sessions_clear`
- [x] 1.4 Add a unit test in `commands.rs` or a dedicated test module: seed rows into `sessions`, call the handler, assert table is empty; assert calling again on empty table returns `Ok`

## 2. Frontend IPC

- [x] 2.1 In `src/lib/ipc/index.ts`, add `export async function clearSessionHistory(): Promise<void>` that invokes `sessions_clear`

## 3. SystemSection — data management group

- [x] 3.1 In `SystemSection.svelte`, import `resetSettings` and `clearSessionHistory` from `$lib/ipc`
- [x] 3.2 Add `confirmingReset` and `confirmingClear` boolean `$state` variables
- [x] 3.3 Add a `system_group_data` localisation key to all message catalogues (value: `"Data"`)
- [x] 3.4 Add a `<div class="group-heading">{m.system_group_data()}</div>` heading above the data actions, consistent with the existing Integrations / Language / Logging / System Tray headings
- [x] 3.5 Add the two action rows beneath the heading:
  - Clear Session History row using `confirmingClear` state (button → inline Cancel/Confirm)
  - Reset All Settings row using `confirmingReset` state (button → inline Cancel/Confirm), logic moved from AboutSection
- [x] 3.6 Style the action rows to match the existing reset-group styles in AboutSection (copy and adapt)

## 4. AboutSection — remove reset group

- [x] 4.1 Remove the `resetSettings` import and `confirming` state variable from `AboutSection.svelte`
- [x] 4.2 Remove the `reset-group` div and all its child elements from the template
- [x] 4.3 Remove the `handleReset` function
- [x] 4.4 Remove `.reset-group`, `.reset-row`, `.confirm-label`, `.confirm-reset` and related CSS rules from the `<style>` block

## 5. Verify

- [x] 5.1 Clicking Clear Session History shows inline confirmation; Cancel dismisses; Confirm clears rows and returns to button state
- [x] 5.2 Reset All Settings in System section behaves identically to its former location in About
- [x] 5.3 Settings → About no longer shows any reset or clear button
- [x] 5.4 Run `npm run check` — no type errors
- [x] 5.5 Run `cargo test` in `src-tauri/` — all tests pass
