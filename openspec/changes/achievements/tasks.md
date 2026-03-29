## 1. Database & Settings Foundation

- [x] 1.1 Add migration v6 to `src-tauri/src/db/migrations.rs`: create `achievements (id TEXT PRIMARY KEY, unlocked_at INTEGER NOT NULL)` table and insert `achievement_notifications = 'true'` setting if absent
- [x] 1.2 Add `achievement_notifications: bool` field to the `Settings` struct in `src-tauri/src/settings/mod.rs`, with DB key `achievement_notifications`, default `true`; include it in `seed_defaults()` so "Reset all settings" restores it to `true`
- [x] 1.3 Add `achievement_notifications` to the frontend `Settings` type in `src/lib/types.ts`
- [x] 1.4 Add the Achievement Notifications toggle to `src/lib/components/settings/sections/NotificationsSection.svelte`

## 2. Achievement Definitions & DB Queries (Rust)

- [x] 2.1 Create `src-tauri/src/achievements/mod.rs` with the `AchievementDef` struct (id, name, description, category, secret, color, emoji, progress kind, triggers: `&[Trigger]`), the `Trigger` enum (`SessionComplete`, `ThemeCreated`), and the static `ACHIEVEMENTS` array of all 16 definitions
- [x] 2.2 Add `AchievementView` struct (serializable to frontend: all fields + earned, unlocked_at, progress_current, progress_total; secret+unearned fields nulled out)
- [x] 2.3 Add DB query `get_earned_achievement_ids(conn) -> HashSet<String>` to `src-tauri/src/db/queries.rs`
- [x] 2.4 Add DB query `insert_achievement(conn, id, unlocked_at) -> Result<()>` to `src-tauri/src/db/queries.rs`
- [x] 2.5 Add DB query `get_hat_trick_max_day(conn) -> u32` (max completed work sessions in any single calendar day) to `src-tauri/src/db/queries.rs`
- [x] 2.6 Add DB query `get_early_sessions(conn) -> bool` (any completed work session with local hour < 7) to `src-tauri/src/db/queries.rs`
- [x] 2.7 Add DB query `get_midnight_sessions(conn) -> bool` (any completed work session with local hour ≥ 23) to `src-tauri/src/db/queries.rs`
- [x] 2.8 Add DB query `get_perfect_day_exists(conn) -> bool` (any calendar day with ≥ 4 started sessions and 100% completion rate) to `src-tauri/src/db/queries.rs`
- [x] 2.9 Add DB query `get_creature_of_habit(conn) -> bool` (≥ 5 distinct days sharing the same clock-hour) to `src-tauri/src/db/queries.rs`
- [x] 2.10 Add DB query `get_comeback_kid(conn) -> bool` (has prior sessions + current streak = 1 with a gap of ≥ 2 days before the most recent session) to `src-tauri/src/db/queries.rs`
- [x] 2.11 Add DB query `get_long_break_completed(conn) -> bool` (any long-break session with completed = 1) to `src-tauri/src/db/queries.rs`
- [x] 2.12 Add DB query `get_has_custom_theme(conn) -> bool` (any row in `custom_themes` table) to `src-tauri/src/db/queries.rs` — implemented via filesystem check in eval.rs (custom themes live in the filesystem, not the DB)

## 3. Achievement Evaluation & Event Emission (Rust)

- [x] 3.1 Create `src-tauri/src/achievements/eval.rs` with `check_achievements(trigger: Trigger, conn, app_handle) -> Result<Vec<String>>`: filter ACHIEVEMENTS by trigger, gather needed stats, compare against criteria, insert newly earned ones, return their IDs
- [x] 3.2 Wire `check_achievements(Trigger::SessionComplete, ...)` into the session completion path in `src-tauri/src/timer/mod.rs` — call after `complete_session()`, outside the timer's hot loop
- [x] 3.3 Wire `check_achievements(Trigger::ThemeCreated, ...)` into the themes watcher's `reload_and_emit` in `src-tauri/src/themes/watcher.rs` (custom themes are filesystem-based, not a command)
- [x] 3.4 After evaluation, if newly earned IDs exist: check `achievement_notifications` setting; if true, emit `achievement:unlocked` event to all windows with `{ ids, count }`
- [x] 3.5 Add Tauri command `achievements_get_all` in `src-tauri/src/commands.rs`: load earned IDs from DB, build `Vec<AchievementView>` from ACHIEVEMENTS static list, return serialized

## 4. IPC & Frontend Store

