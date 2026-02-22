# Implementation Task List

> Sequenced and grouped by phase. Each task is independently completable and verifiable.
> Complexity: S = Small (< 2 hrs), M = Medium (2–6 hrs), L = Large (6+ hrs)

---

## Legend

- `[HUMAN]` — Requires a decision or approval before proceeding. See PROPOSAL.md Open Questions.
- `[DEPENDS: X]` — Cannot begin until task X is complete.
- `[BLOCKS: X]` — This task must complete before X can begin.
- Task status in headers: **`✓ DONE`** · **`▶ IN PROGRESS`** · *(unmarked = TODO)*

---

## Progress Tracker

| Task | Title | Status | Notes |
|---|---|---|---|
| FOUND-01 | Initialize Tauri + Svelte Project | DONE | 360×478 frameless window opens; cargo check passes |
| FOUND-02 | Configure Rust Dependencies | DONE | All deps in Cargo.toml; cargo check passes |
| FOUND-03 | Set Up CI/CD Pipeline | DONE | .github/workflows/build.yml; Linux x64; clippy + tests + artifact upload |
| FOUND-04 | Set Up Frontend Build Pipeline | DONE | SvelteKit + adapter-static + Vite configured in scaffold |
| FOUND-05 | Create Project Directory Structure | DONE | All Rust module stubs + Svelte component stubs; both checks pass |
| DATA-01 | SQLite Database Initialization | DONE | WAL mode, migrations, Arc<Mutex<Connection>> state |
| DATA-02 | Settings Read/Write | DONE | load/save/seed_defaults; 6 unit tests pass |
| DATA-03 | Session Recording | DONE | insert_session/complete_session/get_all_time_stats; 3 unit tests pass |
| DATA-04 | ~~Legacy Migration~~ | N/A | Eliminated — start fresh |
| TIMER-01 | Timer Engine Thread | DONE | Drift-correcting; Suspend/WakeResume; 6 unit tests |
| TIMER-02 | Pomodoro Sequence Logic | DONE | Full cycle logic; 6 unit tests (work_rounds 1/2/4/12) |
| TIMER-03 | Timer Controller | DONE | Bridges engine+sequence; emits Tauri events; get_snapshot |
| CMD-01 | Timer Commands (IPC) | DONE | timer_toggle/reset/skip/get_state registered |
| CMD-02 | Settings Commands | DONE | settings_get/set/reset_defaults; emits settings:changed |
| CMD-03 | Theme Commands | DONE | themes_list/theme_apply; 17 themes embedded at compile time |
| CMD-04 | Stats Commands | DONE | stats_get_all_time/stats_get_session |
| OS-01 | Window Management | DONE | window_set_visibility/always_on_top commands; break-always-on-top; close-to-tray |
| OS-02 | System Tray (tiny-skia) | DONE | 32×32 arc rendering; pause bars; round-change color; click toggle; Show/Exit menu |
| OS-03 | Global Shortcuts | DONE | Ctrl+F1/F2/F3 defaults; parse_shortcut; re-register on settings change |
| OS-04 | Desktop Notifications | DONE | tauri-plugin-notification; fires on round-change; respects notifications_enabled |
| OS-05 | WebSocket Server | DONE | tokio+axum; 127.0.0.1:{port}; getState handler; roundChange broadcast; error event |
| THEMES-01 | Theme Loading | DONE | Bundled (include_str!) + custom dir; list_all/find; 3 tests |
| THEMES-02 | Theme Hot-Reload (notify) | DONE | spawn_watcher; 500ms debounce; emits themes:changed |
| UI-TITLEBAR-01 | Titlebar Component | DONE | drag region; hamburger animation; minimize/close |
| UI-TIMER-01 | Timer Dial Component | DONE | SVG arc fills up; tweened animation; round label |
| UI-TIMER-02 | Timer Controls and Footer | DONE | play/pause; reset/skip; round counter |
| UI-TIMER-03 | Volume Control | DONE | speaker icon; vertical slider on hover |
| UI-TIMER-04 | Timer State Subscriptions | DONE | tick/paused/resumed/round-change/reset; hydrated on mount |
| UI-DRAWER-01 | Drawer Shell and Menu | DONE | Slides right; icon tabs with accent underline; DrawerMenu |
| UI-DRAWER-02 | Timer Configuration Panel | DONE | Colored sliders (mins); rounds; Reset Defaults |
| UI-DRAWER-03 | Settings Panel | DONE | Toggle rows; ShortcutInput; all bool settings |
| UI-DRAWER-04 | Themes Panel | DONE | Cards with own theme colors; checkmark; hot-reload |
| UI-DRAWER-05 | About Panel | DONE | Logo SVG; version; GitHub links via openUrl |
| AUDIO-01 | Rust Audio Engine (rodio) | DONE | Dedicated thread; include_bytes! assets; Tick+Alert cues |
| AUDIO-02 | Audio Volume Commands | DONE | apply_settings() called from settings_set via try_state |
| UI-NOTIFY-01 | Notification Manager (Rust-only) | DONE | Confirmed: all notifications in Rust; no frontend component needed |
| INT-01 | Full App Initialization | DONE | +page.svelte onMount loads settings/timer/themes; event listeners registered |
| INT-02 | Break Always On Top Logic | DONE | Handled in timer/mod.rs listen_events Complete handler |
| INT-03 | Session Recording Integration | DONE | insert/complete session on Tick(1)/Complete; skip_pending flag |
| INT-04 | CSS Transitions and Animations | DONE | App fade-in; play/pause icon fade; drawer panel fade; CSS timing vars |
| INT-05 | Port 17 Built-in Theme Files | DONE | All 17 JSON files already in static/themes/; bundled via include_str! |
| INT-06 | Bundle Audio Assets | DONE | 4 MP3 files in static/audio/; embedded via include_bytes! |
| TEST-01 | Timer Engine Unit Tests | DONE | 9 tests: ticks, drift, pause/resume, reset, skip, reconfigure, suspend/wake, monotonic, shutdown |
| TEST-02 | Sequence Logic Unit Tests | DONE | 7 tests: 1/2/4/12-round cycles, round-number increment, reset-after-long-break, reset |
| TEST-03 | Settings Round-Trip Tests | DONE | 8 tests: defaults, idempotent seed, bool/volume/time save, missing-keys fallback, reset, repeated-write |
| TEST-04 | Integration Tests | TODO | Manual checklist; automated subset via tauri::test is future work |
| TEST-05 | Platform-Specific Testing | TODO | Requires physical test on Windows, macOS, ARM |
| PKG-01 | Icons and Bundle Metadata | DONE | Icons regenerated from static/icon.png; tauri.conf.json: category, copyright, description, Linux/macOS/Windows bundle targets |
| PKG-02 | Code Signing | DEFERRED | Blocked on certificates; not a dev blocker |
| PKG-03 | Release Documentation | DONE | README: dev setup (Tauri/Rust/Svelte), custom theme format, WebSocket API; CHANGELOG: v1.0.0 entry |

