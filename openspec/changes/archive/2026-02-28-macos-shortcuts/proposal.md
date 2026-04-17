## Why

Global keyboard shortcuts are silently non-functional on macOS due to two compounding issues: the OS requires explicit Accessibility permission before any app can receive global key events, and the default shortcuts (`Control+F1–F4`) conflict with macOS media keys and system conventions. Users on macOS have no working shortcuts and no indication of why.

## What Changes

- Seed macOS-specific default shortcuts (`Command+Shift+1–4`) instead of `Control+F1–F4` on first launch
- Add a Rust IPC command that queries macOS Accessibility trust status (`AXIsProcessTrusted`)
- Show a contextual notice in the Shortcuts settings section on macOS when Accessibility access is not granted, with a direct link to open System Settings
- Re-check trust status whenever the settings window regains focus (so the notice dismisses automatically once the user grants access)

## Capabilities

### New Capabilities

- `macos-shortcuts`: macOS-specific shortcut defaults and Accessibility permission detection/guidance

### Modified Capabilities

- `shortcuts`: Default shortcut values are now platform-aware (macOS gets `Command+Shift+1–4`)

## Impact

- `src-tauri/src/settings/defaults.rs` — macOS shortcut default values
- `src-tauri/src/settings/mod.rs` — platform-aware seeding in `seed_defaults()`
- `src-tauri/src/commands.rs` — new `accessibility_trusted()` command (macOS: `AXIsProcessTrusted`; other platforms: returns `true`)
- `src-tauri/src/lib.rs` — register new command
- `src/lib/ipc/index.ts` — `accessibilityTrusted()` TypeScript wrapper
- `src/lib/components/settings/sections/ShortcutsSection.svelte` — permission notice banner, focus re-check
- `messages/*.json` (all 7 locales) — 2–3 new i18n keys for the notice text
