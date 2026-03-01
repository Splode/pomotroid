## Why

Pomotroid already captures session data (round type, duration, completion) to SQLite on every round, but there is no UI to surface it. Users have no way to see how productive they've been — today, this week, or historically. Adding a dedicated Statistics window gives that data a home and makes Pomotroid meaningfully more useful as a focus tool.

## What Changes

- Add a **Statistics window** (third `WebviewWindow`, route `/stats`) accessible via a new chart icon in the main window titlebar
- Expose three **new Rust query functions** for daily, weekly, and yearly (heatmap) data, plus streak computation
- Add two **new Tauri commands**: `stats_get_detailed` (daily + weekly + streak) and `stats_get_heatmap` (52-week per-day counts)
- Add `stats` to the capability windows list so it inherits the shared permissions

## Capabilities

### New Capabilities

- `statistics`: Dedicated stats window with three tabs — Today (stat cards + hourly breakdown), This Week (daily bar chart + streak), All Time (annual heatmap + lifetime totals)

### Modified Capabilities

- `shortcuts`: No change to requirements — `shortcuts` window entry point pattern is being reused as a model (not a behavioral change)

## Impact

- **`src-tauri/src/db/queries.rs`**: New query functions (`get_daily_stats`, `get_weekly_stats`, `get_heatmap_data`, `get_streak`)
- **`src-tauri/src/commands.rs`**: Two new commands (`stats_get_detailed`, `stats_get_heatmap`)
- **`src-tauri/src/lib.rs`**: Register new commands in `generate_handler!`
- **`src-tauri/capabilities/default.json`**: Add `"stats"` to `windows` array
- **`src/lib/ipc/index.ts`**: Two new typed invoke wrappers
- **`src/routes/stats/+page.svelte`**: New stats route (full page with tabs)
- **`src/lib/components/stats/`**: New component directory — `DailyView.svelte`, `WeeklyView.svelte`, `YearlyView.svelte`, `StatCard.svelte`, `HeatmapGrid.svelte`
- **`src/lib/components/Titlebar.svelte`**: Add stats button alongside existing settings button
- No new npm dependencies — charts implemented as pure SVG, adapting to CSS custom properties
