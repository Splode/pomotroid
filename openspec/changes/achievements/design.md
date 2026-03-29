## Context

Pomotroid has a mature session-tracking layer: every work round is recorded in SQLite with `started_at`, `ended_at`, `round_type`, `duration_secs`, and `completed`. Aggregates (total sessions, total focus hours, current/longest streak, heatmap by day) are already computed by `db/queries.rs` and exposed via IPC. The `custom_themes` table records user-created themes. Together these data sources are sufficient to evaluate all planned achievements — no new data collection is required.

The Settings system (SQLite key/value → Rust `Settings` struct → frontend store) provides a clear pattern for adding the `achievement_notifications` toggle. The `WebviewWindow` creation pattern is already used for the settings window.

## Goals / Non-Goals

**Goals:**
- Persistent achievement unlock records with timestamps (SQLite, migration v6)
- 16 well-crafted achievements evaluated from existing session and theme data, checked on relevant trigger events
- Celebratory corner `WebviewWindow` toast with animation and chime — appears regardless of focused window
- Achievement gallery as a 4th tab in the Statistics window, grouped by category
- Opt-out notification toggle in the Notifications settings section
- English-only strings for MVP (localization deferred)

**Non-Goals:**
- Cloud sync or sharing of achievements
- User-defined or unlockable custom achievements
- Achievement points / XP / leaderboards
- Retroactive notifications for achievements already earned at feature launch (they unlock silently on first check)
- Localization of achievement strings in MVP

## Decisions

### Achievement evaluation: computed on-the-fly vs. persisted unlock records

**Decision:** Hybrid — compute which achievements *are* earned from session data on every check; persist only the *unlock moment* (first time earned) in the `achievements` table.

**Rationale:** Pure computed approach can't distinguish "just unlocked" from "already known". Without persisted records, we cannot fire the toast exactly once or display the unlock date in the gallery. Pure persistence (writing every state) is redundant. The hybrid gives us the unlock timestamp and "new this check" detection with minimal schema.

**Alternative considered:** Storing a `notified` flag alongside `unlocked_at`. Rejected — the notification opt-out is a setting checked at toast-creation time, not stored per achievement.

---

### Achievement check timing: trigger-driven evaluation

**Decision:** Expose a `check_achievements(trigger: Trigger, app_handle)` function callable from any event handler. Each `AchievementDef` declares which `Trigger` variants can cause it to unlock. The evaluator skips achievements whose trigger set does not include the current trigger.

```rust
pub enum Trigger {
    SessionComplete,
    ThemeCreated,
    // future: AppOpened, SettingChanged, ...
}
```

**Rationale:** Hardwiring checks to `complete_session()` prevents achievements that respond to other user actions (creating a custom theme, changing a setting, opening the app on a specific day, etc.). The trigger enum makes the relationship between actions and achievements explicit, keeps evaluation efficient (only relevant criteria run per trigger), and makes the system straightforwardly extensible — adding a new achievement type means adding a trigger variant and wiring it into one call site.

**Alternative considered:** Check all achievements on every trigger regardless of relevance. Rejected — as the achievement set grows this becomes wasteful and makes it harder to reason about which actions can cause which unlocks.

**Implementation note:** `check_achievements` must not block the timer thread for `SessionComplete` triggers. For other triggers (e.g., `ThemeCreated` fired from a Tauri command handler) it runs on the command handler's thread, which is acceptable.

---

### Toast window: in-app vs. Steam-style corner `WebviewWindow`

**Decision:** A short-lived `WebviewWindow` anchored at the bottom-right of the primary monitor.

**Rationale:** Achievements often unlock at session completion, which is exactly when users context-switch to other apps. An in-app toast on the timer or stats window is invisible when the user is in their editor or browser. A dedicated corner window is visible everywhere, matching the use-case where Pomotroid runs in the background.

**Alternative considered:** OS desktop notifications (Tauri `notification` plugin). Rejected — OS notification styling is inconsistent across platforms, can't run animations, and goes to the notification center rather than appearing immediately.

**Alternative considered:** In-app toast on whichever Pomotroid window is focused. Rejected — if neither window is focused (common scenario), nothing appears.

**Window properties:** `decorations: false`, `transparent: true`, `always_on_top: true`, `skip_taskbar: true`, fixed size `340 × 110`, positioned `(screen_width - 356, screen_height - 126)` to clear typical taskbar/dock height.

