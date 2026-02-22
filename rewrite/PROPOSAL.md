# Rewrite Proposal: Pomotroid → Tauri + Rust + Svelte

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Process                         │
│  ┌──────────────────┐    ┌────────────────────────────┐ │
│  │   Svelte WebView │    │     Rust Backend           │ │
│  │                  │    │                            │ │
│  │  • UI rendering  │◄──►│  • Timer thread            │ │
│  │  • State display │IPC │  • Settings persistence    │ │
│  │  • Controls      │    │  • SQLite (rusqlite)       │ │
│  │  • Theme engine  │    │  • System tray             │ │
│  │  • Audio (Web API│    │  • Notifications           │ │
│  │  • Animations    │    │  • Global shortcuts        │ │
│  └──────────────────┘    │  • WebSocket server        │ │
│                          │  • Window management       │ │
│                          └────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

The key architectural shift: **all timer logic moves to a Rust thread**. The frontend becomes a pure display layer that receives state snapshots from Rust and sends commands to it. No timer arithmetic occurs in JS.

---

## 2. Concern Mapping: Existing → New Stack

| Existing (Electron/Vue/JS) | New (Tauri/Rust/Svelte) |
|---|---|
| `Timer.js` + `timer.worker.js` — setInterval in a Web Worker | Rust timer thread using `std::time::Instant` with monotonic drift correction |
| `Vuex` store — global state | Svelte stores (`writable`, `derived`) — no separate library needed |
| `LocalStore.js` — JSON file I/O | `rusqlite` SQLite database via Tauri's `app_dir` |
| `Themer.js` — runtime CSS variable injection | Svelte CSS custom properties + Rust reads theme JSON files from disk |
| Electron `globalShortcut` API | Tauri's `global-shortcut` plugin |
| Electron `Tray` + `nativeImage` | Tauri's `tray-icon` plugin |
| Web Notification API + `node-notifier` | Tauri's `notification` plugin (unified across platforms) |
| Electron IPC (`ipcMain`/`ipcRenderer`) | Tauri commands (`#[tauri::command]`) + events (`emit`/`listen`) |
| `ws` WebSocket server | Rust `tokio-tungstenite` or `axum` WebSocket handler |
| `winston` logger | Rust `tracing` + `tracing-subscriber` with file output |
| `electron-builder` packaging | Tauri bundler (`tauri build`) — built in |
| `animejs` for dial animation | CSS animation + Svelte tweened stores |

---

## 3. Timer Accuracy: Rust Implementation

### Problem with the Current Approach
`setInterval(fn, 1000)` in JavaScript (even in a Worker) is subject to:
- Event loop congestion
- OS scheduling jitter
- Browser throttling of background tabs/workers
- No drift correction whatsoever

A 25-minute session can drift 5–15 seconds on a loaded machine.

### Proposed Rust Solution: Monotonic Clock with Drift Correction

```rust
use std::time::{Duration, Instant};
use std::thread;

fn timer_loop(duration_secs: u64, sender: mpsc::Sender<TimerEvent>) {
    let start = Instant::now();
    let total = Duration::from_secs(duration_secs);
    let mut next_tick = start + Duration::from_secs(1);

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start);

        if elapsed >= total {
            sender.send(TimerEvent::Complete).ok();
            break;
        }

        let tick_elapsed = elapsed.as_secs();
        sender.send(TimerEvent::Tick { elapsed_secs: tick_elapsed }).ok();

        // Sleep until the next scheduled tick, correcting for any drift
        let sleep_duration = next_tick.saturating_duration_since(Instant::now());
        thread::sleep(sleep_duration);
        next_tick += Duration::from_secs(1);
    }
}
```

**Key properties**:
- Uses `Instant::now()` (monotonic clock) — unaffected by system clock changes.
- Each tick sleep is computed as the time until the *next scheduled moment*, not a fixed 1000ms. If a tick fires late, the next sleep is shorter to compensate.
- `elapsed` is always computed from the original `start` instant, not accumulated — it cannot drift.
- The timer runs in a dedicated OS thread, not the async executor. It is not affected by async task scheduling.
- The frontend receives `elapsed_secs` (an integer) and derives the display from it. It never does its own timekeeping.

### Backgrounding / System Sleep
If the OS suspends the machine (sleep/hibernate), `Instant::now()` will reflect actual elapsed time on wake. The timer should check total elapsed against `Instant::now() - start` on each tick and handle the case where many seconds have passed at once (skip directly to completion or skip to the current second).

**Open Question**: Should the timer skip ahead and complete immediately on wake from sleep if the session would have finished? Or should it remain paused while sleeping? (See Open Questions.)

---

## 4. Data Storage: SQLite Schema Proposal

Using `rusqlite` (synchronous, ideal for embedded desktop use).

### Why SQLite over JSON

| JSON File | SQLite |
|---|---|
| Full rewrite on every change | Row-level writes |
| No history / analytics possible | Sessions, rounds, totals queryable |
| No schema enforcement | Schema constraints enforced |
| No migration path | Schema migrations via version table |
| Losing `totalWorkRounds` on restart | Persisted across sessions |

### Proposed Schema

```sql
-- Application settings (key-value)
CREATE TABLE settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Completed pomodoro sessions
CREATE TABLE sessions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at    INTEGER NOT NULL,  -- Unix timestamp (seconds)
    ended_at      INTEGER,           -- NULL if abandoned
    round_type    TEXT NOT NULL,     -- 'work', 'short_break', 'long_break'
    duration_secs INTEGER NOT NULL,  -- configured duration
    completed     INTEGER NOT NULL DEFAULT 0  -- 1 if finished, 0 if skipped/reset
);

-- Themes (user-defined; built-in themes are bundled as files)
CREATE TABLE custom_themes (
    id     INTEGER PRIMARY KEY AUTOINCREMENT,
    name   TEXT NOT NULL UNIQUE,
    colors TEXT NOT NULL  -- JSON blob of 10 color values
);

-- Schema version for migrations
CREATE TABLE schema_version (
    version INTEGER NOT NULL
);
INSERT INTO schema_version VALUES (1);
```

**Settings keys** (stored as TEXT, deserialized in Rust):
```
alwaysOnTop, breakAlwaysOnTop, autoStartWorkTimer, autoStartBreakTimer,
minToTray, minToTrayOnClose, notifications, workRounds, theme,
tickSoundsWork, tickSoundsBreak, timeLongBreak, timeShortBreak,
timeWork, volume, shortcutToggle, shortcutReset, shortcutSkip
```

**Migration strategy**: On startup, read `schema_version`. Apply migrations in sequence. Migrations are embedded Rust strings/constants.

---

## 5. System Tray, Notifications, and OS Integrations

### System Tray — `tauri-plugin-tray-icon`
- Create/destroy tray icon dynamically based on the `minToTray` setting.
- Tray icon image: render a PNG in Rust using the `image` crate (or `tiny-skia` for arc drawing).
- Context menu: "Show" / "Exit" via Tauri's menu API.
- Click handler: toggle window visibility.
- macOS positioning: position window near cursor/tray on toggle (Tauri exposes `window.set_position()`).
- Update tray arc at most once per second (throttled to match timer ticks).

**Alternative (simpler)**: Use a static icon set (full/paused/break states) rather than a dynamically rendered arc. This eliminates the need for image rendering in Rust and is more consistent across OS DPI settings.

### Desktop Notifications — `tauri-plugin-notification`
- Single unified API across Windows, macOS, Linux.
- No platform-specific branching needed (eliminates `node-notifier` vs Web Notification divergence).
- On round transitions, send notification with round-specific title and body.
- Custom icons per round type (work, short break, long break) via bundled assets.

### Global Shortcuts — `tauri-plugin-global-shortcut`
- Register/unregister shortcuts via Rust commands.
- Shortcut events routed to the Rust backend first, then forwarded to frontend as Tauri events if needed.
- Persist shortcut configuration in the `settings` table.

### Window Management
- Tauri supports `decorations: false` (frameless window) via `tauri.conf.json`.
- Fixed window size: `width: 360, height: 478`, `resizable: false`.
- `always_on_top` configurable at runtime via `window.set_always_on_top()`.
- Minimize to tray: call `window.hide()` instead of `window.minimize()`.
- Custom titlebar in Svelte (drag region via `data-tauri-drag-region` attribute).

### WebSocket Server
- Implement in Rust using `tokio` + `axum` (WebSocket upgrade).
- Same protocol as existing: port 1314, JSON messages, `getState` / `roundChange` events.
- Run in the same Tauri process, but on a separate async task.
- Add configurable port or disable option (currently always-on, even if not used).

---

## 6. Feature Reconsideration

### Keep As-Is
- All timer settings (durations, rounds count, 1-90 min range, 1-12 rounds range)
- All toggle settings (always-on-top, auto-start, tick sounds, notifications, tray)
- Theme system (built-in themes + custom JSON themes in userData)
- Global keyboard shortcuts (3 configurable shortcuts)
- Audio cues (4 sounds, per-phase tick toggle, volume control)
- WebSocket server (external integration point, valuable for power users)
- Fixed window size and frameless design

### Improve
- **Timer accuracy**: Move to Rust monotonic clock (non-negotiable).
- **Session persistence**: Store completed rounds in SQLite. Show total rounds completed this session AND all-time.
- **Custom theme hot-reload**: Watch the themes directory for new files and reload without restart.
- **Volume slider**: Replace hardcoded pixel bounds with a proper hover/blur detection in Svelte.
- **Notification unification**: Single code path for all platforms.
- **`breakAlwaysOnTop` logic**: Implement as pure state, not DOM manipulation.

### Reconsider / Possibly Remove
- **WebSocket server always-on**: Consider making this opt-in (default: off) since it binds a port on all interfaces and has no authentication. Power users can enable it. **Requires decision.**
- **Logging with daily rotation**: Tauri has built-in log handling via `tauri-plugin-log`. Consider simplifying to a single rolling log rather than the Winston daily-rotate setup with its 14-day retention.
- **`totalWorkRounds` session-only display**: Either persist it (via SQLite sessions table) or remove the counter. A non-persisted counter of limited value adds confusion.
- **Auto-updater**: The existing code is dead. Consider implementing properly with Tauri's updater plugin, or explicitly exclude it from scope.

### Do Not Replicate
- `vue-electron` bridge (not applicable in Svelte/Tauri)
- Dual notification implementations (Windows vs. non-Windows)
- `setInterval`-based timer (replaced by Rust thread)
- `requestAnimationFrame`-based timer (replaced by Rust events)
- Hardcoded pixel bounds for volume slider timeout

---

## 7. Risks and Open Questions

### OQ-1: Timer Behavior on System Sleep/Hibernate
**Question**: If the OS sleeps while the timer is running and wakes up 30 minutes later, should the timer:
- (a) Complete immediately (and trigger round change) because the elapsed time exceeds the total duration
- (b) Pause automatically on sleep and resume on wake
- (c) Skip to the correct elapsed position (e.g., if a 25-min timer had 5 min left, it completes on wake)

Option (a) or (c) is technically correct but may be surprising. Option (b) requires OS sleep/wake detection, which is possible in Tauri via power events. **Human decision required.**

**Answer**: Option (b)

### OQ-2: Session Data / Statistics
**Question**: The existing app shows `totalWorkRounds` for the current session only. Should the rewrite:
- (a) Only show session total (same as existing, but persisted)
- (b) Show all-time totals via SQLite
- (c) Add a statistics/history view (out of scope for MVP, but schema should support it)

If (b) or (c), this implies a UI surface that doesn't exist today. **Human decision required.**

**Answer** Option (c)

### OQ-3: WebSocket Server Opt-In vs. Always-On
**Question**: The current WebSocket server starts unconditionally on port 1314 with no auth. Should the rewrite:
- (a) Keep it always-on (no change in behavior, but documented)
- (b) Make it opt-in via a settings toggle
- (c) Make the port configurable

**Human decision required.**

**Answer** I am open to suggestions on the best-practice approach for this.

### OQ-4: Tray Icon Rendering
**Question**: The current tray icon is a dynamically rendered arc (canvas in JS). For the rewrite:
- (a) Continue dynamic arc rendering in Rust (using `tiny-skia` or similar)
- (b) Use static icon set: full circle, half circle, pause bars, etc. (simpler, no rendering library)
- (c) Use a CSS/SVG-rendered icon sent as a data URL from the webview (mirrors current approach)

Option (b) is much simpler to implement and maintain. Option (a) preserves the current behavior exactly. **Human decision required.**

**Answer** Option (a) is preferable.

### OQ-5: Frontend Framework — Svelte vs. React
The brief specifies "Svelte (preferred) or React." This proposal assumes **Svelte** because:
- Zero-runtime overhead (compiled to vanilla JS)
- Built-in reactivity without a state library
- Simpler bundle for a small app like this
- CSS-in-component scoping aligns well with the existing Vue SFC approach

If React is required for team familiarity or other reasons, the architecture is the same — only the component implementation changes. **Human confirmation recommended.**

**Answer** Svelte is preferred, but React is a possiblity due to its ubiquity.

### OQ-6: Restart Required for Custom Themes
The current app requires a restart to pick up new custom theme JSON files. Should the rewrite implement:
- (a) Directory watcher (e.g., `notify` crate) to hot-reload themes on file change
- (b) A "Reload Themes" button in the UI
- (c) Keep requiring restart (simpler, acceptable for rare operation)

**Answer** Option (a)

### OQ-7: Global Shortcut Format Migration
Electron accelerator format (`Control+F1`) differs from Tauri's shortcut format (varies by platform). Stored shortcut strings will need migration or conversion. The rewrite should define the canonical format and provide a migration path for existing `user-preferences.json` (if migrating existing user data).

**Question**: Should the rewrite import existing settings from the old JSON file on first launch, or start fresh?

**Answer** Start fresh.

### OQ-8: Audio Engine
The existing app uses HTML `<audio>` elements loaded from static file paths. Tauri's webview supports the Web Audio API and `<audio>` elements. However:
- (a) Continue using `<audio>` elements in the Svelte frontend (simplest, no change)
- (b) Use Rust audio playback (`rodio` crate) for guaranteed playback even when window is hidden

Option (b) is technically superior (audio plays even if webview is backgrounded/hidden) but adds complexity. **Human decision recommended.**

**Answer** Option (b)

### OQ-9: Frameless Window on Linux
Tauri's frameless window (`decorations: false`) can behave inconsistently across Linux desktop environments (X11 vs. Wayland, GNOME vs. KDE). The existing app uses `-webkit-app-region: drag` which also has Wayland implications. Testing across Linux DEs is required. The custom titlebar drag region should be tested specifically.

**Answer** Let's proceed with the best-supported implementation, IE default behavior for each system type. We can build in custom window treatments after the MVP.

### OQ-10: Minimum Platform Targets
The brief specifies Windows, macOS, Linux (x64 and ARM). Specifically:
- Windows: x64 + ARM64 (Windows on ARM)?
- macOS: x64 + Apple Silicon (M-series)?
- Linux: x64 + ARM64 (Raspberry Pi? server?)?

Tauri supports all these targets. CI/CD configuration (cross-compilation or separate runners) needs to be scoped. **Human clarification requested.**

**Answer** For the moment, let's build for Linux x64 (development system), and we will add targets in a later phase.

---

## 8. Migration Path for Existing Users

If existing user settings should be preserved on first launch:
1. Check for `{userData}/user-preferences.json` (old format).
2. If found, read and import into the new SQLite settings table.
3. Rename/delete the old file to prevent repeated imports.

This is a one-time migration. Custom theme files in `{userData}/themes/` can be read directly by the new app with no migration needed (same JSON format).
