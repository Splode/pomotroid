## Why

The `sessions` table accumulates a row for every completed or skipped timer round with no way for the user to clear it, and the "Reset All Settings" action is buried in Settings → About where data-management actions don't belong. Grouping both destructive data actions in Settings → System gives users a single, predictable place to manage their stored data.

## What Changes

- A new **Clear Session History** action is added to Settings → System. It deletes all rows from the `sessions` SQLite table (round type, duration, completion status, timestamps). A two-step inline confirmation mirrors the existing Reset All Settings pattern.
- The existing **Reset All Settings** action moves from Settings → About to Settings → System, alongside Clear Session History.
- Settings → About loses the reset group; it retains only app metadata (version, links, log directory).

## Capabilities

### New Capabilities

- `session-history-management`: Clear all stored session history records from the `sessions` table via a confirmed destructive action in Settings → System.

### Modified Capabilities

- `settings`: The "Reset All Settings" action relocates from Settings → About to Settings → System. No change to its behaviour or confirmation flow.

## Impact

- `src/lib/components/settings/sections/AboutSection.svelte` — remove reset group and its styles
- `src/lib/components/settings/sections/SystemSection.svelte` — add data-management section with Reset All Settings (moved) and Clear Session History (new)
- `src/lib/ipc/index.ts` — add `clearSessionHistory` IPC wrapper
- `src-tauri/src/commands.rs` — add `sessions_clear` Tauri command; register in `lib.rs`
- `src-tauri/capabilities/default.json` — allow the new command