---

## Phase 0: Decisions — Resolved

All open questions have been answered. Decisions are recorded in PROPOSAL.md.

| # | Item | Decision |
|---|---|---|
| OQ-1 | Timer behavior on OS sleep/hibernate | **Pause on sleep, resume on wake** (requires OS power event detection) |
| OQ-2 | Session data scope | **Schema supports history; no history UI in MVP** (all-time `totalWorkRounds` displayed; statistics view is future work) |
| OQ-3 | WebSocket server | **Opt-in, off by default, configurable port** (recommended: prevents unconditional port binding, best practice for desktop tools) |
| OQ-4 | Tray icon rendering | **Dynamic arc in Rust** using `tiny-skia` |
| OQ-5 | Frontend framework | **Svelte** (preferred; React as fallback if needed) |
| OQ-6 | Custom theme hot-reload | **`notify` crate directory watcher** (option a) |
| OQ-7 | Settings migration | **Start fresh** — no import of old `user-preferences.json` |
| OQ-8 | Audio engine | **Rust `rodio`** — guaranteed playback even when window is hidden |
| OQ-9 | Linux frameless window | **Default system behavior per platform**; custom treatments post-MVP |
| OQ-10 | Platform targets | **Linux x64 first**; add Windows, macOS, ARM in a later phase |

---

## Phase 1: Project Foundation

Set up the project structure, toolchain, and empty scaffolding.

### FOUND-01 — Initialize Tauri + Svelte Project
**Complexity: M** | `[BLOCKS: all Phase 2+]`

Steps:
1. Run `npm create tauri-app@latest` with Svelte + TypeScript template.
2. Configure `tauri.conf.json`: window size (360×478), frameless, identifier, product name.
3. Verify dev build runs: `npm run tauri dev`.
4. Commit initial scaffold.

Acceptance: `npm run tauri dev` opens a blank 360×478 frameless window.

---

### FOUND-02 — Configure Rust Workspace and Dependencies
**Complexity: S** | `[DEPENDS: FOUND-01]`

Steps:
1. Add all Rust dependencies to `Cargo.toml` (see DESIGN.md §7 for list).
2. Add Tauri plugins: `tauri-plugin-log`, `tauri-plugin-notification`, `tauri-plugin-global-shortcut`.
3. Run `cargo check` to verify all deps resolve.
4. Configure `tauri.conf.json` permissions for required plugins.

Acceptance: `cargo check` passes with no errors.

---

### FOUND-03 — Set Up CI/CD Pipeline
**Complexity: M** | `[DEPENDS: FOUND-01]`

Scope: **Linux x64 only** for this phase (per OQ-10). Additional platform jobs are added in a later phase.

Steps:
1. Create `.github/workflows/build.yml` with a single Linux x64 job (Ubuntu runner).
2. Install required system packages: `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`.
3. Add `cargo clippy -- -D warnings` step (fail on warnings).
4. Add `cargo test` step.
5. Configure artifact upload for the Linux build output.

Acceptance: A push triggers CI; Linux build, clippy, and test steps all pass.

---

### FOUND-04 — Set Up Frontend Build Pipeline
**Complexity: S** | `[DEPENDS: FOUND-01]`

Steps:
1. Configure `svelte.config.js` for static adapter (SPA mode, no SSR).
2. Configure `vite.config.ts`.
3. Set up `tsconfig.json` with strict mode.
4. Add path aliases (`$lib`, etc.).

Acceptance: `npm run build` produces a valid `build/` directory.

---

### FOUND-05 — Create Project Directory Structure
**Complexity: S** | `[DEPENDS: FOUND-01]`

