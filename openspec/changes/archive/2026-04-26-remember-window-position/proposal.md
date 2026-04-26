## Why

Pomotroid always opens at the same fixed position, ignoring where it was last closed. On multi-monitor setups it invariably reopens on the primary display, which is disruptive — especially when _Always on Top_ is enabled and the user habitually parks the window on a secondary screen.

## What Changes

- When the main window moves or is resized, its position and size are saved to the settings database.
- On startup the saved position is restored, placing the window on the correct monitor at the correct coordinates.
- Before restoring, the saved position is validated against the current display layout. If the target monitor is no longer available (disconnected or resolution changed such that the position is off-screen), the position is discarded and the window opens at the OS default.

## Capabilities

### New Capabilities

- `window-position`: Persist and restore the main window's position and size across restarts, with display-change invalidation.

### Modified Capabilities

- `settings`: Four new settings fields (`window_x`, `window_y`, `window_width`, `window_height`) — position stored as signed integers (negative on non-primary monitors), size as unsigned integers; no schema-version migration required since they are optional and safe to omit on first run.

## Impact

- **`src-tauri/src/settings/mod.rs`** — four new optional fields (`window_x: Option<i32>`, `window_y: Option<i32>`, `window_width: Option<u32>`, `window_height: Option<u32>`) on `Settings`; loaded from DB, default `None`.
- **`src-tauri/src/settings/defaults.rs`** — no default entry needed (absence → `None` → use OS default).
- **`src-tauri/src/lib.rs`** — on setup: restore position if valid; register `WindowEvent::Moved` and `WindowEvent::Resized` handlers on the main window to persist changes.
- **`src-tauri/src/db/`** — no migration needed; `save_setting` upsert handles first write; missing keys are treated as `None`.
- No frontend changes required.