---

### Multiple simultaneous unlocks

**Decision:** Show a single rollup toast — "You unlocked N achievements!" — with up to 3 badge icons displayed inline when N > 1.

**Rationale:** Queueing individual toasts would spam the corner and play the chime multiple times. This is most likely to occur when a user returns after a long absence. A rollup is friendlier.

---

### Achievement icon format

**Decision:** A single `AchievementBadge.svelte` component renders all badges. Each achievement definition provides a `color` (hex) and `icon_paths` (SVG path data as strings). The badge shape is a 64×64 rounded square (rx=14), filled with `color`, with white icon paths centered at 32,32.

**Rationale:** A consistent badge shape creates visual coherence in the gallery. Per-icon SVG components would require 15 separate files and make the gallery grid harder to render uniformly. The definition-driven approach keeps art data colocated with criteria.

**Unearned state:** CSS `filter: grayscale(1) opacity(0.4)` on the badge wrapper. No SVG changes needed.
**Secret+unearned state:** Replace icon with a lock path; replace name/description with "???" — the badge shape and color remain hidden (neutral dark fill).

**MVP fallback:** Emoji can substitute for SVG paths during development; the component accepts an optional `emoji` prop used if `icon_paths` is empty.

---

### Achievement definitions: Rust static vs. frontend JSON

**Decision:** Static `AchievementDef` array in Rust (`src-tauri/src/achievements/mod.rs`).

**Rationale:** Achievement criteria are evaluated in Rust after session completion. Keeping definitions in Rust avoids a round-trip to the frontend for evaluation. The frontend receives a serialized view of all achievements (with earned status and progress) via a single IPC command.

---

### The 15 achievements

| ID | Name | Description | Secret | Category | Criteria |
|---|---|---|---|---|---|
| `the_seed` | The Seed | Complete your first session | No | Milestone | total_work_sessions ≥ 1 |
| `hat_trick` | Hat Trick | Complete 3 sessions in one day | No | Milestone | max daily rounds ≥ 3 |
| `the_centurion` | The Centurion | Complete 100 sessions | No | Milestone | total_work_sessions ≥ 100 |
| `tomato_baron` | Tomato Baron | Complete 500 sessions | No | Milestone | total_work_sessions ≥ 500 |
| `tomato_tycoon` | Tomato Tycoon | Complete 1,000 sessions | No | Milestone | total_work_sessions ≥ 1000 |
| `time_lord` | Time Lord | Accumulate 100 hours of focus | No | Milestone | total_work_secs ≥ 360000 |
| `on_a_roll` | On a Roll | Maintain a 3-day streak | No | Habit | current_streak ≥ 3 OR longest_streak ≥ 3 |
| `week_warrior` | Week Warrior | Maintain a 7-day streak | No | Habit | longest_streak ≥ 7 |
| `month_of_zen` | Month of Zen | Maintain a 30-day streak | No | Habit | longest_streak ≥ 30 |
| `comeback_kid` | Comeback Kid | Return after a broken streak | Yes | Habit | had prior sessions + current_streak = 1 after a gap |
| `early_bird` | Early Bird | Complete a session before 7am | Yes | Discovery | any completed work session with hour(started_at) < 7 |
| `midnight_oil` | Midnight Oil | Complete a session after 11pm | Yes | Discovery | any completed work session with hour(started_at) ≥ 23 |
| `perfect_day` | Perfect Day | 100% completion rate with ≥ 4 sessions in a day | Yes | Discovery | any calendar day: started ≥ 4, completion_rate = 1.0 |
| `the_long_haul` | The Long Haul | Complete a full Pomodoro cycle | No | Discovery | any `long-break` session with completed = 1 |
| `creature_of_habit` | Creature of Habit | Work in the same hour on 5 different days | Yes | Discovery | ≥ 5 distinct calendar days with a completed session in the same clock-hour |
| `theme_artist` | Theme Artist | Create your first custom theme | No | Discovery | ≥ 1 row in `custom_themes` table |

---

### Streak-based achievements use `longest_streak`, not `current_streak`

**Decision:** "Week Warrior" and "Month of Zen" unlock when `longest_streak` reaches the threshold, not `current_streak`. "On a Roll" uses `longest_streak ≥ 3` so it is never un-earned.

