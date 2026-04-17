## Context

The sessions table (schema v1) already records every round with `started_at`, `ended_at`, `round_type`, `duration_secs`, and `completed`. Two commands (`stats_get_all_time`, `stats_get_session`) exist but are minimal — they power no UI today. The settings window pattern (a `WebviewWindow` opened from `Titlebar.svelte`, sharing the capabilities file and CSS theme) is the model for the statistics window.

## Goals / Non-Goals

**Goals:**

- Dedicated statistics window with three tabs: Today, This Week, All Time
- Today tab: stat cards (rounds, focus time, completion rate) + hourly session timeline
- This Week tab: per-day bar chart (7 days) + current streak counter
- All Time tab: 52-week heatmap grid + lifetime totals + longest streak
- Stats button in the main titlebar (left side, alongside settings icon on Linux/Windows; right side on macOS)
- All charts rendered as pure SVG — no external library dependencies
- Streak active until midnight: yesterday's streak is preserved until end of the current day
- Heatmap uses a fixed 4-level intensity scale (0 / 1–3 / 4–7 / 8+) mapped to theme colors

**Non-Goals:**

- Editing or deleting session history
- Export to CSV/JSON
- Per-project or tag-based tracking
- Push notifications or reminders based on stats

## Decisions

### 1. Window architecture: dedicated `WebviewWindow` at `/stats`

**Decision**: Third Tauri `WebviewWindow` (label `"stats"`, url `"/stats"`), following the same pattern as the settings window.

**Rationale**: Stats is a distinct view with its own lifecycle — it can be open while the timer is running without interfering. Re-using the settings window (e.g., adding a Stats section) would embed a wide heatmap inside a fixed 720px sidebar layout and mix concerns.

**Alternative considered**: Settings section — rejected because the heatmap needs ~800px width and stats is not a setting.

### 2. No external charting library

**Decision**: All charts (bar chart, heatmap) are rendered with pure inline SVG computed from data in Svelte components.

**Rationale**: The app has no existing chart dependency. The two chart types needed (bar chart, calendar grid) are geometrically trivial — a bar chart is rectangles, a heatmap is a grid of `<rect>` elements. Adding a library (Chart.js, D3, etc.) would add ~200–500 KB and complicate theming. Pure SVG reads from CSS custom properties naturally.

**Alternative considered**: D3.js — rejected (overkill, bundle size, theming complexity).

### 3. Timezone: SQLite `'localtime'` modifier

**Decision**: All per-day grouping uses `date(started_at, 'unixepoch', 'localtime')` rather than UTC.

**Rationale**: A session at 11 PM local time in Tokyo is `started_at` in UTC the following day. Grouping by UTC date produces wrong daily totals for users outside UTC. SQLite's built-in `'localtime'` modifier uses the OS timezone, matching what the user sees on their clock.

**Alternative considered**: Storing local date alongside unix timestamp — rejected (redundant, requires migration, locale changes would corrupt historical data).

### 4. Streak computation: Rust, not SQL

**Decision**: Fetch per-day counts from SQLite (`get_heatmap_data` returns all days with at least 1 completed work session), then compute current and longest streaks in Rust by walking the sorted date list.

**Rationale**: A recursive SQL CTE for streaks is possible but fragile and hard to test. Walking a `Vec<NaiveDate>` in Rust is straightforward, unit-testable, and handles the "until midnight" grace period cleanly (if today has no sessions, check whether yesterday had sessions — if yes, streak is still alive).

**Alternative considered**: SQL recursive CTE — rejected (complex, hard to test, timezone-sensitive).

### 5. Heatmap intensity scale: fixed 4-level

**Decision**: 0 rounds → level 0 (empty); 1–3 → level 1; 4–7 → level 2; 8+ → level 3. Colors derived from `--color-focus-round` at varying opacity via `color-mix(in oklch, var(--color-focus-round) N%, var(--color-background))`.

**Rationale**: A fixed scale is predictable and immediately legible (the user always knows what "full" looks like). A relative scale (max = personal best) would shift meaning as data grows.

**Alternative considered**: Relative-to-max scale — deferred to a future iteration.

### 6. IPC shape: two new commands

**Decision**:

- `stats_get_detailed` → `{ today: DailyStats, week: Vec<DayStat>, streak: StreakInfo }` — batched to minimize round-trips for the common case (Today + This Week tabs)
- `stats_get_heatmap` → `Vec<HeatmapEntry>` — separate command since it queries a full year of data and is only needed for the All Time tab

**Rationale**: Batching daily + weekly into one call avoids two sequential async invocations on tab switch. Heatmap is separated because it's heavier and only needed on demand.

## Risks / Trade-offs

- **SQLite `'localtime'` accuracy**: The OS timezone is used at query time, not at session-recording time. If a user changes their timezone while the app is open, historical grouping could shift. This is an acceptable edge case — the data itself (unix timestamps) is always correct; only the day-grouping presentation changes.
- **Heatmap width**: 52 weeks × (cell + gap) requires ~800px. Window is set to 820×540. If the OS scales the window differently (HiDPI), the grid may overflow. Mitigation: use `overflow-x: auto` on the heatmap container.
- **Empty state**: New users will see mostly empty charts. Each view needs a graceful empty state rather than showing a blank SVG grid.

## Open Questions

_None — all decisions resolved during explore phase._
