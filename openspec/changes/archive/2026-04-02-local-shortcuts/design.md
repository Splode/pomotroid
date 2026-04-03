## Context

Pomotroid has an existing global shortcuts system that registers OS-level hotkeys via `tauri-plugin-global-shortcut`. Those shortcuts are stored in SQLite, exposed through the `Settings` struct, and editable in the Settings → Shortcuts section (`ShortcutsSection.svelte`).

Local shortcuts are a different, complementary mechanism: they fire only while the app window has focus, need no OS-level registration, and are handled entirely in the frontend via standard `keydown` event listeners. The actions they invoke (timer toggle, skip, reset, volume change, mute, fullscreen) are already available through existing IPC commands.

## Goals / Non-Goals

**Goals:**
- 7 default local shortcuts active whenever any Pomotroid window has focus
- All 7 bindings are user-configurable via Settings → Shortcuts
- Bindings persist in SQLite alongside global shortcut bindings
- "Reset All Settings" resets local bindings to defaults
- Shortcut bindings update live: changing a binding takes effect without restart
- No separate enable/disable toggle (local shortcuts are always active while focused)

**Non-Goals:**
- Modifier-key chording for local shortcuts (single keys only, consistent with defaults like Space, Arrow keys, M, F11)
- Per-window shortcut overrides (same bindings apply to both main and settings windows)
- Local shortcut conflict detection with global shortcuts
- Disabling individual shortcuts (user can leave a binding blank/unbound to effectively disable)

## Decisions

### 1. Pure frontend keydown listeners — no Rust involvement for dispatch

**Decision**: Handle keydown in Svelte via `document.addEventListener('keydown', ...)`. When a matching key is pressed, call the existing IPC functions (`timerToggle()`, `timerSkip()`, `timerReset()`, `volumeSet()`, `settingsSet()`, etc.) directly from the listener.

**Why over Rust-side handling**: The Tauri `tauri-plugin-global-shortcut` plugin is the right tool for OS-level shortcuts, but for focus-scoped shortcuts the browser's own keyboard event model is simpler and sufficient. There is no IPC round-trip needed just to dispatch — the frontend already has all the IPC wrappers it needs.

**Alternative**: Register shortcuts in Rust using `Window::on_window_event` focus guards. Rejected: adds complexity, introduces a Rust→frontend callback path for no benefit.

### 2. Store bindings in Settings struct as plain key strings

**Decision**: Add 7 new fields to the `Settings` struct (e.g. `local_shortcut_toggle: String`) with DB keys like `local_shortcut_toggle`. Each value is a key string (e.g. `" "`, `"ArrowLeft"`, `"F11"`, `"m"`) matching the browser `KeyboardEvent.key` property.

**Why `KeyboardEvent.key` over `KeyboardEvent.code`**: `key` is layout-aware and maps to what the user expects ("Space", not "Space" vs "KeySpace"). Single-character keys like `m` are unambiguous. Arrow keys and function keys have stable `key` names across platforms.

**Why not a JSON blob**: The existing settings pattern stores flat key/value strings. Keeping the same shape makes settings reset, migration, and the `settings_set` IPC flow consistent with the rest of the codebase.

### 3. Key capture UI reuses global shortcut recorder pattern

**Decision**: The local shortcuts section in `ShortcutsSection.svelte` uses the same single-key recorder input pattern already used for global shortcuts — click to focus, press a key, the binding is saved. Difference: local shortcuts record the raw `KeyboardEvent.key` value rather than a Tauri accelerator string.

**Why**: The UX is already proven and familiar. Minor adaptation needed: global shortcut recording listens for a Tauri accelerator format; local shortcut recording listens for `KeyboardEvent.key` directly.

### 4. Listener mounted on `document` in `+page.svelte` (main window) and `settings/+page.svelte`

**Decision**: The keydown handler lives in both page root components, added in `onMount` and removed in `onDestroy`. It reads the current local shortcut settings from the reactive settings store.

**Why both pages**: Both windows can be focused. The settings window should also respond to shortcuts (e.g., user adjusts volume while in settings).

**Guard**: The handler should skip if the focused element is an `<input>` or `<textarea>` to avoid interfering with typing in shortcut capture fields.

### 5. Fullscreen via Tauri `appWindow.setFullscreen()` toggle

**Decision**: F11 calls `getCurrentWindow().setFullscreen(!isFullscreen)`, reading current fullscreen state from a local reactive variable updated via `Window.onResized` or `Window.isFullscreen()`.

**Why**: Tauri exposes `setFullscreen` on the `WebviewWindow` API. This is the correct cross-platform way; native OS F11 handling is bypassed in Tauri's decoration-free window mode anyway.

### 6. Volume up/down as fixed increments (±5%)

**Decision**: Each Up/Down Arrow press adjusts volume by 0.05 (5%) clamped to [0.0, 1.0]. Volume is then saved with `settingsSet('volume', ...)`.

**Why 5%**: Consistent with typical media application increments. Larger steps feel coarse; smaller steps require too many presses.

## Risks / Trade-offs

- **Key conflicts with browser defaults** → Arrow keys may scroll the page in some contexts. Calling `event.preventDefault()` in the handler mitigates this. Must be careful not to swallow keys when an input is focused.
- **Settings window shortcut duplication** → Both windows mount listeners, so both will fire if both have focus simultaneously (impossible in practice; OS enforces single focus). Low risk.
- **KeyboardEvent.key normalization** → On some platforms/locales, single-character keys may differ in case. Storing and comparing in lowercase for letter keys avoids mismatches.
- **Binding an already-used key** → No conflict detection in v1. A key bound to both a local and global shortcut will fire both if the app is focused. Acceptable for now; conflict UI is explicitly a non-goal.

## Migration Plan

1. Add DB migration (increment schema version, insert 7 new key/value rows with defaults)
2. Add 7 fields to `Settings` struct and `defaults.rs`
3. Update `types.ts` to match
4. Wire keydown listeners in both page components, reading from settings store
5. Expand `ShortcutsSection.svelte` with local shortcut capture rows
6. Verify "Reset All Settings" path resets the 7 new keys to defaults (handled by existing `settings_reset` command which re-runs `defaults.rs`)

Rollback: remove migration (or let it be — extra DB rows are harmless), revert frontend changes. No data loss risk.

## Open Questions

- Should the Space bar shortcut be suppressed on the settings page to avoid accidental timer toggle while a user types? Likely yes — the `input`/`textarea` focus guard handles this.
- Should we display the current key binding next to each action in the main timer window as a tooltip? Out of scope for v1 but worth noting.
