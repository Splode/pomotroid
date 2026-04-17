## Why

Global shortcuts in Pomotroid use system-wide key combinations (e.g. `Control+F1`–`F4`) that conflict with other applications by default, and there is currently no way to disable them without reassigning every individual shortcut to an obscure combination. Like the WebSocket server — another system-level feature that can interfere with the user's environment — global shortcuts should be opt-in and off by default.

## What Changes

- Add a `global_shortcuts_enabled` boolean setting (default: `false`) stored in SQLite.
- When disabled, all four global shortcuts are unregistered and not re-registered until the user enables them.
- When enabled, shortcuts behave exactly as today.
- A toggle is added to Settings → Shortcuts, above the individual shortcut fields, consistent with how the WebSocket toggle sits above the port field.
- Global shortcuts are **off by default** on all platforms for both new and existing installs. **BREAKING**: existing users will find their shortcuts disabled after upgrading and must re-enable them in Settings → Shortcuts.
- Individual shortcut key bindings are preserved regardless of the toggle state.

## Capabilities

### New Capabilities

- none

### Modified Capabilities

- `shortcuts`: Add requirement that global shortcuts can be enabled or disabled as a unit via a single toggle; disabled state unregisters all shortcuts immediately and persists across restarts.
- `settings`: Add `global_shortcuts_enabled` setting key with default value `false`.

## Impact

- **Rust**: `src-tauri/src/shortcuts/mod.rs` — `register_all` must check `settings.global_shortcuts_enabled` and skip registration (calling `unregister_all`) when false.
- **Rust**: `src-tauri/src/settings/mod.rs` — add `global_shortcuts_enabled: bool` field and `"global_shortcuts_enabled"` → `"false"` default.
- **Rust**: `src-tauri/src/db/migrations.rs` — migration to add `global_shortcuts_enabled` row.
- **Rust**: `src-tauri/src/commands.rs` — `settings_set` already calls `shortcuts::register_all` for shortcut key changes; extend this to also trigger on `global_shortcuts_enabled`.
- **Frontend**: `src/lib/types.ts` — add `global_shortcuts_enabled: boolean` to `Settings`.
- **Frontend**: `src/lib/components/settings/sections/ShortcutsSection.svelte` — add enable/disable toggle above the shortcut fields, disable the fields (visual only — they still save) when the toggle is off.