- [x] 4.1 Add `achievementsGetAll(): Promise<AchievementView[]>` to `src/lib/ipc/index.ts`
- [x] 4.2 Add `onAchievementUnlocked(cb)` event listener to `src/lib/ipc/index.ts`
- [x] 4.3 Add `AchievementView` TypeScript type to `src/lib/types.ts` matching the Rust struct

## 5. Achievement Badge Component

- [x] 5.1 Create `src/lib/components/AchievementBadge.svelte`: 64×64 SVG with rounded square background (`rx=14`) filled with `color` prop, white icon paths from `icon_paths` prop, emoji fallback when `icon_paths` is empty, lock icon and neutral fill when `locked` prop is true
- [x] 5.2 Apply `filter: grayscale(1) opacity(0.4)` to badge wrapper when `earned = false` and not locked

## 6. Achievements Gallery Tab

- [x] 6.1 Create `src/lib/components/stats/AchievementsTab.svelte`: calls `achievementsGetAll()` on mount, groups achievements by category (Milestone → Habit → Discovery), renders a grid of `AchievementCard` components per section
- [x] 6.2 Create `src/lib/components/stats/AchievementCard.svelte`: renders `AchievementBadge`, achievement name (or "???"), description (or hidden for secret+unearned), unlock date for earned, progress indicator (`current / total`) for unearned milestone achievements
- [x] 6.3 Add "Achievements" as the fourth tab in `src/routes/stats/+page.svelte`, rendering `AchievementsTab` when active; listen for `achievement:unlocked` event and refresh the tab's data if it is currently active

## 7. Toast Window

- [x] 7.1 Create SvelteKit route `src/routes/achievement-toast/+page.svelte`: parses achievement data from URL search params (`?ids=...&count=N`), looks up definitions, renders toast UI (single achievement or rollup)
- [x] 7.2 Implement toast animation in CSS: slide-up entrance (translateY 40px → 0, 300ms ease-out), badge icon pop (scale 0 → 1.2 → 0.95 → 1, 400ms), 6 sparkle `<span>` elements with `--angle` CSS custom properties radiating from icon center
- [x] 7.3 Apply achievement-color glow border: `box-shadow: 0 0 0 1.5px var(--achievement-color), 0 0 24px 0 color-mix(in oklch, var(--achievement-color) 35%, transparent)`
- [x] 7.4 Add bundled chime audio asset to `src/assets/` (soft bell, .ogg format); play it on mount using the Web Audio API or an `<audio>` element — synthesized via Web Audio API (no asset needed)
- [x] 7.5 Implement auto-close: after 4200ms begin fade-out (300ms), then call `appWindow.close()` at 4500ms
- [x] 7.6 Add Rust helper in `src-tauri/src/achievements/toast.rs`: `spawn_toast_window(app_handle, ids, count)` — computes screen dimensions, creates the `WebviewWindow` with correct size (340×110), position (screen_width − 356, screen_height − 126), and properties (`decorations: false`, `transparent: true`, `always_on_top: true`, `skip_taskbar: true`); wraps in `Result` and logs on error
- [x] 7.7 Call `spawn_toast_window` from step 3.3 when notifications are enabled and achievements were newly unlocked
- [x] 7.8 Add `achievement-toast` window to Tauri capabilities in `src-tauri/capabilities/default.json` if it requires permissions beyond the shared default

## 8. Data Lifecycle Correctness

- [x] 8.1 Update `sessions_clear` in `src-tauri/src/commands.rs` to also execute `DELETE FROM achievements` in the same operation, then emit `achievement:unlocked` (with empty payload or a dedicated `achievements:cleared` event) so any open Statistics window refreshes the gallery
- [x] 8.2 Update `AchievementsTab.svelte` to listen for the clear event and reset its displayed state to all-unearned

## 9. Verification

- [x] 9.1 Run `npm run check` — zero Svelte type errors
- [x] 9.2 Run `cargo test` in `src-tauri/` — all existing tests pass; add unit tests for `compute_streak` edge cases relevant to achievement criteria if not already covered
- [ ] 9.3 Manually verify: complete a work session → The Seed unlocks → toast appears at bottom-right corner → chime plays → toast auto-closes
- [ ] 9.4 Manually verify: Achievements gallery tab shows correct earned/unearned/secret states; progress numbers are accurate
- [ ] 9.5 Manually verify: disabling Achievement Notifications suppresses toast and chime; achievement still appears in gallery
- [ ] 9.6 Manually verify: "Reset all settings" restores Achievement Notifications toggle to on
- [ ] 9.7 Manually verify: "Clear all session data" empties the achievements gallery
