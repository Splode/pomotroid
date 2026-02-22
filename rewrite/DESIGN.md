# Technical Design: Pomotroid Rewrite

> Tauri 2.x + Rust + Svelte 5 + SQLite (rusqlite)

---

## 1. Project Structure

```
pomotroid-v2/
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── icons/                    # App icons (all sizes, all platforms)
│   │   ├── icon.png
│   │   ├── icon.ico
│   │   ├── icon.icns
│   │   └── tray/
│   │       ├── tray-work.png
│   │       ├── tray-short-break.png
│   │       ├── tray-long-break.png
│   │       └── tray-paused.png
│   └── src/
│       ├── main.rs               # Entry point, Tauri app setup
│       ├── lib.rs                # Public Tauri command registrations
│       ├── timer/
│       │   ├── mod.rs            # Timer state machine, public API
│       │   ├── engine.rs         # Rust timer thread, monotonic clock
│       │   └── sequence.rs       # Pomodoro round sequencing logic
│       ├── settings/
│       │   ├── mod.rs            # Settings struct, read/write
│       │   └── defaults.rs       # Default values
│       ├── db/
│       │   ├── mod.rs            # DB connection pool, init
│       │   ├── migrations.rs     # Embedded SQL migrations
│       │   └── queries.rs        # All SQL queries
│       ├── audio/
│       │   └── mod.rs            # Rust audio playback (rodio)
│       ├── themes/
│       │   ├── mod.rs            # Theme loading (bundled + custom)
│       │   └── watcher.rs        # notify crate directory watcher (required)
│       ├── tray/
│       │   └── mod.rs            # Tray icon + tiny-skia arc rendering
│       ├── shortcuts/
│       │   └── mod.rs            # Global shortcut register/unregister
│       ├── notifications/
│       │   └── mod.rs            # Desktop notification dispatch
│       ├── websocket/
│       │   └── mod.rs            # WebSocket server (tokio + axum)
│       └── commands.rs           # All #[tauri::command] definitions
│
├── src/                          # Svelte frontend
│   ├── app.html                  # HTML entry point
│   ├── app.css                   # Global CSS (CSS custom properties)
│   ├── lib/
│   │   ├── stores/
│   │   │   ├── timer.ts          # Timer state (reactive)
│   │   │   ├── settings.ts       # Settings state (reactive)
│   │   │   └── theme.ts          # Active theme state
│   │   ├── audio/                # REMOVED — audio is in Rust (rodio)
│   │   │   └── AudioManager.ts   # Audio playback coordinator
│   │   ├── ipc/
│   │   │   └── index.ts          # Tauri invoke/listen wrappers (typed)
│   │   └── types.ts              # Shared TypeScript types
│   └── routes/
│       └── +page.svelte          # Single page (SPA, no routing needed)
│
├── static/
│   ├── audio/
│   │   ├── alert-work.mp3
│   │   ├── alert-short-break.mp3
│   │   ├── alert-long-break.mp3
│   │   └── tick.mp3
│   └── themes/                   # Built-in theme JSON files (17 themes)
│       ├── pomotroid.json
│       ├── dracula.json
│       └── ...
│
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
└── package.json
```

---

## 2. Rust Module Breakdown

### 2.1 `timer/engine.rs` — The Core Timer Thread

Responsibilities:
- Spawn and manage a dedicated OS thread for the countdown.
- Use `std::time::Instant` for monotonic elapsed measurement.
- Send `TimerEvent` messages to the main thread via `mpsc::Sender`.
- Accept `TimerCommand` messages via `mpsc::Receiver` (start, pause, resume, reset, skip).
- Implement drift correction: sleep until the next scheduled tick, not for a fixed duration.

```rust
pub enum TimerCommand {
    Start,
    Pause,
    Resume,
    Reset,
    Skip,        // triggers Complete immediately
    Reconfigure { duration_secs: u32 },
}

pub enum TimerEvent {
    Tick { elapsed_secs: u32, total_secs: u32 },
    Complete,
    Paused,
    Resumed,
    Reset,
    StateChanged { state: TimerEngineState },
}

#[derive(Clone, Serialize)]
pub enum TimerEngineState {
    Idle,
    Running,
    Paused,
}
```

### 2.2 `timer/sequence.rs` — Pomodoro Round Sequencing

