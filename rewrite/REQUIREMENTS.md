# Requirements: Pomotroid Rewrite

---

## 1. Functional Requirements

### FR-TIMER — Timer Core

| ID | Requirement | Source |
|---|---|---|
| FR-TIMER-01 | The app shall support three round types: Work (Focus), Short Break, and Long Break. | Existing |
| FR-TIMER-02 | Each round type shall have a configurable duration in whole minutes. | Existing |
| FR-TIMER-03 | Work duration range: 1–90 minutes; default 25 minutes. | Existing |
| FR-TIMER-04 | Short Break duration range: 1–90 minutes; default 5 minutes. | Existing |
| FR-TIMER-05 | Long Break duration range: 1–90 minutes; default 15 minutes. | Existing |
| FR-TIMER-06 | The user shall be able to start the timer from the initial idle state. | Existing |
| FR-TIMER-07 | The user shall be able to pause a running timer. | Existing |
| FR-TIMER-08 | The user shall be able to resume a paused timer from where it stopped. | Existing |
| FR-TIMER-09 | The user shall be able to reset the current timer to the round's full duration. | Existing |
| FR-TIMER-10 | The user shall be able to skip the current round, advancing to the next. | Existing |
| FR-TIMER-11 | The timer shall run in a Rust thread using a monotonic clock with drift correction. | New |
| FR-TIMER-12 | The timer shall not drift more than 1 second per 60 seconds of elapsed time. | New |
| FR-TIMER-13 | The timer shall continue running when the application window is hidden or minimized. | Existing (improved) |
| FR-TIMER-14 | Timer state (running/paused/idle) shall be recoverable after the frontend window is restored. | Existing |
| FR-TIMER-15 | **When the OS enters sleep/hibernate, the timer shall pause automatically.** On wake, the timer shall resume from the exact elapsed position where it paused. | New (OQ-1) |

### FR-SEQ — Round Sequence

| ID | Requirement | Source |
|---|---|---|
| FR-SEQ-01 | The work-to-break sequence shall follow the Pomodoro technique: Work → Short Break → Work → ... → (after N work rounds) Long Break → Work → ... | Existing |
| FR-SEQ-02 | The number of work rounds before a long break shall be configurable: range 1–12; default 4. | Existing |
| FR-SEQ-03 | After a long break, the round counter shall reset to 1. | Existing |
| FR-SEQ-04 | The current round number (e.g., "2/4") shall be displayed to the user. | Existing |
| FR-SEQ-05 | The total number of completed work rounds for the current session shall be displayed. | Existing |
| FR-SEQ-06 | Total work rounds completed (all-time) shall be persisted to the database. | Improved |

### FR-AUTO — Auto-Start

| ID | Requirement | Source |
|---|---|---|
| FR-AUTO-01 | When enabled, the work timer shall start automatically after a break completes (after a 1.5 second delay). | Existing |
| FR-AUTO-02 | When enabled, the break timer shall start automatically after a work round completes (after a 1.5 second delay). | Existing |
| FR-AUTO-03 | Auto-start for work and break timers shall be independently configurable. Both default to on. | Existing |

### FR-UI — User Interface

| ID | Requirement | Source |
|---|---|---|
| FR-UI-01 | The application window shall be fixed at 360×478 pixels. | Existing |
| FR-UI-02 | The application shall use a custom (frameless) window with a custom titlebar. | Existing |
| FR-UI-03 | The titlebar shall be draggable to move the window. | Existing |
| FR-UI-04 | A circular dial shall visually represent time remaining in the current round. | Existing |
| FR-UI-05 | The dial color shall reflect the current round type (red=work, green=short break, blue=long break). | Existing |
| FR-UI-06 | The current round type label ("Focus", "Short Break", "Long Break") shall be displayed inside the dial. | Existing |
| FR-UI-07 | Before the timer starts, the display shall show `MM:00` (full minutes). | Existing |
| FR-UI-08 | While the timer is running, the display shall show `MM:SS` (time remaining). | Existing |
| FR-UI-09 | A hamburger/settings button shall open a settings drawer. | Existing |
| FR-UI-10 | The drawer shall slide in from the left and contain four tabs: Timer, Settings, Themes, About. | Existing |
| FR-UI-11 | The drawer shall cover the main timer view when open. | Existing |
| FR-UI-12 | Pressing the Space bar shall toggle the timer (play/pause). | Existing |

### FR-AUDIO — Audio

