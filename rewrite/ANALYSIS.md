# Pomotroid Codebase Analysis

> Complete audit of the existing Electron + Vue.js application prior to rewrite.

---

## 1. Feature Inventory

### 1.1 Timer Engine

| Feature | Detail |
|---|---|
| Work (Focus) round | Configurable 1–90 min, default 25 min |
| Short break round | Configurable 1–90 min, default 5 min |
| Long break round | Configurable 1–90 min, default 15 min |
| Start | Begins countdown from full duration |
| Pause | Suspends countdown, preserves elapsed time |
| Resume | Continues from paused position |
| Reset | Returns current round to full duration, does not advance round |
| Skip | Immediately completes the current round and advances to the next |
| Auto-start work timer | After a break ends, work timer starts automatically (default: on) |
| Auto-start break timer | After a work round ends, break timer starts automatically (default: on) |
| 1500ms delay on auto-start | There is a `setTimeout(1500)` before auto-started timers begin |
| Work rounds counter | Tracks current round number (1 to N) within a cycle |
| Total work rounds | Session-level counter of completed focus rounds (never persisted, lost on restart) |
| Reset Defaults | Resets work/break durations and rounds count to factory defaults |

#### Timer Implementation Detail
The timer runs inside a **Web Worker** (`timer.worker.js`) which wraps a `Timer` class (`Timer.js`). The worker receives commands via `postMessage` and emits events back to the main render thread. The timer mechanism is `setInterval(fn, 1000)` — it increments an integer counter (`time`) by 1 each second.

**Critical issue**: `setInterval` does not fire exactly every 1000ms. Under CPU load or when the browser throttles the worker, intervals can fire late, causing the displayed time to drift behind real time. There is no drift-correction mechanism.

### 1.2 Pomodoro Sequence Logic

```
Work → Short Break → Work → Short Break → ... → (after N work rounds) → Long Break → Work (reset counter) → ...
```

- After each work round, `round` counter is incremented.
- When `round >= workRounds`, the next break is a long break and `round` resets to 1.
- `totalWorkRounds` (session counter) is incremented on each completed work round.
- After any break, the next round is always work.

### 1.3 Timer Dial (Visual)

- Circular SVG arc that animates from full to empty as time elapses.
- Animation driven by **anime.js** using `requestAnimationFrame` (RAF).
- Arc color changes by round type: Focus=red (`--color-focus-round`), Short Break=green (`--color-short-round`), Long Break=blue (`--color-long-round`).
- Label beneath center shows "Focus", "Short Break", or "Long Break".
- Time display: shows `MM:00` before timer starts; shows `MM:SS` (time remaining) once started.
- `prettyTime` calculation: remaining time computed from integer elapsed seconds and total minutes, formatted with zero-padded seconds.
- **RAF pause problem**: When the window is hidden or minimized, RAF stops executing, freezing the animation. On `win-restore` and `win-show` IPC events, the animation seeks to the correct elapsed position and resumes. The timer worker continues counting regardless.

### 1.4 Audio System

| Sound | Trigger |
|---|---|
| `alert-work.mp3` | Fired when a break ends (`ready-work` event) |
| `alert-short-break.mp3` | Fired when a short break begins (`ready-short-break` event) |
| `alert-long-break.mp3` | Fired when a long break begins (`ready-long-break` event) |
| `tick.mp3` | Fired on every timer tick (if enabled) |

- Volume is controlled via a 0–100 integer stored in preferences, applied as `volume * 0.01` to the HTML `<audio>` element.
- Tick sounds have separate toggles for work rounds and break rounds.
- HTML `<audio>` elements are pre-rendered in the DOM with static `src` paths.
- **Note**: The Web Notification API notifications are set to `silent: true` — all audio is handled by the Audio component, not the notification system.

### 1.5 Volume Control

- Mute/unmute toggle button in the timer footer (click).
- Hover over mute button reveals a vertical range slider.
- Slider hides after 6 seconds unless the mouse is within a hardcoded pixel region (305–355 x, 305–455 y). This is brittle and tied to the fixed window size.
- Volume is persisted to the JSON preferences file.
- Toggling mute: if volume is `'0'` (string), sets to `'100'`; otherwise sets to `'0'`. Note: comparison uses string `'0'` but `localVolume` starts as a number from the store.

### 1.6 Notifications

Two separate implementations exist:

| Platform | Implementation |
|---|---|
| Windows (`win32`) | `node-notifier` npm package |
| macOS / Linux | Web Notification API (browser built-in) |