**Rationale:** Achievements should not be revoked. A user who once had a 7-day streak and broke it has still demonstrated that capability.

---

### Badge colors per achievement

| Achievement | Color |
|---|---|
| The Seed | `#22c55e` (green) |
| Hat Trick | `#f97316` (orange) |
| The Centurion | `#eab308` (yellow) |
| Tomato Baron | `#ef4444` (red) |
| Tomato Tycoon | `#a855f7` (purple) |
| Time Lord | `#3b82f6` (blue) |
| On a Roll | `#fb923c` (amber-orange) |
| Week Warrior | `#10b981` (emerald) |
| Month of Zen | `#14b8a6` (teal) |
| Comeback Kid | `#ec4899` (pink) |
| Early Bird | `#fbbf24` (yellow) |
| Midnight Oil | `#6366f1` (indigo) |
| Perfect Day | `#34d399` (light emerald) |
| The Long Haul | `#d97706` (amber) |
| Creature of Habit | `#8b5cf6` (violet) |
| Theme Artist | `#f472b6` (pink) |

---

### New IPC surface

| Command/Event | Direction | Payload |
|---|---|---|
| `achievements_get_all` | Frontend → Rust | → `Vec<AchievementView>` |
| `achievement:unlocked` | Rust → Frontend (all windows) | `{ ids: Vec<String>, count: u32 }` |

`AchievementView` (serialized to frontend):
```
id, name, description, secret, color, emoji,
category, earned: bool, unlocked_at: Option<i64>,
progress_current: Option<u32>, progress_total: Option<u32>
```

Unearned secret achievements are serialized with `name = null`, `description = null`, `color = null` — the frontend renders a locked placeholder.

---

### Toast animation sequence

```
t=0ms    Window created; Svelte mounts; initial state: opacity 0, translateY(40px)
t=16ms   CSS transition starts: translateY(0), opacity 1 — 300ms ease-out
t=100ms  Badge icon scale animation: 0 → 1.2 → 0.95 → 1 — 400ms (cubic-bezier spring)
t=150ms  6 sparkle <span> elements animate outward (CSS keyframes, --angle per element)
t=200ms  Chime audio plays (bundled .ogg, ~0.5s)
t=4500ms Fade out: opacity → 0 — 300ms ease-in
t=4800ms Window closes via `appWindow.close()`
```

The toast window itself has a `box-shadow` glow using the achievement's color:
```css
box-shadow: 0 0 0 1.5px var(--achievement-color),
            0 0 24px 0 color-mix(in oklch, var(--achievement-color) 35%, transparent);
```

---

### New settings key

| DB key | Rust field | Default | Type |
|---|---|---|---|
| `achievement_notifications` | `achievement_notifications` | `true` | `bool` |

Added to `Settings` struct in `src-tauri/src/settings/mod.rs`, DB migration v6.

## Risks / Trade-offs

**[Risk] `always_on_top` window behavior varies by Linux WM** → Mitigation: test on GNOME and KDE; fall back gracefully if window appears but without always-on-top. The feature degrades to "visible when Pomotroid is focused" rather than failing.

**[Risk] Screen corner position doesn't account for taskbar position** → Mitigation: offset from bottom-right by a fixed 80px vertical buffer. Covers default taskbar heights on all platforms. A user with a very tall taskbar may see the toast partially obscured; acceptable for MVP.

**[Risk] "Comeback Kid" and "Creature of Habit" require additional DB queries not currently in `queries.rs`** → Mitigation: implement these as targeted SQL queries during the achievement evaluation pass. They run at most once per session completion and are not in the hot path.

**[Risk] Retroactive unlocks at feature launch** → Mitigation: on first run after migration v6, the achievement check runs after the next session completion. All previously-earned achievements will unlock silently (no toast) because the toast only fires for achievements newly added to the `achievements` table in the current session's check.

**[Risk] Toast window creation fails (e.g., Wayland compositors that don't support `always_on_top`)** → Mitigation: wrap window creation in a `Result`; log the error and continue. The achievement is still recorded in the DB; the gallery will reflect it correctly.

## Migration Plan

1. DB migration v6 runs automatically on app start (existing migration system).
2. The `achievements` table is empty on creation — no data migration needed.
3. The `achievement_notifications` setting defaults to `true` — inserted by migration if not present.
4. No rollback path needed; the table can be left empty if the feature is reverted and the migration skipped on an older build.
