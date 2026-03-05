## 1. Extend TrayState with menu item handles

- [x] 1.1 Define `TrayMenuItems` struct in `tray/mod.rs` holding `MenuItem<tauri::Wry>` fields for `toggle`, `skip`, and `reset_round`
- [x] 1.2 Add `menu_items: Mutex<Option<TrayMenuItems>>` field to `TrayState`
- [x] 1.3 Update `TrayState::new()` to initialize `menu_items` as `Mutex::new(None)`

## 2. Update `create_tray()` to build the new menu

- [x] 2.1 Create the three new `MenuItem` instances with ids `"toggle"` (label "Start"), `"skip"` (label "Skip", disabled), `"reset-round"` (label "Reset Round", disabled)
- [x] 2.2 Add a `PredefinedMenuItem::separator()` between the timer controls and the existing Show/Exit items
- [x] 2.3 Include all five items (toggle, skip, reset-round, separator, show, exit) in `Menu::with_items`
- [x] 2.4 After a successful `TrayIconBuilder::build`, store the three `MenuItem` handles in `TrayState.menu_items`
- [x] 2.5 Ensure the early-return path (reusing existing icon) does NOT overwrite stored handles

## 3. Add `update_menu_items()` to `tray/mod.rs`

- [x] 3.1 Implement `pub fn update_menu_items(state: &Arc<TrayState>, is_running: bool, is_paused: bool)` that:
  - Sets toggle label to "Pause" when running, "Resume" when paused, "Start" otherwise
  - Sets skip and reset-round enabled to `is_running || is_paused`
- [x] 3.2 Guard with early return when `menu_items` is `None`

## 4. Wire `update_menu_items()` into the timer event listener

- [x] 4.1 Call `tray::update_menu_items(&tray, true, false)` in the `TimerEvent::Started` arm of `listen_events()` in `timer/mod.rs`
- [x] 4.2 Call `tray::update_menu_items(&tray, false, true)` in the `TimerEvent::Paused` arm
- [x] 4.3 Call `tray::update_menu_items(&tray, true, false)` in the `TimerEvent::Resumed` arm
- [x] 4.4 Call `tray::update_menu_items(&tray, false, false)` in the `TimerEvent::Reset` arm

## 5. Handle new menu item events in `on_menu_event`

- [x] 5.1 Add `"toggle"` arm to `on_menu_event`: call `app.try_state::<TimerController>().map(|t| t.toggle())`
- [x] 5.2 Add `"skip"` arm: call `timer.skip()`
- [x] 5.3 Add `"reset-round"` arm: call `timer.restart_round()`

## 6. Verify and test

- [x] 6.1 Run `cargo test` in `src-tauri/` and confirm no regressions
- [x] 6.2 Run `npm run tauri dev` and manually verify toggle label changes (Start → Pause → Resume → Start) across all transitions
- [x] 6.3 Verify Skip and Reset Round are disabled when timer is idle and enabled once started
- [x] 6.4 Verify Reset Round preserves the round type and work round number after clicking