On each round transition:
- Work round ending → long break: "Focus Round Complete" + "Begin a N minute long break." + blue icon
- Work round ending → short break: "Focus Round Complete" + "Begin a N minute short break." + green icon
- Break ending → work: "Break Finished" + "Begin focusing for N minutes." + default icon

Notifications use `silent: true` (Web API) or `sound: false` (node-notifier). Notifications can be disabled globally via settings. The condition in `App.vue` selects the correct component at mount time based on `os` (platform string from `process.platform`).

### 1.7 Settings Panel

All settings are boolean toggles except timer durations:

| Setting | Default | Persisted |
|---|---|---|
| Always On Top | false | yes |
| Break Always On Top | false | yes |
| Auto-start Work Timer | true | yes |
| Auto-start Break Timer | true | yes |
| Tick Sounds (Work) | false | yes |
| Tick Sounds (Break) | true | yes |
| Desktop Notifications | true | yes |
| Minimize to Tray | false | yes |
| Minimize to Tray on Close | false | yes |

**"Break Always On Top" dependency**: This option is only shown when "Always On Top" is active. When "Always On Top" is turned off, `breakAlwaysOnTop` is also turned off by programmatically clicking the checkbox DOM element — a fragile technique.

### 1.8 Global Shortcuts

Three system-wide (OS-level) keyboard shortcuts:

| Action | Default |
|---|---|
| Toggle Timer | Control+F1 |
| Reset Timer | Control+F2 |
| Skip Round | Control+F3 |

- Registered via Electron's `globalShortcut.register()` in the main process.
- User-configurable via a custom `ShortcutInput` component that captures keyup events.
- **Known bug**: `metaKey` (Windows key / Cmd) detection does not work. The code pushes `'Super'` but the behavior is broken (self-noted in comment).
- On shortcut change, main process unregisters all shortcuts and re-registers from the new config.
- Shortcut format: `Control+F1`, `Shift+Alt+P`, etc. (Electron accelerator format).
- Shortcuts fire an `event-bus` IPC message from main to renderer, which the Timer component forwards to the EventBus.

### 1.9 Themes

- **17 built-in themes**: Andromeda, Ayu Mirage, City Lights, Dracula, D.Va, GitHub, Graphite, Gruvbox, Monokai, Nord, One Dark Pro, Pomotroid (default), Popping and Locking, Solarized Light, Spandex, Synthwave, Tokyo Night Storm.
- **Custom themes**: Users can drop JSON files into `{userData}/themes/` directory. App must be restarted to pick up new theme files.
- Theme format is a JSON file with `name` and `colors` object containing 10 CSS custom property values.
- Themes are applied by setting CSS custom properties on `document.documentElement`.
- The `Themer` class loads themes from both the bundled static directory and the user data directory at startup.
- Selected theme name is persisted to preferences. The default theme ("Pomotroid") is not applied via `themer.apply()` — it falls through to the CSS defaults (`:root` block in `_variables.scss`).

**10 CSS custom properties (theme tokens):**
```
--color-long-round       (long break arc and accents)
--color-short-round      (short break arc, title text)
--color-focus-round      (work arc)
--color-background       (main window background)
--color-background-light (drawer background)
--color-background-lightest (borders, inactive icon fill)
--color-foreground       (primary text, button icons)
--color-foreground-darker  (secondary text)
--color-foreground-darkest (tertiary text)
--color-accent           (active checkbox fill, hover color, selected theme checkmark)
```

### 1.10 System Tray

- **Optional** (enabled via "Minimize to Tray" setting).
- On macOS: uses `icon--macos--tray.png` (19px, black/white for menu bar). On other platforms: `icon.png` (32px).
- **Context menu**: View (toggle window), Exit (quit app).
- **Click behavior**: Toggles window visibility.
- **macOS special behavior**: On click, positions the window directly below the tray icon.
- **Animated tray icon**: A canvas-rendered arc showing timer progress, matching the theme colors (reads CSS custom property values at render time).
  - Colors: bg, focus, short break, long break — read from `document.documentElement.style`.
  - Paused state: shows two vertical bars (pause icon).
  - Active state: shows arc from top, sweeping clockwise as time remaining decreases.
  - Update rate: throttled — skips updates when elapsed change is < 1% of total (reduces unnecessary redraws).
  - Size: 19px on macOS, 32px on other platforms.
  - Tray icon update is sent via IPC as a data URL: `tray-icon-update` → main process reconstructs as `nativeImage`.

### 1.11 Window Management

