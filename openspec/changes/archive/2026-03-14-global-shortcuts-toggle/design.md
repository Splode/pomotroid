## Context

Global shortcuts are registered unconditionally at startup via `shortcuts::register_all()` in `lib.rs`. The function is also called reactively from `settings_set` whenever a shortcut key changes. A `shortcuts::unregister_all()` helper already exists but is never called conditionally. The new `global_shortcuts_enabled` setting gates this registration — when false, `register_all` becomes a no-op and `unregister_all` is called instead. The individual shortcut key values are preserved regardless of the toggle state so users don't lose their custom bindings when temporarily disabling shortcuts.

## Goals / Non-Goals

**Goals:**
- Single boolean setting controls whether any global shortcuts are active.
- Toggling off unregisters all shortcuts immediately (no restart required).
- Toggling on registers shortcuts immediately using current key bindings.
- Default is `false` (opt-in), matching the WebSocket server precedent.
- Individual shortcut fields remain editable while disabled (values persist, just not active).

**Non-Goals:**
- Per-shortcut enable/disable (all or nothing).
- Changing the individual shortcut key defaults.
- Any change to shortcut behaviour when enabled.

## Decisions

**Gate registration inside `register_all`** rather than at every call site. `register_all` already owns the full register/unregister cycle; checking `settings.global_shortcuts_enabled` there means no call-site changes are needed in `lib.rs` or `commands.rs` beyond ensuring `global_shortcuts_enabled` changes also trigger a `register_all` call.

**Extend `settings_set` trigger list** to include `"global_shortcuts_enabled"` alongside the existing `shortcut_*` keys. This reuses the existing reactive path without new IPC surface.

**Default `false` for new installs via DB migration.** The migration inserts `('global_shortcuts_enabled', 'false')` with `INSERT OR IGNORE`, so existing users who already have the row (from a future install) are unaffected. For existing installs upgrading, the migration adds the row with the default.

**Visual treatment in ShortcutsSection**: the four shortcut input rows are rendered with reduced opacity and `pointer-events: none` when disabled. They remain in the DOM (no conditional rendering) so layout doesn't shift. This is purely cosmetic — changes to those fields while disabled still persist to the DB and take effect when shortcuts are re-enabled.

## Risks / Trade-offs

**Breaking change: existing users' shortcuts are disabled after upgrade.** The migration sets `global_shortcuts_enabled = false` for all installs. This is an intentional opt-in reset — global shortcuts now behave consistently with the WebSocket server (off by default). The toggle is at the top of Settings → Shortcuts and easy to re-enable. The breaking change will be noted in the changelog.

**Risk: `unregister_all` fails silently.** The plugin returns a `Result` that is currently discarded. A failure leaves stale shortcuts registered.
→ Mitigation: Log a warning on error, consistent with existing `register_all` error handling. No user-visible change needed.

## Migration Plan

Add a new migration version in `db/migrations.rs`:
```sql
INSERT OR IGNORE INTO settings (key, value) VALUES ('global_shortcuts_enabled', 'false');
```

No rollback strategy is needed — the row can simply be deleted to restore prior behaviour, and the app falls back gracefully (the settings loader will use the struct default `false` if the key is absent).