| ID | Requirement | Source |
|---|---|---|
| FR-AUDIO-01 | The app shall play an alert sound when transitioning to a work round. | Existing |
| FR-AUDIO-02 | The app shall play an alert sound when transitioning to a short break round. | Existing |
| FR-AUDIO-03 | The app shall play an alert sound when transitioning to a long break round. | Existing |
| FR-AUDIO-04 | The app shall support an optional tick sound played each second. | Existing |
| FR-AUDIO-05 | Tick sounds during work rounds shall be independently toggleable. Default: off. | Existing |
| FR-AUDIO-06 | Tick sounds during break rounds shall be independently toggleable. Default: on. | Existing |
| FR-AUDIO-07 | Volume shall be adjustable from 0 to 100. Default: 100. | Existing |
| FR-AUDIO-08 | A mute toggle shall be available in the timer footer. | Existing |
| FR-AUDIO-09 | A volume slider shall be accessible from the mute button area. | Existing |
| FR-AUDIO-10 | Volume setting shall be persisted. | Existing |
| FR-AUDIO-11 | **Audio shall play via Rust (`rodio`) in the main process**, not the webview. This guarantees playback when the window is hidden to the system tray. | New (OQ-8) |
| FR-AUDIO-12 | Audio files shall be embedded in the binary at compile time (no external file path dependency). | New |

### FR-NOTIFY — Notifications

| ID | Requirement | Source |
|---|---|---|
| FR-NOTIFY-01 | The app shall send a desktop notification when transitioning between rounds. | Existing |
| FR-NOTIFY-02 | Notifications shall include a round-appropriate title and body message. | Existing |
| FR-NOTIFY-03 | Notifications shall use a round-appropriate icon (work/short break/long break). | Existing |
| FR-NOTIFY-04 | Notifications shall be globally toggleable. Default: on. | Existing |
| FR-NOTIFY-05 | Notifications shall be silent (no notification sound — audio is handled separately). | Existing |
| FR-NOTIFY-06 | Notifications shall work on Windows, macOS, and Linux without platform-specific code paths. | Improved |

### FR-SETTINGS — Settings

| ID | Requirement | Source |
|---|---|---|
| FR-SET-01 | Always On Top: window stays above all other windows. Default: off. | Existing |
| FR-SET-02 | Break Always On Top: deactivates Always On Top during break rounds. Only effective when Always On Top is enabled. Default: off. | Existing |
| FR-SET-03 | Minimize to Tray: minimize action hides the window to the system tray. Default: off. | Existing |
| FR-SET-04 | Minimize to Tray on Close: close button hides to tray instead of closing. Default: off. | Existing |
| FR-SET-05 | All settings shall be persisted to the SQLite database immediately on change. | Improved |
| FR-SET-06 | A "Reset Defaults" button shall reset all timer durations and round count to factory defaults. | Existing |

### FR-THEME — Themes

| ID | Requirement | Source |
|---|---|---|
| FR-THEME-01 | The app shall ship with 17 built-in themes. | Existing |
| FR-THEME-02 | Users shall be able to add custom themes by placing JSON files in `{userData}/themes/`. | Existing |
| FR-THEME-03 | Custom theme format: JSON with `name` (string) and `colors` (10 hex color values). | Existing |
| FR-THEME-04 | The theme selection shall be persisted and applied on the next launch. | Existing |
| FR-THEME-05 | The themes panel shall display each theme's name with its own background and accent colors. | Existing |
| FR-THEME-06 | The currently active theme shall show a checkmark in the theme list. | Existing |
| FR-THEME-07 | Custom themes shall be loaded without requiring an app restart. | Improved |

### FR-TRAY — System Tray

| ID | Requirement | Source |
|---|---|---|
| FR-TRAY-01 | When Minimize to Tray is enabled, a system tray icon shall be shown. | Existing |
| FR-TRAY-02 | Clicking the tray icon shall toggle window visibility. | Existing |
| FR-TRAY-03 | The tray context menu shall contain "Show" and "Exit" options. | Existing |
| FR-TRAY-04 | On macOS, showing the window via tray shall position it below the tray icon. | Existing |
| FR-TRAY-05 | The tray icon shall reflect the current round type and pause state. | Existing |

### FR-SHORTCUTS — Global Shortcuts

| ID | Requirement | Source |
|---|---|---|
| FR-SC-01 | Three global keyboard shortcuts shall be supported: Toggle Timer, Reset Timer, Skip Round. | Existing |
| FR-SC-02 | Default shortcuts: Toggle=Ctrl+F1, Reset=Ctrl+F2, Skip=Ctrl+F3. | Existing |
| FR-SC-03 | Each shortcut shall be user-configurable via a key-capture input in the settings panel. | Existing |
| FR-SC-04 | Shortcuts shall support modifier keys: Control, Shift, Alt, Super (Windows/Cmd key). | Existing (Meta key was broken — must be fixed) |
| FR-SC-05 | Shortcut changes shall take effect immediately without restarting the app. | Existing |

