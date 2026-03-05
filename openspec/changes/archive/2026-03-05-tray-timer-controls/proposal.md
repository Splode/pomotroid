## Why

The system tray is the app's primary interface when the main window is hidden, yet it currently offers no way to control the timer — users must show the window just to start, pause, or skip a round. Adding controls directly to the tray menu removes that friction.

## What Changes

- The tray context menu gains three new items: a dynamic **toggle** item ("Start" / "Pause" / "Resume"), a **Skip** item, and a **Reset Round** item.
- Toggle label and the enabled state of Skip and Reset Round update automatically as the timer transitions between idle, running, and paused states.
- Skip and Reset Round are disabled while the timer is idle; all three items are enabled when the timer is running or paused.
- Reset Round resets only the current round's elapsed time (`restart_round()`); it does not change the round type or position in the work/break cycle.
- `TrayState` is extended to store `MenuItem` handles so labels and enabled states can be updated from the timer event listener thread.

## Capabilities

### New Capabilities

- `tray-timer-controls`: Timer control actions (toggle, skip, reset round) accessible directly from the system tray context menu, with dynamic item labels and enabled states reflecting live timer state.

### Modified Capabilities

*(none — no existing spec-level behavior changes)*

## Impact

- **`src-tauri/src/tray/mod.rs`**: `TrayState` gains a `menu_items` field; `create_tray()` stores `MenuItem` handles; new `update_menu_items()` function updates labels and enabled states.
- **`src-tauri/src/timer/mod.rs`**: `listen_events()` calls `tray::update_menu_items()` on `Started`, `Paused`, `Resumed`, and `Reset` events.
- No frontend changes, no new IPC commands, no dependency additions.
