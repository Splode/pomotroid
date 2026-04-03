## 1. Rust: Settings struct and DB migration

- [x] 1.1 Add 7 local shortcut fields to `Settings` struct in `src-tauri/src/settings/mod.rs` (`local_shortcut_toggle`, `local_shortcut_reset`, `local_shortcut_skip`, `local_shortcut_volume_down`, `local_shortcut_volume_up`, `local_shortcut_mute`, `local_shortcut_fullscreen`)
- [x] 1.2 Add default values for all 7 fields in `src-tauri/src/settings/defaults.rs` (`" "`, `"ArrowLeft"`, `"ArrowRight"`, `"ArrowDown"`, `"ArrowUp"`, `"m"`, `"F11"`)
- [x] 1.3 Add DB key mappings for all 7 fields in the `settings_load` / `settings_save` mapping table in `src-tauri/src/settings/mod.rs`
- [x] 1.4 Add DB migration in `src-tauri/src/db/migrations.rs`: insert 7 new rows into the settings table with default values, increment the schema version constant
- [x] 1.5 Handle all 7 new DB keys in the `settings_set` command match arms in `src-tauri/src/commands.rs`

## 2. TypeScript: Types and IPC

- [x] 2.1 Add 7 new string fields to the `Settings` interface in `src/lib/types.ts` (mirroring the Rust struct field names)
- [x] 2.2 Verify `src/lib/ipc/index.ts` already exports `timerToggle`, `timerReset`, `timerSkip`, `settingsSet` — add any missing wrappers

## 3. Frontend: Keydown handler

- [x] 3.1 Create `src/lib/utils/localShortcuts.ts` — exports a `createLocalShortcutHandler(settings, extraState)` factory that returns a `keydown` event handler. Handler: skips if `event.target` is `INPUT`/`TEXTAREA`/`[contenteditable]`; matches `event.key` against current bindings; calls the appropriate IPC action; calls `event.preventDefault()` for matched keys
- [x] 3.2 Implement volume up/down logic: read current volume from settings, clamp ±0.05, call `settingsSet('volume', newValue)`
- [x] 3.3 Implement mute toggle: if volume > 0, save pre-mute volume in local state and set volume to 0; if volume === 0, restore pre-mute volume (default restore to 0.5 if no prior value)
- [x] 3.4 Implement fullscreen toggle: call `getCurrentWindow().setFullscreen(!isFullscreen)` using `@tauri-apps/api/window`; track `isFullscreen` state locally
- [x] 3.5 Mount the handler in `src/routes/+page.svelte` via `onMount` / `onDestroy`, passing reactive settings and fullscreen state
- [x] 3.6 Mount the same handler in `src/routes/settings/+page.svelte` via `onMount` / `onDestroy`

## 4. Frontend: Settings UI

- [x] 4.1 Add a "Local Shortcuts" subsection to `src/lib/components/settings/sections/ShortcutsSection.svelte` below the existing global shortcuts section
- [x] 4.2 Render one row per local shortcut action (label + key capture input) for all 7 actions
- [x] 4.3 Implement key capture input for local shortcuts: on focus, listen for next `keydown`; record `event.key` (ignore modifier-only keys: Shift, Control, Alt, Meta); call `settingsSet` with the new value; blur the input
- [x] 4.4 Display the current binding value in the input (read from settings store, formatted for readability: `" "` → `Space`, `"ArrowLeft"` → `←`, etc.)
- [x] 4.5 Style local shortcut rows consistently with global shortcut rows (same input sizing, alignment, disabled-state handling)

## 5. Settings reset integration

- [x] 5.1 Verify that the existing "Reset All Settings" path in the frontend calls the Rust `settings_reset` command (or equivalent) which re-runs defaults — confirm the 7 new defaults are included automatically by the existing mechanism
- [x] 5.2 Manually test: set a custom local shortcut binding, trigger Reset All Settings, confirm bindings revert to defaults

## 6. Manual verification

- [x] 6.1 Launch app, confirm all 7 default shortcuts work in the main window (Space toggles, arrows adjust volume/rounds, M mutes, F11 fullscreens)
- [x] 6.2 Confirm shortcuts do not fire while a shortcut capture input is focused in the Settings window
- [x] 6.3 Rebind pause/resume to a different key in Settings, confirm old key no longer works and new key does
- [x] 6.4 Confirm bindings survive app restart (loaded from DB)
- [x] 6.5 Run `npm run check` with no type errors
