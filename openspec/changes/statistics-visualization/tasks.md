## 1. Rust — Data Layer

- [ ] 1.1 Add `get_daily_stats(conn, today_epoch_start)` query in `queries.rs` — returns rounds, minutes, completion rate, and per-hour counts (using `'localtime'` modifier)
- [ ] 1.2 Add `get_weekly_stats(conn)` query — returns `Vec<DayStat>` for the last 7 local calendar days (date string + completed round count)
- [ ] 1.3 Add `get_heatmap_data(conn)` query — returns `Vec<HeatmapEntry>` (date string + count) for all days with at least 1 completed work session in the last 52 weeks
- [ ] 1.4 Add `compute_streak(days: &[NaiveDate]) -> StreakInfo` function in Rust — takes sorted list of days with sessions, returns `{ current: u32, longest: u32 }` applying the "active until midnight" rule
- [ ] 1.5 Add unit tests for `compute_streak` covering: active streak with no session today, active streak with session today, broken streak, longest-streak tracking

## 2. Rust — Commands

- [ ] 2.1 Add `stats_get_detailed` command in `commands.rs` — queries daily stats + weekly stats + streak, returns combined struct `DetailedStats { today: DailyStats, week: Vec<DayStat>, streak: StreakInfo }`
- [ ] 2.2 Add `stats_get_heatmap` command in `commands.rs` — calls `get_heatmap_data` and returns `Vec<HeatmapEntry>` plus lifetime totals (total rounds, total hours, longest streak)
- [ ] 2.3 Register both commands in `generate_handler!` in `lib.rs`

## 3. Config & IPC

- [ ] 3.1 Add `"stats"` to the `windows` array in `capabilities/default.json`
- [ ] 3.2 Add `statsGetDetailed()` and `statsGetHeatmap()` typed invoke wrappers in `src/lib/ipc/index.ts`
- [ ] 3.3 Add corresponding TypeScript types for `DetailedStats`, `DailyStats`, `DayStat`, `StreakInfo`, `HeatmapEntry`, `HeatmapStats` in `src/lib/types.ts`

## 4. Titlebar Entry Point

- [ ] 4.1 Add `openStats()` function to `Titlebar.svelte` — mirrors `openSettings()` pattern (get-by-label, focus if exists, else create new `WebviewWindow('stats', { url: '/stats', width: 820, height: 540, resizable: false, decorations: isMac })`)
- [ ] 4.2 Add stats icon button (bar-chart SVG) to the titlebar alongside the settings button — left side on Linux/Windows, right side on macOS

## 5. Stats Window — Shell

- [ ] 5.1 Create `src/routes/stats/+page.svelte` — full page shell with theme initialization (same `onMount` pattern as settings page), three-tab nav (Today / This Week / All Time), default tab = Today
- [ ] 5.2 Create stats-specific titlebar (or reuse/extend `SettingsTitlebar`) with window title and close button

## 6. Frontend — Today Tab

- [ ] 6.1 Create `src/lib/components/stats/StatCard.svelte` — reusable card component displaying a label, large numeric value, and optional unit string
- [ ] 6.2 Create `src/lib/components/stats/DailyView.svelte` — three `StatCard` instances (Rounds, Focus Time, Completion Rate) + hourly bar chart SVG (24 bars, each bar = rounds in that hour); include empty state when no data
- [ ] 6.3 Wire `statsGetDetailed` IPC call in the stats page; pass `today` data to `DailyView`

## 7. Frontend — This Week Tab

- [ ] 7.1 Create `src/lib/components/stats/WeeklyView.svelte` — 7-bar SVG bar chart (one bar per day, today's bar highlighted with `--color-focus-round`, past days with `--color-accent`); streak counter displayed below chart; include empty state
- [ ] 7.2 Wire `week` and `streak` data from `statsGetDetailed` response to `WeeklyView`

## 8. Frontend — All Time Tab

- [ ] 8.1 Create `src/lib/components/stats/HeatmapGrid.svelte` — 52×7 SVG grid; cells colored via `color-mix(in oklch, var(--color-focus-round) N%, var(--color-background))` at 4 levels (0%, 30%, 60%, 100%); month labels above columns; tooltip on hover showing date + count
- [ ] 8.2 Create `src/lib/components/stats/YearlyView.svelte` — `HeatmapGrid` + three lifetime stat cards (Total Rounds, Total Hours, Longest Streak); include empty state for first launch
- [ ] 8.3 Wire `statsGetHeatmap` IPC call (load on All Time tab activation); pass data to `YearlyView`; handle `settings:changed` / `themes:changed` events to re-apply theme if received while open

## 9. Polish & Verification

- [ ] 9.1 Verify `cargo test` passes (all existing + new streak unit tests)
- [ ] 9.2 Verify `npm run check` passes (no TypeScript errors in new files)
- [ ] 9.3 Manual smoke test: open stats window, complete a work round, confirm Today tab updates on next open; verify heatmap renders correctly on All Time tab
- [ ] 9.4 Verify stats window respects active theme (open stats window, switch theme in settings, confirm colors update)
