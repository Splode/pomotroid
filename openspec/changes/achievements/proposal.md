## Why

Pomotroid tracks rich session history but offers no recognition for the effort users invest. An achievements system rewards consistency and milestones with moments of genuine delight — reinforcing the habit loop that makes a Pomodoro timer valuable over time.

## What Changes

- **New achievements gallery** — a dedicated "Achievements" tab in the Statistics window, showing all 15 achievements grouped by category (Milestone, Habit, Discovery). Earned achievements display full color, unlock date, and the badge icon. Unearned visible achievements show progress toward criteria. Secret achievements remain hidden until earned.
- **Steam-style corner toast** — a short-lived, decoration-free `WebviewWindow` anchors at the bottom-right corner of the display when an achievement is unlocked. It appears regardless of which window (or app) is focused, plays a soft chime, and runs a celebratory animation (slide-up, icon pop, sparkle burst). Auto-closes after ~4.5 seconds.
- **Achievement notification toggle** — a new opt-in/opt-out setting in the Notifications section controls whether the corner toast (and chime) are shown. Achievements still unlock and appear in the gallery regardless.
- **Persistent unlock records** — a new `achievements` SQLite table (DB migration v6) stores the `id` and `unlocked_at` timestamp for each earned achievement. The backend checks for newly earned achievements after every completed session.
- **15 curated achievements** — a static, well-crafted set organised into three categories, evaluated from existing session/streak data with no new data collection needed.

## Capabilities

### New Capabilities

- `achievements`: The achievements system — definition, storage, evaluation, gallery UI, and corner toast notification.

### Modified Capabilities

- `notifications`: Adding the achievement notification opt-out toggle to the Notifications settings section.

## Impact

- **New Tauri window** (`achievement-toast`) — requires a new SvelteKit route, new capability entry if needed, and `WebviewWindowBuilder` usage in Rust.
- **DB migration v6** — adds `achievements (id TEXT PRIMARY KEY, unlocked_at INTEGER NOT NULL)` table; backward-compatible, no data loss.
- **Timer completion path** — `complete_session()` call site gains a post-completion achievement check; must not block the timer thread.
- **Settings** — one new key (`achievement_notifications`, default `true`) added to the settings system (SQLite + Rust struct + frontend store + UI toggle).
- **Audio** — one new bundled chime sound asset (not user-replaceable; separate from the existing alert sound system).
- **Stats window** — gains a fourth tab; existing tab layout must accommodate the new label without truncation.
- **No external dependencies** — all animations via CSS keyframes; all achievement evaluation from existing DB queries.