- **Fixed size**: 360×478 pixels, non-resizable, non-fullscreenable.
- **Frameless**: Custom titlebar replaces OS chrome.
- **Drag region**: Titlebar uses `-webkit-app-region: drag`; interactive elements use `no-drag`.
- **Background color**: `#2F384B` (hardcoded on BrowserWindow, not themed).
- **Minimize**: Sends `window-minimize` IPC with a boolean (true = hide to tray, false = OS minimize).
- **Close**: If `minToTrayOnClose` is set, close button calls minimize instead; otherwise sends `window-close` IPC.
- **Menu bar removed**: `mainWindow.setMenu(null)` prevents Ctrl+W from closing the window (see issue #121).
- **Hardware acceleration disabled**: `app.disableHardwareAcceleration()` called at startup.
- **Space key**: Bound to play/pause toggle in the Timer component via `window.addEventListener('keypress')`.

### 1.12 WebSocket Server

- Local WebSocket server runs on **port 1314** at startup (always, not optional).
- Allows external tools (e.g., stream decks, status bar widgets) to monitor timer state.
- Supported events:
  - `getState` (client → server): Returns `{ event: 'getState', data: { state: timerState } }`.
  - `roundChange` (broadcast to all clients): Fired when round changes; state is one of `'work'`, `'short-break'`, `'long-break'`, `'paused'`, or `'idle'`.
- No authentication or rate limiting.
- Error handling: logs errors, but if port 1314 is already bound, the server fails silently (the error is logged).

### 1.13 Logging

- **Library**: Winston with `winston-daily-rotate-file`.
- **Error log**: `pomotroid-error.log` (max size: 1KB, then overwritten — likely a bug; should be larger or rotating).
- **Daily log**: `pomotroid-{DATE}.log` (max 20MB per file, 14 days retention).
- **Log location**: `{userData}/logs/`.
- Development mode adds console transport.
- All log entries include `hostname` as metadata.
- **Usage**: Main process and renderer process both import the logger. Renderer imports from relative path `../renderer/utils/logger`.

### 1.14 About Panel

- Displays app version (read from `package.json`).
- Link to release notes on GitHub (constructs URL from version string).
- Link to license/documentation on GitHub.
- Uses `electron.shell.openExternal()` to open URLs in system browser.

---

## 2. Application States and Transitions

### 2.1 Timer State Machine

```
                    ┌─────────────────┐
                    │      IDLE       │ ← app start, after reset
                    └────────┬────────┘
                             │ start
                             ▼
                    ┌─────────────────┐
              ┌────►│    RUNNING      │◄─────┐
              │     └────────┬────────┘       │
              │              │ pause           │ resume
              │              ▼                │
              │     ┌─────────────────┐       │
              │     │     PAUSED      ├───────┘
              │     └────────┬────────┘
              │              │ reset
              │              ▼
              │     ┌─────────────────┐
              │     │    RESETTING    │ (immediate)
              │     └────────┬────────┘
              │              │
              └──────────────┘

          RUNNING ──(complete/skip)──► ROUND_TRANSITION ──► next IDLE (auto or manual)
```

### 2.2 Round Sequence State Machine

```
State: { currentRound, round, workRounds }

work [round < workRounds]  ──complete──► short-break ──complete──► work [round++]
work [round >= workRounds] ──complete──► long-break  ──complete──► work [round=1, totalWorkRounds++]
```

### 2.3 Drawer State

```
CLOSED ──(hamburger click)──► OPEN
OPEN   ──(hamburger click)──► CLOSED
OPEN: currentDrawer ∈ { Timer, Settings, Themes, About }
```

### 2.4 Window Visibility State

```
VISIBLE ──(minimize, minToTray=false)──► MINIMIZED
VISIBLE ──(minimize, minToTray=true) ──► HIDDEN (tray only)
VISIBLE ──(close, minToTrayOnClose=false)──► CLOSED (quit on non-Darwin)
VISIBLE ──(close, minToTrayOnClose=true) ──► HIDDEN (tray only)
HIDDEN  ──(tray click or "View")──► VISIBLE
MINIMIZED──(OS restore)──► VISIBLE
```

---

## 3. Data Models and Persistence

### 3.1 Preferences File

**Location**: `{electron.app.getPath('userData')}/user-preferences.json`

**OS Paths**:
- Windows: `%APPDATA%\pomotroid\user-preferences.json`
- macOS: `~/Library/Application Support/pomotroid/user-preferences.json`
- Linux: `~/.config/pomotroid/user-preferences.json`

**Schema**:
```json
{
  "alwaysOnTop": false,
  "breakAlwaysOnTop": false,
  "autoStartWorkTimer": true,
  "autoStartBreakTimer": true,
  "minToTray": false,
  "minToTrayOnClose": false,
  "notifications": true,
  "workRounds": 4,
  "theme": null,
  "tickSounds": false,
  "tickSoundsDuringBreak": true,
  "timeLongBreak": 15,
  "timeShortBreak": 5,
  "timeWork": 25,
  "volume": 100,
  "globalShortcuts": {
    "call-timer-toggle": "Control+F1",
    "call-timer-reset": "Control+F2",
    "call-timer-skip": "Control+F3"
  }
}
```

**Persistence Mechanism**: `LocalStore` class reads file on startup. Writes entire JSON blob on each change via `fs.writeFileSync`.

### 3.2 Custom Themes

**Location**: `{userData}/themes/*.json`

Theme file format:
```json
{
  "name": "Theme Name",
  "colors": {
    "--color-long-round": "#hex",
    "--color-short-round": "#hex",
    "--color-focus-round": "#hex",
    "--color-background": "#hex",
    "--color-background-light": "#hex",
    "--color-background-lightest": "#hex",
    "--color-foreground": "#hex",
    "--color-foreground-darker": "#hex",
    "--color-foreground-darkest": "#hex",
    "--color-accent": "#hex"
  }
}
```

### 3.3 Session State (Not Persisted)

These values are lost on app restart:
- `totalWorkRounds` — cumulative completed focus rounds for the session
- `round` — current round number within the cycle
- `currentRound` — current round type (work/short-break/long-break)
- Timer elapsed time (no resumption across sessions)

### 3.4 Log Files

**Location**: `{userData}/logs/`
- `pomotroid-error.log` — max 1KB error log
- `pomotroid-{YYYY-MM-DD}.log` — daily rotating, 14-day retention, 20MB max

---

## 4. IPC / Communication Patterns

### 4.1 Renderer → Main (one-way)

| Channel | Payload | Effect |
|---|---|---|
| `toggle-alwaysOnTop` | `boolean` | `mainWindow.setAlwaysOnTop(arg)` |
| `toggle-breakAlwaysOnTop` | `boolean` | Stores `breakAlwaysOnTop` flag in main |
| `onBreak` | `boolean` | If `breakAlwaysOnTop`, toggles always-on-top off during breaks |
| `toggle-minToTray` | `boolean` | Creates or destroys the Tray instance |
| `window-close` | none | `mainWindow.close()` |
| `window-minimize` | `boolean` | `true` → `hide()`, `false` → `minimize()` |
| `tray-icon-update` | data URL string | Updates tray icon from canvas-rendered PNG |
| `reload-global-shortcuts` | shortcuts object | Unregisters all, re-registers from new config |
| `roundChange` | string state | Updates WebSocket server's `timerState` and broadcasts |

### 4.2 Main → Renderer (one-way)

| Channel | Payload | Effect |
|---|---|---|
| `win-restore` | none | Timer dial animation seeks to correct position |
| `win-show` | none | Timer dial animation seeks to correct position |
| `event-bus` | string event name | Forwarded to EventBus (global shortcuts) |

### 4.3 Internal Renderer Communication (EventBus)

The EventBus is a bare Vue instance used as a pub/sub system between components.

| Event | Publisher | Subscribers |
|---|---|---|
| `timer-completed` | Timer.vue (worker message), Timer-footer.vue (skip) | Timer-controller.vue |
| `timer-started` | Timer.vue | Timer-dial.vue, TrayIcon.vue |
| `timer-paused` | Timer.vue | Timer-dial.vue, TrayIcon.vue, Audio.vue (implicit via tick stop) |
| `timer-reset` | Timer.vue | Timer-dial.vue, TrayIcon.vue |
| `timer-tick` | Timer.vue | Timer-dial.vue (via RAF), TrayIcon.vue, Audio.vue |
| `timer-init` | Timer-controller.vue, Drawer-timer.vue | Timer.vue, Timer-dial.vue |
| `ready-work` | Timer-controller.vue | Audio.vue, Notification*.vue, TrayIcon.vue |
| `ready-short-break` | Timer-controller.vue | Audio.vue, Notification*.vue, TrayIcon.vue |
| `ready-long-break` | Timer-controller.vue | Audio.vue, Notification*.vue, TrayIcon.vue |
| `call-timer-reset` | Timer-footer.vue, Drawer-timer.vue | Timer.vue |
| `call-timer-toggle` | (global shortcut) | Timer.vue |
| `call-timer-skip` | (global shortcut) | Timer.vue → timer-completed |

### 4.4 Worker ↔ Renderer Thread

| Direction | Message | Fields |
|---|---|---|
| Renderer → Worker | `{ event: 'create', min }` | Creates timer with N minutes |
| Renderer → Worker | `{ event: 'start' }` | Starts interval |
| Renderer → Worker | `{ event: 'pause' }` | Clears interval |
| Renderer → Worker | `{ event: 'resume' }` | Restarts interval from current position |
| Renderer → Worker | `{ event: 'reset' }` | Resets time to 0, clears interval |
| Worker → Renderer | `{ event: 'complete' }` | Timer reached totalSeconds |
| Worker → Renderer | `{ event: 'pause' }` | Pause acknowledged |
| Worker → Renderer | `{ event: 'reset' }` | Reset acknowledged |
| Worker → Renderer | `{ event: 'start', elapsed, totalSeconds }` | Start acknowledged |
| Worker → Renderer | `{ event: 'resume', elapsed, totalSeconds }` | Resume acknowledged |
| Worker → Renderer | `{ event: 'tick', elapsed, totalSeconds }` | Each second tick |

---

## 5. External Dependencies

### Runtime Dependencies

| Package | Version | Purpose |
|---|---|---|
| `animejs` | ^3.2.1 | SVG arc animation for the timer dial |
| `node-notifier` | ^8.0.1 | Windows desktop notifications (bypasses Web Notification API issues on Windows) |
| `vue` | ^2.6.12 | UI framework |
| `vue-electron` | ^1.0.6 | Bridges `this.$electron` in Vue components |
| `vuex` | ^3.6.2 | State management |
| `winston` | ^3.3.3 | Structured logging |
| `winston-daily-rotate-file` | ^4.5.5 | Log rotation transport |
| `worker-loader` | ^2.0.0 | Webpack loader to bundle the Web Worker |
| `ws` | ^7.4.6 | WebSocket server for external integrations |

### Dev Dependencies (notable)

- `electron` ^11.0.1 — Very old version (current is 33+). Electron 11 uses Node.js 12. This is a significant security concern.
- `electron-builder` ^22.10.5 — Packaging tool.
- `webpack` ^4 — Old webpack version.
- `node-sass` ^4 — Deprecated (replaced by Dart Sass).
- `babel-*` — Extensive Babel plugin set for transpilation.

---

## 6. Known Limitations, Bugs, and Tech Debt

### 6.1 Timer Accuracy (Critical)
`setInterval(fn, 1000)` does not guarantee 1-second precision. JavaScript event loop delays, system load, and OS scheduling can cause intervals to fire late. There is **no drift correction**. Over a 25-minute session, the displayed time can lag several seconds behind actual elapsed time.

### 6.2 Animation Decoupled from Timer
The anime.js animation and the worker timer are two separate clocks. The animation runs via RAF and is synchronized at the start of a round, but they can diverge. On window restore, the animation is re-synced to the worker's elapsed count — this is a workaround, not a fix.

### 6.3 Session State Lost on Restart
`totalWorkRounds`, `round`, and `currentRound` are not persisted. Closing and reopening the app always starts fresh from the beginning of a work round with round counter at 1.

### 6.4 metaKey / Super Key Broken
The `ShortcutInput` component attempts to detect the Windows/Cmd key (`metaKey`) but the comment says "this doesn't work I don't know why". Users cannot bind Super/Windows key shortcuts.

### 6.5 Volume Slider Timeout Hardcoded Pixels
The volume slider visibility timeout checks absolute pixel coordinates (305–355 x, 305–455 y). This is tightly coupled to the fixed 360×478 window size and will break if layout changes.

### 6.6 Error Log Size Bug
`pomotroid-error.log` has `maxsize: 1000` (1KB). This is almost certainly a bug — any meaningful error log will overflow immediately. The value should likely be `1000000` (1MB) or use a rotating file approach.

### 6.7 `breakAlwaysOnTop` Toggled via DOM Click
When "Always On Top" is disabled, `breakAlwaysOnTop` is turned off by calling `.click()` on the checkbox DOM element. This is fragile and couples UI interaction to business logic.

### 6.8 `minToTrayOnClose` Behavior Not Obvious
The close (X) button calls minimize logic when `minToTrayOnClose` is true. There is no visual indicator that close will hide rather than quit. Users expect X to close or quit.

### 6.9 WebSocket Port Conflict Not Handled
If port 1314 is already bound (e.g., another instance running), `ws` throws an error that is logged but not surfaced to the user. The app continues with no WebSocket functionality and no indication.

### 6.10 Electron Version (Security)
Electron 11 (released 2020) is very old. It has many known CVEs. `nodeIntegration: true` and `enableRemoteModule: true` are both enabled with no `contextIsolation`, which is a significant security risk for any app loading remote content (though Pomotroid loads only local content).

### 6.11 Duplicate LocalStore Instances
`LocalStore` (and thus the preferences file) is instantiated in multiple places: `src/main/index.js`, `src/renderer/store/modules/index.js`. These use separate in-memory objects but write to the same file. Race conditions are unlikely (single user, low-frequency writes) but the pattern is bad.

### 6.12 Theme Requires App Restart
Custom themes placed in `{userData}/themes/` are only loaded at startup. There is no reload mechanism. Users must restart the app to see new themes.

### 6.13 `totalWorkRounds` Not Persisted
Session total focus rounds is displayed to the user but never saved. It is lost on every restart. Users who care about tracking productivity across sessions get no value from this counter.

### 6.14 Dead Code: Auto-Updater
Commented-out auto-updater code exists in `src/main/index.js`. It is not functional and was apparently never shipped.

### 6.15 `accent` Color Not Defined in Variables
`--color-accent` is not set in the CSS `:root` block in `_variables.scss` (only 9 of 10 tokens are set). The default theme JSON sets it, but when the "Pomotroid" theme is active, `themer.apply()` is not called (it's skipped in `App.vue`). This means `--color-accent` has no fallback if the default theme CSS doesn't define it — it would be empty. However, the default theme JSON matches the SCSS values so in practice, the default theme is expressed via SCSS and custom themes override via JS.

### 6.16 `vue-electron` is Abandoned
`vue-electron` (bridging `this.$electron`) is an old, unmaintained package. Vue 2 is also in end-of-life status.

---

## 7. Non-Obvious Behaviors (Easy to Miss in a Rewrite)

1. **1500ms auto-start delay**: When a round auto-starts, there is always a 1.5-second delay before the timer begins counting. This gives the user a moment to see the round change.

2. **Sound fires on `ready-*`, not `timer-completed`**: Audio is tied to the next-round-ready events, not the completion event. If you skip a round that has already completed (before auto-advance), the audio will still play.

3. **Timer dial seeks on restore**: The animation is not truly paused — it needs to seek to the current position on window focus. This is because RAF pauses when the window is hidden.

4. **Worker runs even when window is hidden**: The timer worker continues ticking even when the window is minimized to tray. The display is updated on restore.

5. **Reset during a round doesn't advance**: "Reset" returns the current round's timer to zero and stops it. It does NOT advance to the next round.

6. **Drawer hides the timer completely**: The drawer overlays the entire timer area (position absolute, full height). The timer continues running but is not visible while the drawer is open.

7. **Space key is a toggle, not a start**: Pressing Space while the timer has never been started will call `startTimer()`. If already started and active, it pauses. If paused, it resumes. The TimerWorker ignores start commands if already running (guarded by `if (!this.timerInt)`).

8. **Rounds counter shows `N/N` at long break trigger**: When `round >= workRounds`, `checkRound()` transitions to long-break before resetting `round`. So the display briefly shows e.g. "4/4" then resets to "1/4" after long break ends.

9. **Volume `'0'` is a string comparison**: In `toggleMute()`, the check is `if (this.localVolume === '0')`. `localVolume` is initialized from the store as a number but `v-model` on an `<input type="range">` returns a string. This means after interaction, it becomes a string, but on first mount it's a number. The toggle may not work on first press if volume was loaded as numeric 0.

10. **Theme "Pomotroid" uses CSS defaults, not the JSON file**: When the saved theme is `null` or `'Pomotroid'`, `App.vue` returns early without calling `themer.apply()`. The visual styling comes from the SCSS `:root` block. All other themes are applied via JS by overriding CSS variables.

11. **Tray icon reads theme colors at render time**: The canvas tray icon reads CSS custom property values from `document.documentElement.style`. If a CSS variable hasn't been set via `themer.apply()` (e.g., default Pomotroid theme), it falls back to hardcoded hex values in `TrayIcon.vue`.

12. **Global shortcuts registered before window ready**: On startup, `loadGlobalShortcuts` reads the stored shortcut config and registers them even if the window hasn't finished loading. Events will be queued via IPC.