Responsibilities:
- Track `current_round: RoundType`, `round_index: u32`, `work_rounds: u32`.
- Compute the next round when `Complete` fires.
- Determine whether to auto-start the next timer.
- Emit `SequenceEvent` to the command layer.

```rust
pub enum RoundType {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct SequenceState {
    pub current_round: RoundType,
    pub round_index: u32,        // 1-based, resets after long break
    pub work_rounds_configured: u32,
    pub total_work_rounds: u32,  // session total
}
```

### 2.3 `timer/mod.rs` — Unified Timer Controller

Responsibilities:
- Owns both the engine (mpsc channel pair) and the sequence state.
- Translates engine events into Tauri events emitted to the frontend.
- Coordinates between engine commands and sequence advancement.
- Exposes a simple public API called by `commands.rs`.

```rust
pub struct TimerController {
    cmd_tx: mpsc::Sender<TimerCommand>,
    state: Arc<Mutex<SequenceState>>,
    settings: Arc<Mutex<Settings>>,
}
```

### 2.4 `settings/mod.rs` — Settings Management

Responsibilities:
- Read settings from SQLite on startup.
- Provide a `Settings` struct with all fields typed.
- Write individual settings on change.
- Apply defaults for missing keys on first launch.
- Detect and import legacy `user-preferences.json` if present.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub always_on_top: bool,
    pub break_always_on_top: bool,
    pub auto_start_work: bool,
    pub auto_start_break: bool,
    pub min_to_tray: bool,
    pub min_to_tray_on_close: bool,
    pub notifications: bool,
    pub work_rounds: u32,
    pub theme: String,
    pub tick_sounds_work: bool,
    pub tick_sounds_break: bool,
    pub time_work_mins: u32,
    pub time_short_break_mins: u32,
    pub time_long_break_mins: u32,
    pub volume: u32,   // 0–100
    pub shortcut_toggle: String,
    pub shortcut_reset: String,
    pub shortcut_skip: String,
    pub websocket_enabled: bool,
    pub websocket_port: u16,
}
```

### 2.5 `db/mod.rs` — Database

Responsibilities:
- Open/create the SQLite file at `{app_data_dir}/pomotroid.db`.
- Run migrations on startup.
- Provide `Connection` wrapped in `Arc<Mutex<Connection>>` for thread-safe access.

### 2.6 `db/migrations.rs`

Embedded SQL constants applied in sequence based on `schema_version`:

```rust
const MIGRATION_1: &str = "
CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at INTEGER NOT NULL,
    ended_at INTEGER,
    round_type TEXT NOT NULL,
    duration_secs INTEGER NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE custom_themes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    colors TEXT NOT NULL
);
CREATE TABLE schema_version (version INTEGER NOT NULL);
INSERT INTO schema_version VALUES (1);
";
```

### 2.7 `themes/mod.rs`

Responsibilities:
- Load built-in themes from the bundled `static/themes/` directory (via `include_str!` or runtime resource path).
- Load custom themes from `{app_data_dir}/themes/`.
- Parse and validate theme JSON.
- Return `Vec<Theme>` to the frontend on demand.
- Optionally: watch `{app_data_dir}/themes/` with the `notify` crate and emit a `themes-changed` event.

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThemeColors {
    pub color_long_round: String,
    pub color_short_round: String,
    pub color_focus_round: String,
    pub color_background: String,
    pub color_background_light: String,
    pub color_background_lightest: String,
    pub color_foreground: String,
    pub color_foreground_darker: String,
    pub color_foreground_darkest: String,
    pub color_accent: String,
}
```

### 2.8 `tray/mod.rs`

Responsibilities:
- Create/destroy the system tray based on the `min_to_tray` setting.
- Set tray icon from a static PNG asset based on current round type and paused state.
- Build context menu (Show / Exit).
- Handle click event (toggle window visibility).
- macOS: position window near tray on show.

### 2.9 `shortcuts/mod.rs`

Responsibilities:
- Register global shortcuts using Tauri's global shortcut plugin.
- On shortcut fire, invoke the appropriate timer command via `TimerController`.
- Re-register all shortcuts when configuration changes.

### 2.10 `notifications/mod.rs`

