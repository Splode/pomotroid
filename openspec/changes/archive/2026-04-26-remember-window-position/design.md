## Context

The main window is declared in `tauri.conf.json` with a fixed size and no position, so the OS places it at its default location on startup. Tauri 2 exposes `WebviewWindow::set_position()` and `WebviewWindow::outer_position()` for reading and writing window coordinates, and `AppHandle::available_monitors()` for enumerating display geometry. The settings database already supports arbitrary key/value string pairs via `save_setting` / `get_setting`, so no schema migration is needed — absent keys are treated as "no saved position."

## Goals / Non-Goals

**Goals:**
- Restore the main window to its last known position and size on every launch.
- Validate the saved position against the live display layout before applying it; fall back to the OS default if the position is no longer on any screen.
- Persist position/size changes as they happen (move and resize events), not only on close.
- Keep the implementation entirely in Rust/Tauri — no frontend changes.

**Non-Goals:**
- Remembering positions for the Settings or Statistics child windows (they are transient and short-lived).
- Smooth animated repositioning.
- Per-display profile (remembering different positions per monitor configuration).
- Saving position when the window is hidden to tray (position is already saved continuously via events).

## Decisions

**Store x/y as separate integer keys in the settings DB.**
`window_x`, `window_y`, `window_width`, `window_height` as four string-encoded `i32`/`u32` values. Alternatives considered:
- A single JSON blob (`{"x":…}`) — more compact but requires a JSON dependency or manual parsing; overkill for four scalars.
- New dedicated table — unnecessary; the existing settings KV table is the canonical store for all persistent config.

**Save width and height alongside position.**
Users resize the window (it is resizable). Restoring only position but not size would be surprising. Saving all four values together keeps the restored state complete.

**Validate with a 1 px intersection check.**
Before restoring, compute whether the saved `(x, y, w, h)` rectangle intersects any available monitor's work area by at least 1 pixel. If it does not, discard the saved values and let the OS place the window. This handles: disconnected monitors, resolution decreases, and display arrangement changes.
Alternative considered: check only that the top-left corner is on a monitor — but a window could have its top-left on-screen while being mostly off-screen after a resolution change.

**Persist on `Moved` and `Resized` window events, not only on `CloseRequested`.**
Saving only at close is fragile — a crash would lose the last position. The `on_window_event` handler already exists on the main window (for tray-hide logic); the same closure handles `Moved` and `Resized`. A simple debounce is not used (no async executor in the handler), so every event writes to the DB — at low frequency (user-driven drags) this is acceptable.

**`window_width` / `window_height` stored as `u32`; `window_x` / `window_y` as `i32`.**
Coordinates can be negative on multi-monitor setups where the primary is not the leftmost display; dimensions cannot be negative.

## Risks / Trade-offs

**Risk: Rapid move/resize events write to SQLite on every event.**
→ Mitigation: Window move/resize events are user-driven and low frequency. SQLite upsert is fast (~0.1 ms). Acceptable without debouncing for now.

**Risk: Saved position becomes stale after OS font-scaling or DPI changes without a monitor topology change.**
→ Mitigation: The intersection check uses physical pixel coordinates from Tauri, which are DPI-aware. A DPI change that moves the window off-screen will be caught. Minor DPI changes that keep the window on-screen may shift it slightly — acceptable since the window will still be visible.

**Risk: On some Linux window managers `Moved` events fire before the window is fully decorated, reporting incorrect coordinates.**
→ Mitigation: Apply the saved position after the window is created (in `setup`), not in response to a `Moved` event from a previous session.

## Migration Plan

No DB migration required. `save_setting` uses `INSERT … ON CONFLICT DO UPDATE`, so first write creates the row. `get_setting` returns `None` for absent keys, which maps to "no saved position → use OS default." Existing users experience no change on first launch after upgrade.
