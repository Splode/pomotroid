# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Pomotroid is a Pomodoro timer desktop app built with **Tauri 2 + Rust** (backend) and **SvelteKit + Svelte 5** (frontend). The app has been fully rewritten from Electron+Vue to this stack.

## Commands

```bash
# Development (runs Vite dev server + Tauri)
npm run tauri dev

# Production build
npm run tauri build

# Frontend-only Vite dev (no Tauri IPC available)
npm run dev

# Format all frontend source files
npm run format

# Type-check Svelte + TypeScript
npm run check

# Regenerate i18n types after editing inlang message files
npm run paraglide:compile
```

There are no automated tests in this codebase.

## Architecture

### Two windows

- **`main`** (`src/routes/+page.svelte`) — the timer window
- **`settings`** (`src/routes/settings/+page.svelte`) — opened via `new WebviewWindow('settings', ...)` from `Titlebar.svelte`
- **`stats`** (`src/routes/stats/+page.svelte`) — statistics window

### IPC layer

All frontend↔backend communication goes through `src/lib/ipc/index.ts`. This module exports typed wrappers around `invoke()` and `listen()`. Never call `invoke()` directly in components — always go through this module.

Rust commands live in `src-tauri/src/commands.rs`. All commands return `Result<T, String>`.

**Tauri events emitted by Rust:**

- `timer:tick` — `{ elapsed_secs, total_secs }` — fires every second while running
- `timer:paused` — `{ elapsed_secs }`
- `timer:resumed` — `{ elapsed_secs }`
- `timer:round-change` — full `TimerSnapshot`
- `timer:reset` — full `TimerSnapshot` (used to sync frontend after settings changes)
- `settings:changed` — full `Settings` object
- `themes:changed` — `Theme[]`
- `sessions:cleared`

### TypeScript ↔ Rust type contract

`src/lib/types.ts` mirrors Rust structs (snake_case field names). When modifying `Settings` in `src-tauri/src/settings/mod.rs` or `TimerSnapshot` in `src-tauri/src/timer/mod.rs`, update `types.ts` to match.

**Important conversions** — Rust converts on load/save, frontend always sees the converted form:

- Time: stored in DB as **minutes**, `Settings` struct holds **seconds**
- Volume: stored in DB as **0–100**, `Settings` struct holds **0.0–1.0**

### Settings storage

SQLite key/value table. DB column names differ from Rust struct field names (e.g., DB key `work_rounds` → struct field `long_break_interval`). Mappings are in `src-tauri/src/settings/mod.rs`. New settings need a DB key mapping, a default value in `settings/defaults.rs`, and a corresponding `settings_set` key handler.

Schema migrations are in `src-tauri/src/db/migrations.rs`. Add a new `MIGRATION_N` constant and increment the schema version check in `run()`.

### Timer engine

The timer runs in a background thread. `TimerController` (`src-tauri/src/timer/mod.rs`) is the public API registered as Tauri state. `SequenceState` (`src-tauri/src/timer/sequence.rs`) tracks round type and count — it has its own `work_rounds_total` field that must be kept in sync via `apply_settings()` when settings change.

### Theme system

Themes are JSON files in `static/themes/` (built-in) and `{app_data_dir}/themes/` (custom, user-created). Each theme is a JSON object with `name` and `colors` (CSS custom property map with `--` prefix keys). The Rust `themes` module watches the custom directory for changes and emits `themes:changed`. `applyTheme()` in `src/lib/stores/theme.ts` sets CSS custom properties on `:root`.

### Capabilities / permissions

`src-tauri/capabilities/default.json` — all allowed Tauri APIs for both windows. Any new IPC command or API plugin used from the frontend must be allowlisted here.

### Localization

Uses `@inlang/paraglide-js`. Message files are in `project.inlang/`. After editing messages, run `npm run paraglide:compile` to regenerate `src/paraglide/`. Import message functions from `$lib/locale.svelte.ts`.

### UI conventions

- Window is decoration-free; custom titlebar in `Titlebar.svelte`
- Compact mode: `isCompact = w < 300 || h < 300` — hides footer/label/play-pause, shows only dial
- `uiScale` applied via CSS `zoom` on `.timer` div (not `transform: scale`)
- Slider track alignment: use `calc(frac * (100% - 14px) + 7px)` to match native thumb position

### Rust modules

| Module          | Purpose                                                                              |
| --------------- | ------------------------------------------------------------------------------------ |
| `audio`         | rodio-based audio playback, custom sound file management                             |
| `db`            | SQLite connection, migrations, queries                                               |
| `notifications` | OS notification wrapper                                                              |
| `settings`      | Settings struct, DB load/save, defaults                                              |
| `shortcuts`     | Global keyboard shortcut registration via tauri-plugin-global-shortcut               |
| `themes`        | Theme loading, custom theme watching (notify crate), tray icon rendering (tiny-skia) |
| `timer`         | Timer engine (background thread), sequence state, controller                         |
| `tray`          | System tray icon, tray menu, timer display in tray                                   |
| `websocket`     | Optional WebSocket server (axum) for external timer control                          |
