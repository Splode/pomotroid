## Why

The "Reset to Defaults" button currently lives inside the Timer section, but the underlying `settings_reset_defaults` command resets all 26 settings globally — timers, appearance, shortcuts, system, language, and more. This scope mismatch creates a misleading UI: a user in the Timer section reasonably expects the button to reset only timer settings. Moving it to the About section (where administrative/meta actions already live) and adding inline two-step confirmation prevents accidental full resets and accurately represents the action's scope.

## What Changes

- Remove the "Reset to Defaults" button and its handler from `TimerSection.svelte`
- Add a "Reset All Settings" row to `AboutSection.svelte` with a two-step inline confirmation (first click shows "Are you sure? [Cancel] [Reset]", second click fires the reset)
- No backend changes — `settings_reset_defaults` command is unchanged

## Capabilities

### New Capabilities

_(none — this is a UI reorganization, no new capability is introduced)_

### Modified Capabilities

- `settings`: The reset-to-defaults interaction moves from the Timer section to the About section; the confirmation pattern changes from no-confirmation to inline two-step.

## Impact

- `src/lib/components/settings/sections/TimerSection.svelte`: remove `resetSettings` import, `handleReset()` function, and reset button row
- `src/lib/components/settings/sections/AboutSection.svelte`: add `resetSettings` import, `$state` for confirmation mode, inline two-step reset row
- `src/lib/ipc/index.ts`: no changes (wrapper already exists)
- `src-tauri/`: no changes
- Localization: new i18n keys needed for reset row label and confirmation prompt