### FR-WS — WebSocket Server (Optional / Opt-In)

| ID | Requirement | Source |
|---|---|---|
| FR-WS-01 | A local WebSocket server shall be optionally available for external integrations. Default: **off**. | Improved (OQ-3) |
| FR-WS-02 | The WebSocket server shall bind to `127.0.0.1` (localhost only) to prevent external access. | Improved (existing binds to all interfaces) |
| FR-WS-03 | The port shall be configurable via the Settings panel. Default: 1314. | Improved (OQ-3) |
| FR-WS-04 | The server shall support `getState` requests and `roundChange` broadcast events. | Existing |
| FR-WS-05 | The server shall be enabled/disabled via a settings toggle. | Required (OQ-3) |
| FR-WS-06 | If the server fails to bind (port in use), the app shall emit a visible error to the user rather than failing silently. | Improved |

### FR-ABOUT — About Panel

| ID | Requirement | Source |
|---|---|---|
| FR-ABOUT-01 | The About panel shall display the current application version. | Existing |
| FR-ABOUT-02 | The About panel shall provide a link to release notes on GitHub. | Existing |
| FR-ABOUT-03 | The About panel shall provide a link to the project's license/documentation. | Existing |
| FR-ABOUT-04 | Links shall open in the system's default web browser. | Existing |

---

## 2. Non-Functional Requirements

### NFR-PERF — Performance

| ID | Requirement |
|---|---|
| NFR-PERF-01 | Application cold start time shall be under 1 second on target hardware. |
| NFR-PERF-02 | Memory usage at idle shall not exceed 80MB RSS. |
| NFR-PERF-03 | CPU usage at idle (timer running, window visible) shall not exceed 2% on a modern CPU. |
| NFR-PERF-04 | CPU usage at idle (timer running, window hidden) shall not exceed 0.5%. |
| NFR-PERF-05 | The installed application size shall not exceed 20MB. |

### NFR-ACCURACY — Timer Accuracy

| ID | Requirement |
|---|---|
| NFR-ACC-01 | Timer drift shall not exceed 1 second per 60 seconds of elapsed time under normal system load. |
| NFR-ACC-02 | Timer drift shall not exceed 1 second per 60 seconds when the application window is hidden. |
| NFR-ACC-03 | Timer accuracy shall be maintained when another application is consuming high CPU. |
| NFR-ACC-04 | The timer completion event shall fire within 500ms of the actual elapsed time matching the configured duration. |

### NFR-PLATFORM — Cross-Platform

| ID | Requirement | Phase |
|---|---|---|
| NFR-PLAT-01 | The app shall run on Linux (Ubuntu 22.04+, Fedora 38+), x64. | MVP |
| NFR-PLAT-02 | The app shall run on Windows 10 and later (x64). | Phase 2 |
| NFR-PLAT-03 | The app shall run on macOS 10.15 (Catalina) and later (x64 and ARM64 universal binary). | Phase 2 |
| NFR-PLAT-04 | All platform builds shall be produced by CI/CD with no manual steps. | MVP (Linux); Phase 2 (all) |
| NFR-PLAT-05 | The app shall use the default system window behavior per platform. Custom titlebar polish is post-MVP. | MVP |
| NFR-PLAT-06 | Additional ARM64 targets (Linux, Windows) may be added in a future phase after x64 builds are stable. | Future |

### NFR-RESOURCE — Resource Usage

| ID | Requirement |
|---|---|
| NFR-RES-01 | The app shall use no network connections except the optional local WebSocket server. |
| NFR-RES-02 | The app shall not spawn unnecessary background processes. |
| NFR-RES-03 | The app shall not write to disk more than once per user action (no continuous polling writes). |

### NFR-SECURITY — Security

| ID | Requirement |
|---|---|
| NFR-SEC-01 | The WebSocket server (when enabled) shall bind to localhost only (`127.0.0.1`). |
| NFR-SEC-02 | Tauri CSP shall be configured to disallow external network requests from the webview. |
| NFR-SEC-03 | No remote content shall be loaded into the webview. |
| NFR-SEC-04 | Tauri IPC shall expose only explicitly defined commands (allowlist). |

### NFR-MAINTAIN — Maintainability

