## 1. Rust — Settings

- [x] 1.1 In `src-tauri/src/settings/mod.rs`, add `pub window_x: Option<i32>`, `pub window_y: Option<i32>`, `pub window_width: Option<u32>`, and `pub window_height: Option<u32>` to the `Settings` struct
- [x] 1.2 In `Settings::default()`, set all four fields to `None`
- [x] 1.3 In `load()`, map the four DB keys to their fields using `parse_opt_i32` / `parse_opt_u32` helpers (return `None` when the key is absent or unparseable)

## 2. Rust — Startup: Restore Position

- [x] 2.1 In `src-tauri/src/lib.rs`, after the main window is obtained, read `initial_settings.window_x/y/width/height`
- [x] 2.2 If all four values are `Some`, call `app.available_monitors()` and check whether the saved `(x, y, width, height)` rectangle intersects at least one monitor's position+size by ≥ 1 px
- [x] 2.3 If the intersection check passes, call `main_window.set_position(PhysicalPosition::new(x, y))` and `main_window.set_size(PhysicalSize::new(width, height))`
- [x] 2.4 If the intersection check fails (or any value is `None`), skip — let the OS place the window at its default position

## 3. Rust — Persist on Move and Resize

- [x] 3.1 In the existing `main_window.on_window_event` closure in `lib.rs`, handle `WindowEvent::Moved { x, y, .. }` by calling `settings::save_setting` for `window_x` and `window_y`
- [x] 3.2 In the same closure, handle `WindowEvent::Resized { width, height, .. }` by calling `settings::save_setting` for `window_width` and `window_height`
- [x] 3.3 Both handlers should also update `window_x`/`window_y` on resize (and vice versa) if the window manager moves the window as part of a resize — capture `main_window.outer_position()` in the `Resized` handler to get the current position alongside the new size

## 4. Verify

- [x] 4.1 Move the window, close and relaunch — window opens at the same position
- [x] 4.2 Resize the window, close and relaunch — window opens at the same size
- [ ] 4.3 Move the window to a second monitor, close and relaunch — window opens on the second monitor
- [ ] 4.4 Move the window to a second monitor, disconnect it (or change display config), relaunch — window falls back to OS default position
- [ ] 4.5 Move the window, trigger Reset All Settings — window stays in place for the current session; on next launch it opens at the OS default position and size
- [ ] 4.6 Run `cargo test` in `src-tauri/` — all tests pass
