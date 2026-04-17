## Context

The tray icon currently shows only "Show" and "Exit" menu items. `TrayState` holds the icon handle, theme colors, and countdown mode, but no menu item handles — they are created and immediately dropped inside `create_tray()`.

Timer state transitions flow through `listen_events()` in `timer/mod.rs`, which already holds `Arc<TrayState>` and calls `tray::update_icon()` on every tick. Extending this thread to also update menu labels is a natural fit.

The `TimerController` public API already exposes all required operations: `toggle()`, `skip()`, and `restart_round()`. No new commands or IPC surface is needed.

## Goals / Non-Goals

**Goals:**

- Add Toggle (Start/Pause/Resume), Skip, and Reset Round items to the tray context menu.
- Keep item labels and enabled states in sync with timer state transitions.
- Dispatch timer commands from `on_menu_event` via `app.try_state::<TimerController>()`.

**Non-Goals:**

- Displaying the remaining time or round type in the menu (tooltip already covers visual state).
- Keyboard shortcuts for tray menu items.
- Tray controls when the tray icon is not visible (tray is optional; controls exist only when the icon exists).

## Decisions

### Decision 1: Store `MenuItem` handles in `TrayState`

`TrayState` is the shared handle already passed into the timer event listener. Storing `MenuItem` handles there lets `update_menu_items()` be called from any thread without additional wiring.

**Alternative considered**: Store handles in a separate `TrayMenuState` struct. Rejected — adds indirection with no benefit; `TrayState` is already the single place for tray-related Tauri handles.

### Decision 2: One toggle item with dynamic label rather than three separate items

A single item whose label changes between "Start", "Pause", and "Resume" keeps the menu compact and matches how the play/pause button in the main window works.

**Alternative considered**: Three separate items, one visible at a time. Rejected — Tauri `MenuItem` doesn't support `set_visible()` reliably across platforms; dynamic text on a single item is the documented pattern.

### Decision 3: Update menu items on state-change events only (not on `Tick`)

`TimerEvent::Tick` fires every second. Calling `set_text()` or `set_enabled()` on every tick would be wasteful since labels only change on four events: `Started`, `Paused`, `Resumed`, and `Reset`. `TimerEvent::Complete` transitions immediately to the next round and resets to idle, so the `Reset` arm handles that path automatically.

### Decision 4: `restart_round()` for Reset Round, not `reset()`

`restart_round()` zeroes elapsed time for the current round without touching sequence position, which is the most useful tray action (redo this round). Full sequence reset via `reset()` is disruptive and better suited to a deliberate UI interaction in the main window.

### Decision 5: Disable Skip and Reset Round when idle

From idle, `skip()` would advance the sequence without completing a real round, and `restart_round()` is a no-op. Disabling both avoids confusing behavior.

## Risks / Trade-offs

- **`MenuItem::set_text()` thread safety**: Tauri 2 `MenuItem` handles are `Send + Sync`; the pattern mirrors how `update_icon()` already mutates tray state from the `timer-events` thread. No known issues.
- **Menu not shown when tray is disabled**: `TrayState.menu_items` will be `None` when the tray icon has never been created or has been destroyed. `update_menu_items()` is a no-op in that case — safe.
- **`create_tray()` reuse path**: On subsequent calls (show after hide), the existing icon is made visible again and the function returns early. Menu item handles must be stored on the _first_ call only; subsequent early-returns leave existing handles intact.