| ID | Requirement |
|---|---|
| NFR-MAIN-01 | All Rust code shall compile without warnings under `cargo clippy`. |
| NFR-MAIN-02 | Timer engine logic shall be unit-testable in isolation from the Tauri application. |
| NFR-MAIN-03 | The settings module shall be independently testable. |
| NFR-MAIN-04 | The project shall maintain a `CHANGELOG.md` following existing conventions. |

---

## 3. Out-of-Scope Items

The following items are explicitly **not** being built in this rewrite:

| Item | Rationale |
|---|---|
| Auto-updater | Not shipped in the original app; complex to implement correctly with code signing. Revisit post-launch. |
| Statistics / history view | No UI exists for this in the original. Schema supports it for future. |
| Task tracking / to-do list | Outside the scope of a Pomodoro timer. |
| Cloud sync / accounts | Out of scope; app is local-only. |
| Mobile (iOS / Android) | Not a target platform. |
| Web browser version | Original had a `build:web` target but it was never maintained. Not replicating. |
| Custom sound uploads | Original only supports bundled sounds. Not expanding. |
| Plugin / extension system | Not in original scope. |
| Multiple timer profiles | Not in original scope. |
| Integration with task management apps | Not in original scope (WebSocket server covers basic external integration). |

---

## 4. Acceptance Criteria

### AC-TIMER — Timer Accuracy
- [ ] A 25-minute timer started and left running completes within ±2 seconds of 25:00 of real time.
- [ ] A 25-minute timer started and left running with the window hidden completes within ±2 seconds.
- [ ] A paused timer, when resumed, continues from the correct elapsed position.
- [ ] Reset returns the timer to the full configured duration, displayed as `MM:00`.

### AC-SEQ — Round Sequencing
- [ ] With work rounds = 4: completing 4 work rounds triggers a long break.
- [ ] After a long break, the round counter resets to 1/4.
- [ ] `totalWorkRounds` increments on each completed work round and is displayed correctly.
- [ ] Session `totalWorkRounds` persists across app restarts.

### AC-AUDIO — Audio
- [ ] Alert sounds play on each round transition.
- [ ] Tick sounds play every second when enabled for the current round type.
- [ ] Volume slider adjusts audio volume in real time.
- [ ] Mute toggle silences all sounds; unmute restores the previous volume.

### AC-NOTIFY — Notifications
- [ ] Desktop notification appears on round transition on Windows, macOS, and Linux.
- [ ] Disabling notifications prevents any desktop notifications from appearing.
- [ ] Notification content matches the incoming round type.

### AC-SETTINGS — Settings Persistence
- [ ] All settings survive an app restart.
- [ ] Changing a setting takes effect immediately (no restart required).
- [ ] "Reset Defaults" restores all timer settings to factory values.

### AC-THEME — Themes
- [ ] All 17 built-in themes are listed and selectable.
- [ ] Selecting a theme immediately updates the application appearance.
- [ ] A JSON file placed in `{userData}/themes/` appears in the theme list without restarting.
- [ ] The active theme is remembered across restarts.

### AC-TRAY — System Tray
- [ ] Enabling "Minimize to Tray" creates the tray icon.
- [ ] Clicking the tray icon toggles window visibility.
- [ ] "Exit" in the context menu fully quits the application.
- [ ] Disabling "Minimize to Tray" removes the tray icon.

### AC-SHORTCUTS — Global Shortcuts
- [ ] Default shortcuts (Ctrl+F1/F2/F3) work system-wide.
- [ ] Capturing a new shortcut in the input field updates the shortcut immediately.
- [ ] The shortcut works even when the application window is hidden.
- [ ] Super/Cmd key modifier is correctly captured and functional.

### AC-PLATFORM — Platform (MVP: Linux x64)
- [ ] App builds successfully on Linux x64 via CI.
- [ ] All features (timer, tray, shortcuts, notifications, audio) work on Linux x64.
- [ ] Sleep/wake: pausing the system while the timer is running, then waking, resumes the timer from the correct position.
- [ ] Windows and macOS builds are added in Phase 2 (not required for MVP acceptance).

### AC-AUDIO — Rust Audio
- [ ] Alert and tick sounds play correctly when the window is **hidden to the tray**.
- [ ] Volume control adjusts playback level in real time.
- [ ] Mute (volume=0) silences all sounds. Unmute restores to the previous level.

### AC-RESOURCE — Resource Usage
- [ ] Memory usage ≤ 80MB RSS at idle with timer running.
- [ ] Installed size ≤ 20MB.
- [ ] CPU usage ≤ 2% at idle (visible), ≤ 0.5% (hidden).