Steps:
1. Create all Rust module files (`src-tauri/src/timer/mod.rs`, etc.) as empty stubs.
2. Create all Svelte component files as empty stubs.
3. Create `lib/stores/`, `lib/ipc/`, `lib/audio/` directories with placeholder files.

Acceptance: `cargo check` and `npm run check` both pass against stubs.

---

## Phase 2: Data Layer

### DATA-01 — Implement SQLite Database Initialization
**Complexity: M** | `[DEPENDS: FOUND-02]` | `[BLOCKS: DATA-02, DATA-03, SETTINGS-01]`

Steps:
1. Implement `db/mod.rs`: open/create `pomotroid.db` in the Tauri app data directory.
2. Implement `db/migrations.rs`: migration 1 SQL (see DESIGN.md §4).
3. On startup, read `schema_version`; apply any pending migrations.
4. Wrap connection in `Arc<Mutex<Connection>>` and register as Tauri state.

Acceptance: On first launch, `pomotroid.db` is created with the correct schema. On second launch, no migration is re-applied.

---

### DATA-02 — Implement Settings Read/Write
**Complexity: M** | `[DEPENDS: DATA-01]` | `[BLOCKS: SETTINGS-01, TIMER-01]`

Steps:
1. Implement `settings/mod.rs`: `Settings` struct with serde derive.
2. Implement `load_settings(db)` → reads all rows from `settings` table, deserializes values.
3. Implement `save_setting(db, key, value)` → upserts a single row.
4. Implement `default_settings()` → seeds the database with all defaults on first run.
5. Write unit tests for serialization/deserialization of each setting type.

Acceptance: Unit tests pass. Settings round-trip correctly through the DB.

---

### DATA-03 — Implement Session Recording
**Complexity: S** | `[DEPENDS: DATA-01]`

Steps:
1. Implement `db/queries.rs`: `insert_session`, `complete_session`, `get_session_stats`, `get_all_time_stats`.
2. Record a session row when a round starts (`started_at`, `round_type`, `duration_secs`).
3. Update `ended_at` and `completed` when the round ends (complete or skip).

Acceptance: After completing a work round, a row appears in the `sessions` table.

---

### DATA-04 — ~~Legacy Settings Migration~~ (REMOVED)

**Decision (OQ-7)**: The rewrite starts fresh — no import of `user-preferences.json`. This task is eliminated. The DB migration in DATA-01 seeds defaults directly. Users start with factory defaults.

---

## Phase 3: Core Timer (Rust)

The most critical phase. All timer logic lives here.

### TIMER-01 — Implement Timer Engine Thread
**Complexity: L** | `[DEPENDS: FOUND-02]` | `[BLOCKS: TIMER-02, TIMER-03]`

Steps:
1. Implement `timer/engine.rs` with the drift-correcting timer loop (see DESIGN.md §3).
2. Define `TimerCommand` and `TimerEvent` enums. Add `Suspend` and `WakeResume` to `TimerCommand`.
3. Implement thread spawn: takes `duration_secs`, `mpsc::Sender<TimerEvent>`, and `mpsc::Receiver<TimerCommand>`.
4. Handle all commands: Start, Pause, Resume, Reset, Skip, Reconfigure, **Suspend, WakeResume**.
5. Implement drift correction: each sleep targets the next scheduled tick instant, not a fixed 1000ms.
6. **Sleep/wake behavior (OQ-1)**: Add OS power event detection in `main.rs` using Tauri's system events or the `system-events` crate. On sleep signal: send `Suspend` command → engine saves current `elapsed_secs` and stops the loop. On wake signal: send `WakeResume` → engine restarts from saved position (does NOT advance or complete — timer resumes exactly where it left off).
7. Write unit tests:
   - Timer fires exactly N `Tick` events for N-second duration.
   - Timer fires `Complete` after `duration_secs` ticks.
   - Pause stops ticks; Resume continues from correct position.
   - Reset returns to 0.
   - Suspend + WakeResume: elapsed position is preserved, no extra ticks fire during gap.

Acceptance: Unit tests pass. A 10-second timer fires 10 ticks and completes within ±100ms of 10 seconds. A suspended+resumed timer continues from the correct position.

---

### TIMER-02 — Implement Pomodoro Sequence Logic
**Complexity: M** | `[DEPENDS: TIMER-01]` | `[BLOCKS: TIMER-03]`

Steps:
1. Implement `timer/sequence.rs`: `SequenceState`, `advance_round()`, `next_round_type()`.
2. Logic: work → short-break → work ... → (round == work_rounds) → long-break → work (reset).
3. Track `round_index`, `total_work_rounds`.
4. Write unit tests covering full cycle with various `work_rounds` values (1, 2, 4, 12).

Acceptance: Unit tests for full Pomodoro cycles pass.

---

### TIMER-03 — Implement Timer Controller (Engine + Sequence Bridge)
**Complexity: M** | `[DEPENDS: TIMER-01, TIMER-02, DATA-02]` | `[BLOCKS: CMD-01]`

Steps:
1. Implement `timer/mod.rs`: `TimerController` that owns the engine channel and sequence state.
2. On `TimerEvent::Complete`: call `sequence.advance_round()`, get next duration, reconfigure engine.
3. Translate engine events into Tauri `app.emit()` calls (see DESIGN.md §3 event table).
4. Implement `get_snapshot()` → `TimerSnapshot` for the frontend to query initial state.
5. Register `TimerController` as Tauri managed state.

