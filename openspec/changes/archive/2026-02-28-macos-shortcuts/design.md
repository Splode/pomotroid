## Context

Global shortcuts use `tauri-plugin-global-shortcut`, which registers OS-level hotkeys. On macOS, the OS silently drops all global key events unless the app has been granted Accessibility access in System Settings → Privacy & Security → Accessibility. Additionally, the existing defaults (`Control+F1–F4`) map to media/system keys on Mac keyboards and use the wrong modifier convention (`Control` instead of `Command`).

The settings shortcut defaults are seeded via `INSERT OR IGNORE` in `seed_defaults()` (`src-tauri/src/settings/defaults.rs` + `mod.rs`). Since the INSERT is idempotent (first-write wins), macOS-specific values can be injected before the cross-platform defaults are applied without any schema changes.

## Goals / Non-Goals

**Goals:**

- Seed `Command+Shift+1–4` as default shortcuts on macOS for new installs
- Expose a Rust IPC command that returns macOS Accessibility trust status
- Show a dismissing notice in ShortcutsSection when Accessibility is not granted
- Re-check trust on settings window focus so the notice disappears automatically once access is granted

**Non-Goals:**

- Automatically requesting Accessibility permission (macOS doesn't allow programmatic prompts)
- Changing shortcuts for existing macOS installs that have already saved preferences
- Modifying shortcut behavior on Windows or Linux

## Decisions

### Decision 1: Platform-aware seeding via pre-insertion, not branched DEFAULTS

**Chosen:** Insert macOS shortcut rows first (before the main `DEFAULTS` loop) using `#[cfg(target_os = "macos")]`. Since `seed_defaults` uses `INSERT OR IGNORE`, these rows win on first launch and the general defaults are skipped for those keys.

**Alternative considered:** Maintain separate `DEFAULTS` and `MACOS_DEFAULTS` const arrays. Rejected — more code duplication, still needs the same `#[cfg]` guard, less clear intent.

**Alternative considered:** Runtime `cfg!()` macro inside a single loop. Rejected — const arrays can't be conditionally sliced cleanly at runtime; compile-time `#[cfg]` is the idiomatic Rust approach.

### Decision 2: `AXIsProcessTrusted` via `extern "C"` linkage

**Chosen:** Declare `AXIsProcessTrusted()` via `extern "C"` with `#[link(name = "ApplicationServices", kind = "framework")]`. Returns `bool` directly, no args. Wrapped in a `#[cfg(target_os = "macos")]` block; other platforms return `true` unconditionally.

**Alternative considered:** `accessibility` crate. Rejected — adds a dependency for a single function call that's trivially bindable.

**Alternative considered:** Running a shell command to check. Rejected — fragile, slow, wrong abstraction.

### Decision 3: Re-check on window focus, not polling

**Chosen:** In `ShortcutsSection.svelte`, when `trusted` is `false`, attach a `visibilitychange` / `focus` listener on the settings window that re-calls `accessibilityTrusted()`. Remove the listener once `trusted` becomes `true`.

**Alternative considered:** Periodic `setInterval` poll. Rejected — wastes IPC cycles when settings window isn't even open.

### Decision 4: Notice placement — top of ShortcutsSection, macOS-only

The notice renders conditionally: only on macOS (detected via the existing `isMac` platform utility) and only when `trusted === false`. It shows above the shortcut rows with a direct deep-link to System Settings using `tauri-plugin-opener`.

## Risks / Trade-offs

- **AXIsProcessTrusted linkage on non-macOS builds**: The `extern "C"` block is `#[cfg(target_os = "macos")]` so it compiles out entirely on Windows/Linux. Linux and Windows CI builds are unaffected.
- **Existing macOS users with Ctrl+F1–F4 saved**: The `INSERT OR IGNORE` seeding only applies on first launch. Existing users who already have rows in the DB keep their current (broken) shortcuts. They can manually update them in Settings → Shortcuts. This is intentional — we don't clobber saved preferences.
- **macOS Sequoia permission UX**: Apple has moved some privacy panes in recent macOS versions. The deep-link URL `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility` works on Ventura/Sonoma/Sequoia but Apple could change it. Low risk for now.
