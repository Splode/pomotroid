## 1. macOS Default Shortcuts

- [x] 1.1 In `src-tauri/src/settings/defaults.rs`, remove the four `shortcut_*` entries from the `DEFAULTS` const (they will be seeded by platform-specific logic)
- [x] 1.2 In `src-tauri/src/settings/mod.rs`, update `seed_defaults()` to insert macOS shortcut defaults (`Super+Shift+1–4`) via `#[cfg(target_os = "macos")]` before falling through to the main `DEFAULTS` loop
- [x] 1.3 Add the `shortcut_*` keys back to the non-macOS path (inside `#[cfg(not(target_os = "macos"))]`) so Windows/Linux still seed `Control+F1–F4`
- [x] 1.4 Update `Settings::default()` in `mod.rs` to use `Super+Shift+1` etc. on macOS (used as fallback when DB load fails)
- [x] 1.5 Update the `seed_defaults` unit tests to assert macOS defaults on macOS and Ctrl+F-key defaults otherwise (or mark them `#[cfg]`)

## 2. Accessibility Trusted IPC Command

- [x] 2.1 In `src-tauri/src/commands.rs`, add `accessibility_trusted()` command: on macOS, declare `extern "C" { fn AXIsProcessTrusted() -> bool; }` with `#[link(name = "ApplicationServices", kind = "framework")]` inside a `#[cfg(target_os = "macos")]` block and call it; on other platforms return `true`
- [x] 2.2 Register `accessibility_trusted` in the `invoke_handler` in `src-tauri/src/lib.rs`
- [x] 2.3 Add `accessibilityTrusted(): Promise<boolean>` wrapper to `src/lib/ipc/index.ts`

## 3. Shortcuts Section Notice (Frontend)

- [x] 3.1 Add i18n keys to all 7 locale files (`en`, `es`, `fr`, `de`, `ja`, `zh`, `pt`): `shortcuts_accessibility_notice` (explanatory text) and `shortcuts_accessibility_open` (button label)
- [x] 3.2 In `ShortcutsSection.svelte`, add `let trusted = $state(true)` and call `accessibilityTrusted()` in an async init block on mount (macOS only via `isMac`)
- [x] 3.3 Add a `visibilitychange` / window `focus` event listener that re-calls `accessibilityTrusted()` when `trusted` is `false`; unlisten once `trusted` becomes `true`
- [x] 3.4 Render the notice banner above the shortcut rows when `isMac && !trusted`: include the explanatory text and an "Open Settings" button that calls `openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility')` via `tauri-plugin-opener`
- [x] 3.5 Style the notice banner (warning color, subtle border, compact layout)

## 4. Verification

- [x] 4.1 Run `npm run check` — 0 errors
- [x] 4.2 Run `cargo test` in `src-tauri/` — all tests pass
- [x] 4.3 (macOS) Fresh install: confirm shortcuts default to `⌘⇧1–4`
- [x] 4.4 (macOS, no Accessibility) Open Settings → Shortcuts: notice is visible with working "Open Settings" link
- [x] 4.5 (macOS) Grant Accessibility access, return to settings window: notice dismisses automatically
- [x] 4.6 (macOS, Accessibility granted) Confirm `⌘⇧1` toggles the timer globally
