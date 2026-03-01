## 1. Rust — Data Layer

- [x] 1.1 Add `get_daily_stats(conn)` query in `queries.rs` — returns rounds, minutes, completion rate, and per-hour counts (today resolved internally via SQLite `date('now', 'localtime')`)
- [x] 1.2 Add `get_weekly_stats(conn)` query — returns `Vec<DayStat>` for the last 7 local calendar days (date string + completed round count)
- [x] 1.3 Add `get_heatmap_data(conn)` query — returns `Vec<HeatmapEntry>` (date string + count) for all days with at least 1 completed work session, all-time (no date limit; frontend paginates by year)
- [x] 1.4 Add `compute_streak(days: &[String]) -> StreakInfo` function in Rust — takes sorted list of "YYYY-MM-DD" date strings, returns `{ current: u32, longest: u32 }` applying the "active until midnight" rule; uses custom day-number arithmetic (no chrono dependency)
- [x] 1.5 Add unit tests for `compute_streak` covering: active streak with no session today, active streak with session today, broken streak, longest-streak tracking

## 2. Rust — Commands

- [x] 2.1 Add `stats_get_detailed` command in `commands.rs` — queries daily stats + weekly stats + streak, returns combined struct `DetailedStats { today: DailyStats, week: Vec<DayStat>, streak: StreakInfo }`
- [x] 2.2 Add `stats_get_heatmap` command in `commands.rs` — calls `get_heatmap_data` and returns `Vec<HeatmapEntry>` plus lifetime totals (total rounds, total hours, longest streak)
- [x] 2.3 Register both commands in `generate_handler!` in `lib.rs`

## 3. Config & IPC

- [x] 3.1 Add `"stats"` to the `windows` array in `capabilities/default.json`
- [x] 3.2 Add `statsGetDetailed()` and `statsGetHeatmap()` typed invoke wrappers in `src/lib/ipc/index.ts`
- [x] 3.3 Add corresponding TypeScript types for `DetailedStats`, `DailyStats`, `DayStat`, `StreakInfo`, `HeatmapEntry`, `HeatmapStats` in `src/lib/types.ts`

## 4. Titlebar Entry Point

- [x] 4.1 Add `openStats()` function to `Titlebar.svelte` — mirrors `openSettings()` pattern (get-by-label, focus if exists, else create new `WebviewWindow('stats', { url: '/stats', width: 840, height: 520, resizable: false, decorations: isMac })`)
- [x] 4.2 Add stats icon button (bar-chart SVG) to the titlebar alongside the settings button — left side on Linux/Windows, right side on macOS

## 5. Stats Window — Shell

- [x] 5.1 Create `src/routes/stats/+page.svelte` — full page shell with theme initialization (same `onMount` pattern as settings page), three-tab nav (Today / This Week / All Time), default tab = Today
- [x] 5.2 Stats-specific titlebar with window title and close button, inline in `+page.svelte`

## 6. Frontend — Today Tab

- [x] 6.1 ~~Create `src/lib/components/stats/StatCard.svelte`~~ — dropped; stat cards implemented inline in `DailyView.svelte` (no reuse elsewhere)
- [x] 6.2 Create `src/lib/components/stats/DailyView.svelte` — three inline stat cards (Rounds, Focus Time, Completion Rate) + hourly bar chart SVG (24 bars, each bar = rounds in that hour); include empty state when no data
- [x] 6.3 Wire `statsGetDetailed` IPC call in the stats page; pass `today` data to `DailyView`

## 7. Frontend — This Week Tab

- [x] 7.1 Create `src/lib/components/stats/WeeklyView.svelte` — 7-bar SVG bar chart (one bar per day, today's bar highlighted with `--color-focus-round`, past days muted); streak counter displayed above chart; include empty state
- [x] 7.2 Wire `week` and `streak` data from `statsGetDetailed` response to `WeeklyView`

## 8. Frontend — All Time Tab

- [x] 8.1 ~~Create `src/lib/components/stats/HeatmapGrid.svelte`~~ — dropped; heatmap grid implemented inline in `YearlyView.svelte` (no reuse elsewhere)
- [x] 8.2 Create `src/lib/components/stats/YearlyView.svelte` — inline 52–53 week heatmap grid + year navigation (prev/next) + three lifetime stat cards (Total Rounds, Total Hours, Longest Streak); include empty state for first launch
- [x] 8.3 Wire `statsGetHeatmap` IPC call (load on All Time tab activation); pass data to `YearlyView`; handle `settings:changed` / `themes:changed` events to re-apply theme if received while open

## 9. Polish & Verification

- [x] 9.1 Verify `cargo test` passes (all existing + new streak unit tests) — 60/60 pass
- [x] 9.2 Verify `npm run check` passes (no TypeScript errors in new files) — 0 errors, 0 warnings
- [x] 9.3 Manual smoke test: stats window updates live on round completion via `onRoundChange` (not just on next open); heatmap renders correctly on All Time tab
- [x] 9.4 Verify stats window respects active theme — `onSettingsChanged` and `onThemesChanged` both handled; locale changes also reactive via `setLocale`
