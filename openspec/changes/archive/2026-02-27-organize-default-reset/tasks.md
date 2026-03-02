## 1. Timer Section Cleanup

- [x] 1.1 In `TimerSection.svelte`, remove `resetSettings` from the `'$lib/ipc'` import
- [x] 1.2 Remove the `handleReset()` async function
- [x] 1.3 Remove the `.reset-row` div and its `reset-btn` button from the template
- [x] 1.4 Remove the `.reset-row` and `.reset-btn` CSS rules (and `.reset-btn:hover`)

## 2. Localization Keys

- [x] 2.1 In `messages/en.json`, add `about_reset_all` ("Reset All Settings") and `about_reset_confirm` ("Are you sure? This will reset all settings to defaults.")
- [x] 2.2 Add the same two keys to `messages/es.json`, `messages/fr.json`, `messages/de.json`, `messages/ja.json` (use equivalent translations)

## 3. About Section — Reset UI

- [x] 3.1 In `AboutSection.svelte`, add `resetSettings` to the `'$lib/ipc'` import
- [x] 3.2 Add `import { settings } from '$lib/stores/settings'`
- [x] 3.3 Declare `let confirming = $state(false)`
- [x] 3.4 Add a `handleReset()` async function: calls `resetSettings()`, sets `settings.set(updated)`, resets `confirming = false`
- [x] 3.5 Add the reset row to the template below the links group: when `!confirming` show a single button calling `() => (confirming = true)`; when `confirming` show the confirmation prompt with Cancel (`() => (confirming = false)`) and Reset (`handleReset`) buttons
- [x] 3.6 Style the reset row: use `.link-row` base pattern; initial button muted (`--color-foreground-darker`); confirmation Reset button uses `--color-accent` or a danger-adjacent color; Cancel is neutral

## 4. Audio Reset (oversight fix)

- [x] 4.1 In `settings_reset_defaults`, walk `{app_data_dir}/audio/` and delete files matching the three custom stems
- [x] 4.2 Clear all three slots in `AudioManager`'s in-memory `custom_paths` on reset
- [x] 4.3 In `NotificationsSection.svelte`, extract `refreshAudioInfo()` and subscribe to `onSettingsChanged` so audio state refreshes after reset without restart

## 5. Verification

- [x] 5.1 Run `npm run check` — no type errors
- [x] 5.2 Run `cargo check` — no errors
- [x] 5.3 Confirm no reset button in Timer section
- [x] 5.4 In About section: click "Reset All Settings" — row switches to confirmation state
- [x] 5.5 Click Cancel — row returns to initial button, no settings changed
- [x] 5.6 Click "Reset All Settings" then Reset — all settings revert to defaults