Acceptance: Starting and completing a timer emits the correct Tauri events.

---

## Phase 4: Tauri Commands (IPC Layer)

### CMD-01 — Implement Timer Commands
**Complexity: M** | `[DEPENDS: TIMER-03]` | `[BLOCKS: UI-TIMER-01]`

Steps:
1. Implement in `commands.rs`:
   - `timer_start`, `timer_pause`, `timer_resume`, `timer_reset`, `timer_skip`
   - `timer_get_state` → `TimerSnapshot`
2. Register all commands in `lib.rs` `generate_handler!()`.
3. Test each command via `tauri::test` or manual invocation.

Acceptance: Each command produces the expected state change and Tauri event.

---

### CMD-02 — Implement Settings Commands
**Complexity: S** | `[DEPENDS: DATA-02]` | `[BLOCKS: UI-SETTINGS-01]`

Steps:
1. Implement `settings_get`, `settings_set`, `settings_reset_defaults` in `commands.rs`.
2. After settings change, emit `settings:changed` event to frontend.

Acceptance: Invoking `settings_set` updates the DB and emits the event.

---

### CMD-03 — Implement Theme Commands
**Complexity: S** | `[DEPENDS: THEMES-01]` | `[BLOCKS: UI-THEME-01]`

Steps:
1. Implement `themes_list`, `theme_apply` in `commands.rs`.
2. `theme_apply` persists the selection via `settings_set` and returns the full `Theme` object.

Acceptance: Frontend can list all themes and apply one.

---

### CMD-04 — Implement Stats Commands
**Complexity: S** | `[DEPENDS: DATA-03]` | `[BLOCKS: UI-TIMER-01]`

Steps:
1. Implement `stats_get_session`, `stats_get_all_time` in `commands.rs`.
2. `SessionStats`: `{ session_work_rounds: u32, session_started: Option<i64> }`
3. `AllTimeStats`: `{ total_work_rounds: u32, total_work_minutes: u32 }`

Acceptance: Stats return correct values from the sessions table.

---

## Phase 5: OS Integrations (Rust)

### OS-01 — Implement Window Management
**Complexity: M** | `[DEPENDS: FOUND-01]` | `[BLOCKS: UI-TITLEBAR-01]`

Notes: Linux x64 only for MVP. macOS tray positioning deferred to a future phase (per OQ-10).

Steps:
1. Implement `window_set_visibility`, `window_set_always_on_top` commands.
2. Implement break-always-on-top logic: subscribe to `timer:round-change` in main.rs, apply always-on-top based on round type.
3. Implement minimize-to-tray: `window.hide()` vs `window.minimize()` based on setting.
4. Use default system window decorations behavior per OQ-9 — no platform-specific drag region customization in MVP.

Acceptance: Always-on-top, hide-to-tray, and close behaviors work correctly on Linux.

---

### OS-02 — Implement System Tray
**Complexity: L** | `[DEPENDS: OS-01]`

**Decision (OQ-4)**: Dynamic arc rendering in Rust using `tiny-skia`.

Steps:
1. Add `tiny-skia = "0.11"` to `Cargo.toml`.
2. Implement `tray/mod.rs`:
   - `create_tray()`: initialize tray icon with initial idle image.
   - `destroy_tray()`: drop the tray handle.
   - Both called from the `min_to_tray` setting change handler.
3. Build context menu: "Show" (toggle window visibility), "Exit" (`app.exit(0)`).
4. Handle tray click: toggle window visibility.
5. Implement `render_tray_icon(round_type, paused, progress) -> Vec<u8>`:
   - Use `tiny-skia` to draw: solid background circle + arc from top sweeping clockwise per `progress` (0.0–1.0).
   - Colors come from the active theme's color values (store as `Arc<Mutex<ThemeColors>>` updated when theme changes).
   - Paused state: draw two vertical bars instead of the arc.
   - Output: raw RGBA bytes → `TrayIcon::set_icon(Icon::from_rgba(...))`.
   - Icon size: 32×32 px (standard cross-platform tray size for Linux MVP).
6. Subscribe to `timer:tick`: call `render_tray_icon` if `paused=false` and visual delta > 1% of total (throttle redraws).
7. Subscribe to `timer:paused`, `timer:resumed`, `timer:round-change`: always update icon.

Acceptance: Tray appears when setting is enabled; click toggles window; arc depletes as timer runs; paused icon shows bars; round-change updates color.

---

### OS-03 — Implement Global Shortcuts
**Complexity: M** | `[DEPENDS: FOUND-02, CMD-01]`

Steps:
1. Implement `shortcuts/mod.rs`: register/unregister shortcuts using `tauri-plugin-global-shortcut`.
2. On shortcut fire, invoke the appropriate timer command directly (no IPC round-trip needed).
3. Implement `shortcuts_set` command: unregister all, re-register from new config.
4. Test that Super/Cmd modifier key works correctly (fix bug from existing app).
5. Test that shortcuts work when window is hidden.

Acceptance: Default shortcuts (Ctrl+F1/F2/F3) work system-wide. Configuring a new shortcut takes effect immediately.

---

### OS-04 — Implement Desktop Notifications
**Complexity: S** | `[DEPENDS: FOUND-02]`

Steps:
1. Implement `notifications/mod.rs` using `tauri-plugin-notification`.
2. Subscribe to `timer:round-change` in main.rs; call appropriate notification based on new round type.
3. Respect the `notifications` setting from `Settings`.
4. Include round-specific notification body text and icon.