Responsibilities:
- Send desktop notification using Tauri's notification plugin.
- Construct message based on round type and configured durations.
- Respect the `notifications` setting.

### 2.11 `websocket/mod.rs`

Responsibilities:
- Spawn a `tokio` runtime (or reuse Tauri's) with an `axum` WebSocket handler.
- Bind to `127.0.0.1:{port}` (localhost only, not all interfaces — security improvement).
- Broadcast `roundChange` events to all connected clients.
- Handle `getState` requests.
- Gracefully shut down when the app exits.

### 2.12 `commands.rs` — Tauri IPC Layer

All `#[tauri::command]` functions callable from the frontend. See Section 4.

---

## 3. Tauri Command Definitions (IPC Layer)

All commands are async and return `Result<T, String>`.

### 3.1 Timer Commands

```rust
// Start the current round's timer
#[tauri::command]
async fn timer_start(state: State<'_, TimerController>) -> Result<(), String>

// Pause the running timer
#[tauri::command]
async fn timer_pause(state: State<'_, TimerController>) -> Result<(), String>

// Resume a paused timer
#[tauri::command]
async fn timer_resume(state: State<'_, TimerController>) -> Result<(), String>

// Reset the current round's timer (do not advance round)
#[tauri::command]
async fn timer_reset(state: State<'_, TimerController>) -> Result<(), String>

// Skip the current round (treat as complete, advance to next)
#[tauri::command]
async fn timer_skip(state: State<'_, TimerController>) -> Result<(), String>

// Get current timer and sequence state snapshot
#[tauri::command]
async fn timer_get_state(state: State<'_, TimerController>) -> Result<TimerSnapshot, String>
```

### 3.2 Settings Commands

```rust
// Get all settings
#[tauri::command]
async fn settings_get(state: State<'_, AppDb>) -> Result<Settings, String>

// Update one or more settings
#[tauri::command]
async fn settings_set(state: State<'_, AppDb>, updates: SettingsUpdate) -> Result<(), String>

// Reset all timer-related settings to defaults
#[tauri::command]
async fn settings_reset_defaults(state: State<'_, AppDb>) -> Result<Settings, String>
```

### 3.3 Theme Commands

```rust
// Get all available themes (built-in + custom)
#[tauri::command]
async fn themes_list(state: State<'_, ThemeManager>) -> Result<Vec<Theme>, String>

// Apply a theme by name (persists selection to settings)
#[tauri::command]
async fn theme_apply(
    name: String,
    state: State<'_, ThemeManager>,
    db: State<'_, AppDb>
) -> Result<Theme, String>
```

### 3.4 Window Commands

```rust
// Show/hide window programmatically
#[tauri::command]
async fn window_set_visibility(visible: bool, window: Window) -> Result<(), String>

// Apply always-on-top setting to window
#[tauri::command]
async fn window_set_always_on_top(always_on_top: bool, window: Window) -> Result<(), String>
```

### 3.5 Shortcut Commands

```rust
// Update global shortcuts configuration
#[tauri::command]
async fn shortcuts_set(
    shortcuts: ShortcutConfig,
    state: State<'_, ShortcutManager>
) -> Result<(), String>
```

### 3.6 Data Commands

```rust
// Get session statistics
#[tauri::command]
async fn stats_get_session(db: State<'_, AppDb>) -> Result<SessionStats, String>

// Get all-time statistics
#[tauri::command]
async fn stats_get_all_time(db: State<'_, AppDb>) -> Result<AllTimeStats, String>
```

### Tauri Events (Rust → Frontend)

These are emitted from Rust and listened to in Svelte via `listen()`:

| Event Name | Payload | Trigger |
|---|---|---|
| `timer:tick` | `{ elapsed_secs: u32, total_secs: u32 }` | Every second the timer is running |
| `timer:complete` | `{ round_type: string }` | Timer finished naturally |
| `timer:paused` | `{ elapsed_secs: u32 }` | Pause acknowledged |
| `timer:resumed` | `{ elapsed_secs: u32 }` | Resume acknowledged |
| `timer:reset` | none | Reset acknowledged |
| `timer:round-change` | `RoundChangePayload` | Next round ready |
| `timer:suspended` | none | OS sleep detected; timer paused |
| `settings:changed` | `Settings` | Any setting was updated |
| `themes:changed` | `Vec<Theme>` | Theme files reloaded (hot-reload) |
| `shortcut:fired` | `{ action: string }` | Global shortcut activated |
| `websocket:error` | `{ message: string }` | WebSocket server failed to bind |

```rust
#[derive(Serialize, Clone)]
pub struct RoundChangePayload {
    pub round_type: String,      // "work" | "short_break" | "long_break"
    pub round_index: u32,
    pub work_rounds: u32,
    pub total_work_rounds: u32,
    pub auto_start: bool,
}

#[derive(Serialize, Clone)]
pub struct TimerSnapshot {
    pub state: String,           // "idle" | "running" | "paused"
    pub round_type: String,
    pub round_index: u32,
    pub work_rounds: u32,
    pub elapsed_secs: u32,
    pub total_secs: u32,
    pub total_work_rounds: u32,
}
```

---

## 4. SQLite Schema (Full Table Definitions)

```sql
-- Schema Version 1

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at    INTEGER NOT NULL,
    ended_at      INTEGER,
    round_type    TEXT NOT NULL CHECK(round_type IN ('work', 'short_break', 'long_break')),
    duration_secs INTEGER NOT NULL CHECK(duration_secs > 0),
    completed     INTEGER NOT NULL DEFAULT 0 CHECK(completed IN (0, 1))
);

CREATE TABLE IF NOT EXISTS custom_themes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    colors      TEXT NOT NULL  -- JSON blob
);

CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_sessions_round_type ON sessions(round_type);
```

### Default Settings Seed

```sql
INSERT OR IGNORE INTO settings VALUES ('always_on_top', 'false');
INSERT OR IGNORE INTO settings VALUES ('break_always_on_top', 'false');
INSERT OR IGNORE INTO settings VALUES ('auto_start_work', 'true');
INSERT OR IGNORE INTO settings VALUES ('auto_start_break', 'true');
INSERT OR IGNORE INTO settings VALUES ('min_to_tray', 'false');
INSERT OR IGNORE INTO settings VALUES ('min_to_tray_on_close', 'false');
INSERT OR IGNORE INTO settings VALUES ('notifications', 'true');
INSERT OR IGNORE INTO settings VALUES ('work_rounds', '4');
INSERT OR IGNORE INTO settings VALUES ('theme', 'Pomotroid');
INSERT OR IGNORE INTO settings VALUES ('tick_sounds_work', 'false');
INSERT OR IGNORE INTO settings VALUES ('tick_sounds_break', 'true');
INSERT OR IGNORE INTO settings VALUES ('time_work_mins', '25');
INSERT OR IGNORE INTO settings VALUES ('time_short_break_mins', '5');
INSERT OR IGNORE INTO settings VALUES ('time_long_break_mins', '15');
INSERT OR IGNORE INTO settings VALUES ('volume', '100');
INSERT OR IGNORE INTO settings VALUES ('shortcut_toggle', 'Control+F1');
INSERT OR IGNORE INTO settings VALUES ('shortcut_reset', 'Control+F2');
INSERT OR IGNORE INTO settings VALUES ('shortcut_skip', 'Control+F3');
INSERT OR IGNORE INTO settings VALUES ('websocket_enabled', 'false');
INSERT OR IGNORE INTO settings VALUES ('websocket_port', '1314');
```

---

## 5. Svelte Component Hierarchy

```
+page.svelte                        (root, single route)
├── Titlebar.svelte                 (draggable, minimize/close)
├── Drawer.svelte                   (slide-in overlay)
│   ├── DrawerMenu.svelte           (tab navigation: Timer|Settings|Themes|About)
│   ├── DrawerTimer.svelte          (duration sliders, rounds slider, reset defaults)
│   ├── DrawerSettings.svelte       (all boolean toggles + shortcut inputs)
│   │   └── ShortcutInput.svelte    (captures modifier+key, emits formatted string)
│   ├── DrawerTheme.svelte          (theme list, current selection indicator)
│   └── DrawerAbout.svelte          (version, links)
├── Timer.svelte                    (orchestrator, listens to all timer events)
│   ├── TimerDial.svelte            (SVG arc, Svelte tweened animation)
│   ├── TimerDisplay.svelte         (MM:SS text display)
│   ├── TimerControls.svelte        (play/pause/resume button)
│   └── TimerFooter.svelte          (round counter, reset, skip, volume)
│       └── VolumeControl.svelte    (mute button + slider)
├── AudioManager.svelte             (hidden, HTML audio elements, event-driven)
└── NotificationManager.svelte      (hidden, listens for round-change events)
```

### Component Responsibilities

**`+page.svelte`**
- Bootstraps app on mount: loads settings, loads themes, subscribes to all Rust events.
- Applies active theme CSS variables.
- Renders conditional components (drawer visibility).

**`Titlebar.svelte`**
- `data-tauri-drag-region` on the nav element.
- Hamburger icon (toggle drawer).
- Minimize/Close buttons invoke Tauri commands.
- Close behavior: respects `minToTrayOnClose` from settings store.

**`Drawer.svelte`**
- Slide-in animation via Svelte transitions.
- Contains `DrawerMenu` (always visible) and the active tab content.
- Active tab driven by a local `currentTab` variable.

**`Timer.svelte`**
- Primary event hub for timer state.
- Subscribes to `timer:tick`, `timer:paused`, `timer:resumed`, `timer:reset`, `timer:round-change`.
- Maintains a `$timerStore` with current display state.
- Passes elapsed/total to `TimerDial` and `TimerDisplay`.

**`TimerDial.svelte`**
- Receives `elapsed`, `total`, `roundType`, `isActive` as props.
- Uses Svelte's `tweened` store for smooth arc animation (CSS `stroke-dashoffset`).
- On `elapsed` prop change, updates the tweened value.
- Color class determined by `roundType`.
- No anime.js dependency — Svelte's built-in interpolation handles this.

**`TimerDisplay.svelte`**
- Receives `elapsed`, `total`, `hasStarted` as props.
- Computes `remainingMins` and `remainingSecs` from `total - elapsed`.
- Formats as `MM:SS` with zero-padding.
- Shows `MM:00` when `!hasStarted`.

**`TimerFooter.svelte`**
- Round counter: `{roundIndex}/{workRounds}` + `(totalWorkRounds)` in dimmer text.
- Reset button: calls `timer_reset` command.
- Skip button: calls `timer_skip` command.

**`VolumeControl.svelte`**
- Mute icon button: toggles volume between 0 and last non-zero value.
- Hover reveals a range slider (uses Svelte `on:mouseenter`/`on:mouseleave` — no hardcoded pixels).
- Slider change dispatches `settings_set({ volume })`.

**`AudioManager.svelte`**
- Hidden component with 4 `<audio>` elements.
- Subscribes to `timer:round-change` event from Rust.
- Plays appropriate sound based on new round type.
- Tick sound: played on `timer:tick` event if tick sounds are enabled for the current round type.
- Volume applied from settings store.

**`ShortcutInput.svelte`**
- `<input type="text" readonly>` shows current shortcut.
- On focus + keyup: captures modifier state + key, formats as Tauri accelerator string.
- Emits the formatted string to parent.
- Handles modifier-only keydown without registering (same as existing).

---

## 6. State Management

Svelte stores replace Vuex. No external state library needed.

### `lib/stores/timer.ts`

```typescript
import { writable, derived } from 'svelte/store';

export interface TimerState {
    engineState: 'idle' | 'running' | 'paused';
    roundType: 'work' | 'short_break' | 'long_break';
    roundIndex: number;
    workRounds: number;
    elapsedSecs: number;
    totalSecs: number;
    totalWorkRounds: number;
    hasStarted: boolean;
}

export const timerState = writable<TimerState>({ /* defaults */ });

// Derived: remaining seconds
export const remainingSecs = derived(timerState, $t => $t.totalSecs - $t.elapsedSecs);

// Derived: progress ratio (0.0 → 1.0)
export const progress = derived(timerState, $t =>
    $t.totalSecs > 0 ? $t.elapsedSecs / $t.totalSecs : 0
);
```

### `lib/stores/settings.ts`

```typescript
export const settings = writable<Settings>(defaultSettings);

// Sync settings to Rust on change (debounced for slider values)
settings.subscribe(async ($s) => {
    // batched update to avoid per-keystroke writes
});
```

### `lib/stores/theme.ts`

```typescript
export const themes = writable<Theme[]>([]);
export const activeTheme = writable<Theme>(defaultTheme);

// Apply theme colors to document root CSS variables
activeTheme.subscribe($theme => {
    applyTheme($theme);
});
```

### Event Flow

```
Rust timer thread
    │ mpsc::Sender<TimerEvent>
    ▼
TimerController (Rust)
    │ app.emit("timer:tick", payload)
    ▼
Tauri WebView
    │ listen("timer:tick", handler)
    ▼
Timer.svelte
    │ timerState.update(...)
    ▼
TimerDial.svelte (reactive to timerState)
TimerDisplay.svelte (reactive to timerState)
TimerFooter.svelte (reactive to timerState)
// Audio is in Rust (rodio) — no AudioManager.svelte
// On timer:round-change: Rust plays alert sound directly
// On timer:tick: Rust plays tick sound directly
```

---

## 7. Build and Packaging Configuration

### `tauri.conf.json` (key sections)

```json
{
  "productName": "Pomotroid",
  "version": "1.0.0",
  "identifier": "com.splode.pomotroid",
  "build": {
    "frontendDist": "../build",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev"
  },
  "app": {
    "windows": [
      {
        "title": "Pomotroid",
        "width": 360,
        "height": 478,
        "resizable": false,
        "fullscreen": false,
        "decorations": false,
        "alwaysOnTop": false,
        "center": true,
        "backgroundColor": "#2F384B"
      }
    ],
    "security": {
      "csp": "default-src 'self'; media-src 'self'",
      "dangerousRemoteDomainIpcAccess": []
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/icon.png", "icons/icon.ico", "icons/icon.icns"],
    "category": "Utility",
    "shortDescription": "A simple and visually-pleasing Pomodoro timer",
    "macOS": {
      "minimumSystemVersion": "10.15"
    },
    "windows": {
      "nsis": { "installMode": "currentUser" }
    },
    "linux": {
      "deb": { "depends": [] }
    }
  }
}
```

### `Cargo.toml` (key dependencies)

```toml
[package]
name = "pomotroid"
version = "1.0.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = ["tray-icon", "global-shortcut", "notification"] }
tauri-plugin-log = "2"
tauri-plugin-notification = "2"
tauri-plugin-global-shortcut = "2"
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
axum = { version = "0.7", features = ["ws"] }            # WebSocket server
rodio = "0.17"                                            # Audio playback (OQ-8)
tiny-skia = "0.11"                                        # Tray icon arc rendering (OQ-4)
notify = "6"                                              # Theme directory watcher (OQ-6)
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### Target Matrix

**MVP scope: Linux x64 only** (per OQ-10). Additional targets added in a future phase.

| Phase | Platform | Architecture | Build Method |
|---|---|---|---|
| MVP | Linux | x64 | GitHub Actions Ubuntu runner |
| Phase 2 | Windows | x64 | GitHub Actions Windows runner |
| Phase 2 | macOS | Universal (x64 + ARM64) | GitHub Actions macOS runner |
| Future | Linux | ARM64 | QEMU or native runner |
| Future | Windows | ARM64 | Cross-compile or ARM64 runner |

### CI Pipeline (GitHub Actions) — MVP

```yaml
jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: |
          sudo apt-get install -y libwebkit2gtk-4.1-dev libssl-dev \
            libappindicator3-dev librsvg2-dev patchelf
      - run: npm ci && npm run tauri build
```

### Output Artifacts

**MVP**: Linux only.

| Platform | Format |
|---|---|
| Linux (MVP) | `.deb` + `.AppImage` |
| Windows (Phase 2) | NSIS installer (`.exe`) + portable `.exe` |
| macOS (Phase 2) | `.dmg` + `.app` (universal binary) |

### Audio: Embedded Assets (rodio)

Audio files are embedded at compile time to avoid runtime path resolution across platforms:

```rust
// audio/mod.rs
const ALERT_WORK: &[u8] = include_bytes!("../../../static/audio/alert-work.mp3");
const ALERT_SHORT_BREAK: &[u8] = include_bytes!("../../../static/audio/alert-short-break.mp3");
const ALERT_LONG_BREAK: &[u8] = include_bytes!("../../../static/audio/alert-long-break.mp3");
const TICK: &[u8] = include_bytes!("../../../static/audio/tick.mp3");

pub enum AudioCue { WorkAlert, ShortBreakAlert, LongBreakAlert, Tick }

pub struct AudioManager {
    stream: OutputStream,         // must stay alive
    handle: OutputStreamHandle,
    volume: f32,                  // 0.0–1.0
}

impl AudioManager {
    pub fn play(&self, cue: AudioCue) {
        let data = match cue {
            AudioCue::WorkAlert      => ALERT_WORK,
            AudioCue::ShortBreakAlert => ALERT_SHORT_BREAK,
            AudioCue::LongBreakAlert => ALERT_LONG_BREAK,
            AudioCue::Tick           => TICK,
        };
        let cursor = std::io::Cursor::new(data);
        let source = rodio::Decoder::new(cursor).unwrap();
        let sink = rodio::Sink::try_new(&self.handle).unwrap();
        sink.set_volume(self.volume);
        sink.append(source);
        sink.detach(); // play to completion without blocking
    }
}
```

### Sleep/Wake Handling

OS power events are detected via Tauri's system event listener (available on all platforms via the underlying Wry/WebView layer or an OS-level API):

```rust
// main.rs (inside app setup)
app.on_system_event(|event| {
    match event {
        SystemEvent::Sleep => {
            // Send Suspend command to timer engine
            timer_controller.suspend();
        }
        SystemEvent::Wake => {
            // Send WakeResume command to timer engine
            timer_controller.wake_resume();
        }
        _ => {}
    }
});
```

If Tauri does not expose a sufficient power event API directly, use the `system-events` crate or platform-specific mechanisms. This is a known gap to investigate during TIMER-01 implementation.

---

## 8. CSS / Theme Architecture

The theme system in Svelte mirrors the existing CSS custom property approach.

### Global CSS (app.css)

```css
:root {
  /* Default Pomotroid theme values — overridden by active theme */
  --color-long-round: #0bbddb;
  --color-short-round: #05ec8c;
  --color-focus-round: #ff4e4d;
  --color-background: #2f384b;
  --color-background-light: #3d4457;
  --color-background-lightest: #9ca5b5;
  --color-foreground: #f6f2eb;
  --color-foreground-darker: #c0c9da;
  --color-foreground-darkest: #dbe1ef;
  --color-accent: #05ec8c;
}

/* Fonts */
@font-face {
  font-family: 'Lato';
  src: url('/fonts/Lato-Regular.ttf');
}

@font-face {
  font-family: 'RobotoMono';
  src: url('/fonts/RobotoMono-Light.ttf');
}
```

### Theme Application (TypeScript)

```typescript
export function applyTheme(theme: Theme): void {
    const root = document.documentElement;
    const c = theme.colors;
    root.style.setProperty('--color-long-round', c.color_long_round);
    root.style.setProperty('--color-short-round', c.color_short_round);
    root.style.setProperty('--color-focus-round', c.color_focus_round);
    root.style.setProperty('--color-background', c.color_background);
    root.style.setProperty('--color-background-light', c.color_background_light);
    root.style.setProperty('--color-background-lightest', c.color_background_lightest);
    root.style.setProperty('--color-foreground', c.color_foreground);
    root.style.setProperty('--color-foreground-darker', c.color_foreground_darker);
    root.style.setProperty('--color-foreground-darkest', c.color_foreground_darkest);
    root.style.setProperty('--color-accent', c.color_accent);
}
```

### TimerDial Animation (Svelte `tweened`)

```svelte
<script lang="ts">
    import { tweened } from 'svelte/motion';
    import { linear } from 'svelte/easing';

    export let progress: number = 0; // 0.0 (empty) → 1.0 (full)
    export let roundType: string;

    const CIRCUMFERENCE = 2 * Math.PI * 110; // path radius ≈ 110

    // Svelte tweened store handles smooth interpolation
    const offset = tweened(0, { duration: 200, easing: linear });

    $: offset.set(CIRCUMFERENCE * (1 - progress));
</script>

<svg class="dial-fill {roundType}" viewBox="0 0 230 230">
    <path
        stroke-dasharray={CIRCUMFERENCE}
        stroke-dashoffset={$offset}
        ...
    />
</svg>
```

This replaces anime.js entirely. The `tweened` store smoothly interpolates between tick updates.
