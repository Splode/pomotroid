## Why

Pomotroid already supports global shortcuts that work system-wide, but lacks any keyboard shortcuts that activate while the app window is focused. Users who keep the app visible on screen have no way to control the timer, volume, or round state with the keyboard — they must reach for the mouse even for common actions like pause/resume or volume adjustment.

## What Changes

- Introduce a new local shortcuts system: a set of keyboard shortcuts active only while a Pomotroid window has focus
- Default bindings: Space (pause/resume), Left Arrow (reset current round), Right Arrow (skip round), Down Arrow (volume down), Up Arrow (volume up), M (mute toggle), F11 (fullscreen toggle)
- All local shortcuts are re-mappable in Settings → Shortcuts alongside global shortcuts
- "Reset All Settings" resets local shortcut bindings to defaults along with all other settings
- Local shortcuts are always active when the app is focused (no separate enable/disable toggle — unlike global shortcuts)

## Capabilities

### New Capabilities
- `local-shortcuts`: Keyboard shortcuts active while the application window is focused, covering pause/resume, round reset, round skip, volume up/down, mute, and fullscreen toggle — all re-mappable via Settings

### Modified Capabilities
- `shortcuts`: The existing global shortcuts spec must be extended: Settings → Shortcuts section now displays both global and local shortcut bindings; "Reset All Settings" must also reset local shortcut bindings to defaults

## Impact

- **Frontend**: `ShortcutsSection.svelte` expanded to show local shortcut bindings below global ones; keydown event listeners added on main and settings window roots; IPC wrappers for new local-shortcut commands
- **Backend**: New `Settings` fields for local shortcut key bindings; DB migration adds keys (e.g. `local_shortcut_toggle`, `local_shortcut_reset`, etc.); `settings/defaults.rs` updated; `commands.rs` wires up shortcut actions; `settings_reset` handler also resets local shortcut defaults
- **Types**: `src/lib/types.ts` updated to mirror new `Settings` fields
- **Capabilities file**: No new Tauri plugin permissions required (keydown handling is pure frontend via Svelte)