Acceptance: Notification appears on round transition on all three platforms. Disabling the setting prevents notifications.

---

### OS-05 — Implement WebSocket Server
**Complexity: M** | `[DEPENDS: FOUND-02]`

**Decision (OQ-3)**: Opt-in, off by default, port configurable. Best practice — unconditional port binding causes silent failures, unexpected firewall prompts, and surprises users who don't need external integration. Toggle and port are exposed in the Settings panel (UI-DRAWER-03).

Steps:
1. Implement `websocket/mod.rs` using `tokio` + `axum` WebSocket upgrade.
2. Bind to `127.0.0.1:{port}` only (localhost, not all interfaces — security requirement).
3. Handle `getState` message from clients; respond with current `TimerSnapshot`.
4. Subscribe to timer controller events via a `tokio::sync::broadcast` channel; broadcast `roundChange` to all connected clients on round change.
5. Expose `start_server(port)` and `stop_server()` — called from the settings change handler.
6. On startup, start the server only if `websocket_enabled = true`.
7. Graceful shutdown on app exit (abort the `tokio` task).
8. If port binding fails: emit a `websocket:error` Tauri event to the frontend with the error message; do not crash silently.

Acceptance: Server starts only when enabled. An external client at `ws://127.0.0.1:{port}` receives `roundChange` events. Port conflict produces a visible error event rather than a silent failure.

---

## Phase 6: Themes

### THEMES-01 — Implement Theme Loading (Built-in + Custom)
**Complexity: M** | `[DEPENDS: FOUND-01]` | `[BLOCKS: CMD-03, UI-THEME-01]`

Steps:
1. Implement `themes/mod.rs`: `ThemeManager` struct.
2. Load built-in themes from the bundled `static/themes/` directory (17 JSON files).
3. Load custom themes from `{app_data_dir}/themes/` (create dir if absent).
4. Parse and validate each theme JSON (log errors for invalid files, skip them).
5. Return merged `Vec<Theme>` sorted alphabetically (built-in first, then custom).
6. Register `ThemeManager` as Tauri state.

Acceptance: `themes_list` command returns all 17 built-in themes plus any custom themes.

---

### THEMES-02 — Implement Theme Hot-Reload
**Complexity: M** | `[DEPENDS: THEMES-01]`

**Decision (OQ-6)**: Required — use `notify` crate directory watcher (option a).

Steps:
1. Add `notify = "6"` to `Cargo.toml` (already included in DESIGN.md deps).
2. In `themes/watcher.rs`, spawn a `notify::RecommendedWatcher` on `{app_data_dir}/themes/`.
3. Debounce events by 500ms to avoid multiple reloads on rapid file saves.
4. On debounced event: re-scan directory, rebuild `Vec<Theme>`, update `ThemeManager` state, emit `themes:changed` Tauri event with new list.
5. Log errors from invalid JSON files encountered during hot-reload without panicking.

Acceptance: Placing or modifying a JSON file in `{app_data_dir}/themes/` causes the theme to appear in the UI within 1 second, without restarting the app.

---

## Phase 7: Frontend — Core Timer UI

### UI-TITLEBAR-01 — Implement Titlebar Component
**Complexity: S** | `[DEPENDS: FOUND-04]` | `[BLOCKS: UI-TIMER-01]`

Steps:
1. Create `Titlebar.svelte` with `data-tauri-drag-region` on the nav.
2. Hamburger/menu button: toggles drawer open/closed state.
3. Minimize button: invokes `window_minimize` (hide or minimize based on `minToTray` setting).
4. Close button: invokes `window_close` (close or hide based on `minToTrayOnClose` setting).
5. Style matches existing design (centered title, buttons at right).
6. Test drag region on all platforms.

Acceptance: Window is draggable by the titlebar. Buttons work correctly per settings.

---

### UI-TIMER-01 — Implement Timer Dial Component
**Complexity: M** | `[DEPENDS: FOUND-04, CMD-01]`

Steps:
1. Create `TimerDial.svelte` with SVG circle path.
2. Accept props: `progress` (0.0–1.0), `roundType`, `isActive`.
3. Use Svelte `tweened` store for smooth `stroke-dashoffset` animation.
4. Apply color class based on `roundType`.
5. Show round type label inside the dial.
6. Create `TimerDisplay.svelte`: shows `MM:00` (pre-start) or `MM:SS` (running).
7. Time calculation: `total - elapsed` in pure arithmetic; no JS Date.

Acceptance: Dial animates smoothly. Colors match round type. Time display is correct.

---

### UI-TIMER-02 — Implement Timer Controls and Footer
**Complexity: M** | `[DEPENDS: UI-TIMER-01]`

Steps:
1. Create `TimerControls.svelte`: start/pause/resume button with SVG icons.
   - Shows play icon when idle or paused.
   - Shows pause icon when running.
   - Transitions via Svelte fade.
2. Create `TimerFooter.svelte`:
   - Round counter: `{roundIndex}/{workRounds}` + `(totalWorkRounds)` in dimmer style.
   - Reset button (text).
   - Skip button (icon).
3. Connect all buttons to Tauri commands.
4. Space bar listener: toggle timer.

Acceptance: All controls invoke the correct Tauri commands. Space bar works.

---

