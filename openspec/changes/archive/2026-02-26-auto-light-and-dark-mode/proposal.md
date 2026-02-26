## Why

Pomotroid's theme system is a manual, single-choice picker — users must remember to switch themes when moving between light and dark environments. Adding OS-aware automatic theme switching reduces friction and aligns the app with modern desktop expectations.

## What Changes

- Add a theme mode selector: **Auto**, **Light**, **Dark**
- Add separate light theme and dark theme pickers (both show all available themes)
- In Auto mode, the active theme is determined by the OS `prefers-color-scheme` signal, switching live when the OS changes
- In Light/Dark mode, the user's explicit picker selection is always used regardless of OS
- **BREAKING**: Remove the single `theme` setting field; replace with `theme_mode`, `theme_light`, and `theme_dark`
- Add a DB migration that copies the existing `theme` value to both `theme_light` and `theme_dark` for existing users, and sets `theme_mode` to `"auto"`
- Both the main window and the settings window respond to live OS theme changes

## Capabilities

### New Capabilities

- `theme-mode`: Mode selector (Auto/Light/Dark) controlling how the active theme is resolved from the two pickers and the OS color scheme signal

### Modified Capabilities

- none

## Impact

- **Rust**: `settings/defaults.rs`, `settings/mod.rs`, `db/migrations.rs` — new fields, new migration
- **Frontend**: `src/lib/types.ts`, `src/lib/stores/settings.ts` — new fields on Settings interface
- **Frontend**: `AppearanceSection.svelte` — new UI (mode selector + two pickers replacing single picker)
- **Frontend**: both `src/routes/+page.svelte` and `src/routes/settings/+page.svelte` — startup theme resolution, live `matchMedia` listener
- **Frontend**: new shared `resolveThemeName()` utility
- **No new dependencies** — uses native `window.matchMedia` API