### UI-TIMER-03 — Implement Volume Control
**Complexity: M** | `[DEPENDS: UI-TIMER-02]`

Steps:
1. Create `VolumeControl.svelte` within `TimerFooter`.
2. Mute icon: shows speaker/muted based on volume > 0.
3. Hover on mute icon: reveals vertical range slider using Svelte `on:mouseenter`/`on:mouseleave` (no pixel coordinates).
4. Slider value bound to `settings.volume`.
5. Changes invoke `settings_set({ volume })`.
6. Mute toggle: sets volume to 0 (saves previous non-zero value) or restores it.

Acceptance: Volume slider appears on hover. Mute/unmute cycles correctly. Setting persists.

---

### UI-TIMER-04 — Implement Timer State Subscriptions
**Complexity: M** | `[DEPENDS: UI-TIMER-01, CMD-01]`

Steps:
1. In `Timer.svelte` (or `+page.svelte`), subscribe to all timer Tauri events on mount.
2. On `timer:tick` → update `$timerStore.elapsedSecs`.
3. On `timer:round-change` → update round type, index, total; reset elapsed.
4. On `timer:paused`, `timer:resumed`, `timer:reset` → update `engineState`.
5. On mount, call `timer_get_state` to initialize state.
6. Auto-start: on `timer:round-change` with `auto_start: true`, wait 1.5s then call `timer_start`.

Acceptance: UI reflects exact Rust timer state at all times. Auto-start works correctly.

---

## Phase 8: Frontend — Settings Drawer

### UI-DRAWER-01 — Implement Drawer Shell and Menu
**Complexity: S** | `[DEPENDS: UI-TITLEBAR-01]`

Steps:
1. Create `Drawer.svelte`: slide-in from left, full height minus titlebar.
2. Create `DrawerMenu.svelte`: 4 icon tabs (Timer, Settings, Themes, About).
3. Active tab highlighted with bottom border animation.
4. Tab switching updates `currentTab` local state.

Acceptance: Drawer slides in/out. Tab switching works. Correct panel shows.

---

### UI-DRAWER-02 — Implement Timer Configuration Panel
**Complexity: M** | `[DEPENDS: UI-DRAWER-01, CMD-02]`

Steps:
1. Create `DrawerTimer.svelte` with sliders for work, short break, long break durations and rounds count.
2. Sliders bound to local state; on `change` event, invoke `settings_set`.
3. If the timer's current round matches the changed duration, emit `timer_reset`.
4. "Reset Defaults" button: invokes `settings_reset_defaults`, re-initializes sliders, resets timer.
5. Display current value next to each slider in monospace font.

Acceptance: Changing a slider updates the setting. Changing the current round's duration resets the timer.

---

### UI-DRAWER-03 — Implement Settings Panel
**Complexity: M** | `[DEPENDS: UI-DRAWER-01, CMD-02, OS-01, OS-02, OS-03, OS-05]`

Steps:
1. Create `DrawerSettings.svelte` with all boolean toggles (see REQUIREMENTS.md FR-SET-01 through FR-SET-05).
2. "Break Always On Top" toggle: only visible when "Always On Top" is active (conditional reactive binding).
3. **WebSocket section**: Toggle enable/disable + port number input field (numeric, 1024–65535). Only shown as an advanced setting (can be grouped under a collapsible "Advanced" section or placed at the bottom of the panel).
4. Shortcut inputs via `ShortcutInput.svelte`.
5. Create `ShortcutInput.svelte`:
   - Input captures keyup; builds Tauri accelerator string with correct modifier detection.
   - Fix the existing metaKey bug: test that `event.metaKey` → `Super` registers correctly. On Linux this key may not be capturable at the browser level; document this limitation.
   - Emits formatted string to parent on valid capture.
6. Each toggle change invokes `settings_set` and, where needed, a window/tray/shortcut/websocket command.

Acceptance: All toggles persist. "Break Always On Top" visibility is conditional. Shortcut input captures all modifiers.

---

### UI-DRAWER-04 — Implement Themes Panel
**Complexity: M** | `[DEPENDS: UI-DRAWER-01, CMD-03, THEMES-01]`

Steps:
1. Create `DrawerTheme.svelte`.
2. On mount, invoke `themes_list` to populate theme list.
3. Each theme rendered with its own background and accent colors.
4. Active theme shows checkmark.
5. Clicking a theme invokes `theme_apply`; `activeTheme` store is updated; CSS variables applied immediately.
6. Listen for `themes:changed` event to refresh list (if THEMES-02 is implemented).

Acceptance: All 17 themes are listed. Clicking one changes the app appearance immediately.

---

### UI-DRAWER-05 — Implement About Panel
**Complexity: S** | `[DEPENDS: UI-DRAWER-01]`

Steps:
1. Create `DrawerAbout.svelte`.
2. Display app version (read from Tauri `app.getVersion()`).
3. "Release Notes" link: `tauri.open()` to GitHub releases URL.
4. "License and Documentation" link: `tauri.open()` to GitHub repo URL.

Acceptance: Version is correct. Links open in the default browser.

---

## Phase 9: Rust Audio + Notifications

### AUDIO-01 — Implement Rust Audio Engine
**Complexity: M** | `[DEPENDS: FOUND-02]` | `[BLOCKS: AUDIO-02]`

**Decision (OQ-8)**: Audio playback in Rust via `rodio`. This guarantees sounds play even when the window is hidden to the tray — critical for a Pomodoro timer used in background mode.

Steps:
1. Add `rodio = "0.17"` to `Cargo.toml`.
2. Create `audio/mod.rs`: `AudioManager` struct holding the `rodio::OutputStream` and `OutputStreamHandle`.
3. Embed the 4 audio files at compile time via `include_bytes!` (avoids runtime path resolution issues).
4. Implement `play_sound(sound: AudioCue)` where `AudioCue` is an enum: `WorkAlert`, `ShortBreakAlert`, `LongBreakAlert`, `Tick`.
5. Apply volume: `rodio::Sink::set_volume(volume as f32 / 100.0)`.
6. Subscribe to `timer:round-change` in `TimerController`: play the appropriate alert sound if `notifications` (audio) is enabled.
7. Subscribe to `timer:tick`: play `Tick` sound if the per-phase tick setting is enabled.
8. Store current volume and tick settings as `Arc<Mutex<AudioSettings>>` updated by the settings change handler.
9. Register `AudioManager` as Tauri state.

Note: `rodio` OutputStream must remain alive for the duration of the app. Do not drop it.

Acceptance: Alert sounds play on round transitions. Tick sounds play per-second when enabled. All sounds play with the window hidden.

---

### AUDIO-02 — Expose Audio Volume Commands to Frontend
**Complexity: S** | `[DEPENDS: AUDIO-01, CMD-02]`

The frontend still controls volume (via the volume slider) and mute. These are settings, so `settings_set({ volume })` is the mechanism. `AudioManager` watches the settings store for volume changes.

Steps:
1. In the settings change handler, propagate `volume` changes to `AudioManager::set_volume()`.
2. Volume = 0 effectively mutes all audio (no separate mute state needed in Rust).

Acceptance: Adjusting the frontend volume slider immediately affects audio output level.

---

### UI-NOTIFY-01 — Notification Manager (Rust-only)
**Complexity: S** | `[DEPENDS: OS-04]`

**Decision**: All notifications are handled in Rust via `tauri-plugin-notification` (OS-04). No frontend notification component is needed. This task is a placeholder confirmation: no `NotificationManager.svelte` component should be created. The `AudioManager.svelte` component in the frontend is also eliminated (audio is in Rust via AUDIO-01).

Acceptance: Notifications fire from Rust on round transitions. No audio or notification code lives in the Svelte frontend.

---

## Phase 10: Integration and Polish

### INT-01 — Wire Up Full App Initialization
**Complexity: M** | `[DEPENDS: all Phase 7–9]`

Steps:
1. In `+page.svelte` `onMount`:
   - Call `timer_get_state` → populate `$timerStore`.
   - Call `settings_get` → populate `$settingsStore`.
   - Call `themes_list` → populate `$themes`.
   - Apply active theme from settings.
   - Register all event listeners (timer events, settings:changed, themes:changed, shortcut:fired).
2. Handle Space key listener.
3. Handle app lifecycle (cleanup listeners on destroy).

Acceptance: App launches in the correct state (timer idle at correct duration, theme applied, settings loaded).

---

### INT-02 — Implement `break_always_on_top` Logic
**Complexity: S** | `[DEPENDS: OS-01, TIMER-03]`

Steps:
1. In Rust `main.rs`, listen to `timer:round-change` event.
2. If `break_always_on_top` is enabled: set window not-always-on-top when entering a break; restore when entering work.
3. If `always_on_top` is disabled, `break_always_on_top` has no effect.

Acceptance: Window drops below other windows during breaks when both settings are enabled.

---

### INT-03 — Implement Session Recording Integration
**Complexity: S** | `[DEPENDS: DATA-03, TIMER-03]`

Steps:
1. In `TimerController`, when a round starts, insert a `sessions` row.
2. When a round completes naturally, update `ended_at` and `completed=1`.
3. When skipped, update `ended_at` and `completed=0`.
4. When reset, leave the row as-is (session was abandoned mid-round).

Acceptance: `sessions` table accurately reflects timer activity.

---

### INT-04 — Apply CSS Transitions and Animations
**Complexity: M** | `[DEPENDS: all Phase 7]`

Steps:
1. Add fade-in animation on app mount.
2. Add slide-left transition on drawer open/close.
3. Add fade transition on play/pause button swap.
4. Ensure transitions use CSS variables for consistent timing.
5. Verify animations don't interfere with timer state.

Acceptance: App matches the visual quality of the original (smooth, no janky transitions).

---

### INT-05 — Port All 17 Built-in Theme Files
**Complexity: S** | `[DEPENDS: THEMES-01]`

Steps:
1. Copy existing theme JSON files from `static/themes/` to the new project.
2. Verify all 17 themes load correctly in `DrawerTheme`.
3. Spot-check 3–4 themes visually for correctness.

Acceptance: All 17 themes appear in the list and apply correctly.

---

### INT-06 — Bundle Audio Assets
**Complexity: S** | `[DEPENDS: UI-AUDIO-01]`

Steps:
1. Copy 4 audio files from `static/audio/` to the new project's `static/audio/`.
2. Verify audio plays at correct times in the app.

Acceptance: All 4 sounds play correctly.

---

## Phase 11: Testing

### TEST-01 — Timer Engine Unit Tests
**Complexity: M** | `[DEPENDS: TIMER-01]`

Test cases:
- [ ] N-second timer emits exactly N ticks then Complete
- [ ] Timer fires Complete within ±100ms of the nominal duration (real-time test)
- [ ] Pause stops ticks for the pause duration (no tick events during pause)
- [ ] Resume resumes from correct elapsed position
- [ ] Reset fires Reset event and elapsed returns to 0
- [ ] Skip fires Complete immediately
- [ ] Reconfigure changes the duration (used when settings change mid-round)
- [ ] Drift: 60-second timer completes within ±200ms of 60 real seconds

---

### TEST-02 — Sequence Logic Unit Tests
**Complexity: S** | `[DEPENDS: TIMER-02]`

Test cases:
- [ ] 4-round cycle: W→SB→W→SB→W→SB→W→LB→W (reset round to 1)
- [ ] 1-round cycle: W→LB→W→LB...
- [ ] 12-round cycle: correct long-break trigger
- [ ] totalWorkRounds increments on each completed work round
- [ ] Long break resets round_index to 1

---

### TEST-03 — Settings Round-Trip Tests
**Complexity: S** | `[DEPENDS: DATA-02]`

Test cases:
- [ ] All settings survive write + read
- [ ] Default settings applied on first launch (empty DB)
- [ ] `reset_defaults` restores only timer-related settings
- [ ] Boolean settings don't corrupt on multiple writes

---

### TEST-04 — Integration Tests (End-to-End)
**Complexity: L** | `[DEPENDS: INT-01]`

Manual test checklist (automated via `tauri::test` where possible):
- [ ] Full 25-min timer completes and advances to short break
- [ ] Auto-start fires after 1.5 seconds
- [ ] Timer continues when window is hidden (tray)
- [ ] Skip advances the round correctly
- [ ] Reset returns to correct duration
- [ ] Theme change persists across restart
- [ ] Global shortcuts work with window hidden
- [ ] Notification fires on round transition on all 3 platforms
- [ ] Volume control works and persists

---

### TEST-05 — Platform-Specific Testing
**Complexity: M** | `[DEPENDS: TEST-04]`

- [ ] Windows: Notifications, tray, shortcuts, drag region
- [ ] macOS: Tray positioning, dock icon, shortcuts (Cmd key)
- [ ] Linux (X11): All features
- [ ] Linux (Wayland): Drag region, tray, shortcuts
- [ ] Linux ARM64: Build and basic functionality

---

## Phase 12: Packaging and Release

### PKG-01 — Configure Icons and Bundle Metadata
**Complexity: S** | `[DEPENDS: FOUND-03]`

Steps:
1. Prepare all required icon sizes (Tauri needs multiple formats and sizes).
2. Configure `tauri.conf.json` bundle metadata (category, description, authors).
3. Configure Windows NSIS installer options.
4. Configure macOS DMG layout.
5. Configure Linux `.deb`, `.AppImage`, `.rpm` targets.

Acceptance: `npm run tauri build` produces a complete, installable package on each platform.

---

### PKG-02 — Configure Code Signing (macOS + Windows)
**Complexity: L** | `[HUMAN]` — Requires certificates and accounts.

Steps:
1. macOS: Configure Apple Developer certificate in CI secrets.
2. macOS: Configure notarization credentials.
3. Windows: Configure code signing certificate (or document the "unsigned" situation).

This task requires human action to obtain certificates. **Mark as blocked until certs are available.**

---

### PKG-03 — Write Release Documentation
**Complexity: S** | `[DEPENDS: PKG-01]`

Steps:
1. Update `README.md` with new installation instructions for all platforms.
2. Document the custom theme format (unchanged from existing).
3. Document the WebSocket API (unchanged protocol, new default is off).
4. Update `CHANGELOG.md`.

---

## Task Summary

| Phase | Tasks | Critical Path |
|---|---|---|
| 0: Decisions | P0-01 through P0-08 | Must be resolved first |
| 1: Foundation | FOUND-01–05 | FOUND-01 blocks everything |
| 2: Data Layer | DATA-01–04 | DATA-01 → DATA-02 → SETTINGS commands |
| 3: Core Timer | TIMER-01–03 | **Highest risk; longest single task** |
| 4: Commands | CMD-01–04 | Parallel after TIMER-03 |
| 5: OS Integrations | OS-01–05 | OS-01 and OS-03 are independent |
| 6: Themes | THEMES-01–02 | Simple, no blocking dependencies |
| 7: Frontend Timer | UI-TIMER-01–04 | Sequential |
| 8: Frontend Drawer | UI-DRAWER-01–05 | UI-DRAWER-01 first, then parallel |
| 9: Audio/Notify | UI-AUDIO-01, UI-NOTIFY-01 | Independent |
| 10: Integration | INT-01–06 | INT-01 last (wires everything) |
| 11: Testing | TEST-01–05 | Can begin after each phase |
| 12: Packaging | PKG-01–03 | Last; PKG-02 needs human action |

**Recommended order for first working build (MVP milestones)**:
1. FOUND-01 → FOUND-02 → DATA-01 → DATA-02 → TIMER-01 → TIMER-02 → TIMER-03 → CMD-01
2. At this point: Rust timer with correct sequencing is callable from frontend.
3. UI-TIMER-01 → UI-TIMER-02 → UI-TIMER-04 → INT-01
4. At this point: Functional timer UI with correct state.
5. All remaining tasks in parallel sprints.
