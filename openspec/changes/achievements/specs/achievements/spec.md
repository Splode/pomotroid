## ADDED Requirements

### Requirement: Achievement definitions are static and evaluated from session data
The system SHALL maintain a static list of 15 achievement definitions. Each definition includes an identifier, display name, description, category (Milestone / Habit / Discovery), secret flag, badge color, icon, and a criteria function evaluated against aggregated session data. Achievement criteria SHALL be evaluated from existing session, streak, and heatmap data — no new data collection is required.

#### Scenario: Milestone achievement evaluated from total session count
- **WHEN** the system checks achievements after a session completes
- **THEN** the system queries total completed work sessions and compares against each milestone threshold

#### Scenario: Secret achievement hidden from frontend until earned
- **WHEN** the system serializes an unearned achievement with `secret = true`
- **THEN** the name, description, and color fields are omitted from the response; only the category and a locked state indicator are included

#### Scenario: Progress reported for milestone achievements
- **WHEN** the system serializes an unearned milestone achievement (e.g., The Centurion)
- **THEN** the response includes `progress_current` and `progress_total` fields (e.g., 47 and 100)

---

### Requirement: The 15 achievements
The system SHALL implement exactly the following achievements:

| ID | Name | Category | Secret | Criteria |
|---|---|---|---|---|
| `the_seed` | The Seed | Milestone | No | 1 completed work session |
| `hat_trick` | Hat Trick | Milestone | No | ≥ 3 sessions in a single calendar day |
| `the_centurion` | The Centurion | Milestone | No | 100 completed work sessions |
| `tomato_baron` | Tomato Baron | Milestone | No | 500 completed work sessions |
| `tomato_tycoon` | Tomato Tycoon | Milestone | No | 1,000 completed work sessions |
| `time_lord` | Time Lord | Milestone | No | 100 hours of completed focus time |
| `on_a_roll` | On a Roll | Habit | No | Longest streak ≥ 3 days |
| `week_warrior` | Week Warrior | Habit | No | Longest streak ≥ 7 days |
| `month_of_zen` | Month of Zen | Habit | No | Longest streak ≥ 30 days |
| `comeback_kid` | Comeback Kid | Habit | Yes | Return with a new session after a gap (had prior sessions, current streak = 1 after a break) |
| `early_bird` | Early Bird | Discovery | Yes | Any completed work session started before 07:00 local time |
| `midnight_oil` | Midnight Oil | Discovery | Yes | Any completed work session started at or after 23:00 local time |
| `perfect_day` | Perfect Day | Discovery | Yes | Any calendar day with ≥ 4 started sessions and 100% completion rate |
| `the_long_haul` | The Long Haul | Discovery | No | Any long-break session marked `completed = 1` |
| `creature_of_habit` | Creature of Habit | Discovery | Yes | ≥ 5 distinct calendar days with a completed session in the same clock-hour |
| `theme_artist` | Theme Artist | Discovery | No | ≥ 1 row in the `custom_themes` table |

#### Scenario: First work session earns The Seed
- **WHEN** a user completes their very first work session
- **THEN** `the_seed` is inserted into the `achievements` table and an unlock event is emitted

#### Scenario: Streak milestone uses longest streak, not current
- **WHEN** a user's longest-ever streak reaches 7 days but their current streak is only 2
- **THEN** `week_warrior` is earned and is never revoked when the current streak resets

#### Scenario: Comeback Kid fires after a gap, not on first ever session
- **WHEN** a user has prior session history, broke their streak (gap of ≥ 2 days), and completes a new session starting a fresh streak
- **THEN** `comeback_kid` is earned

#### Scenario: Creature of Habit requires the same clock-hour, not same time of day generally
- **WHEN** a user has completed sessions at 09:15, 09:45, 09:02, 09:30, and 09:58 on five different calendar days
- **THEN** `creature_of_habit` is earned (all fall within hour 9)

#### Scenario: Theme Artist unlocks when first custom theme is created
- **WHEN** a user saves their first custom theme via the Appearance settings
- **THEN** `theme_artist` is earned immediately via the `ThemeCreated` trigger

---

### Requirement: Achievements are persisted at the moment of first unlock
The system SHALL maintain an `achievements` table in SQLite with columns `id TEXT PRIMARY KEY` and `unlocked_at INTEGER NOT NULL` (Unix timestamp). An achievement is inserted exactly once — when it transitions from unearned to earned. Subsequent checks MUST NOT re-insert or overwrite the record.

#### Scenario: Achievement inserted only on first unlock
- **WHEN** an achievement's criteria are met for the first time
- **THEN** a row is inserted into `achievements` with the current Unix timestamp

#### Scenario: Already-earned achievement not re-inserted
- **WHEN** the achievement check runs and an achievement's criteria are still met
- **THEN** if the achievement's `id` already exists in the `achievements` table, no insert or update occurs

#### Scenario: Unlock timestamp visible in gallery
- **WHEN** a user views an earned achievement in the gallery
- **THEN** the achievement card displays the formatted `unlocked_at` date

#### Scenario: Clearing session data also clears achievements
- **WHEN** a user triggers "Clear all session data" from the Data section
- **THEN** the `achievements` table is truncated in the same operation, the gallery reflects zero earned achievements, and a refresh event is emitted to all open windows

---

### Requirement: Achievement evaluation is trigger-driven
The system SHALL expose a `check_achievements(trigger, app_handle)` function callable from any event handler. Each achievement definition SHALL declare which trigger types can cause it to unlock. The evaluator SHALL skip achievements whose trigger set does not include the current trigger. Defined triggers are: `SessionComplete`, `ThemeCreated`. New triggers may be added as new achievement types are introduced.

#### Scenario: Session-complete trigger only evaluates session-based achievements
- **WHEN** `check_achievements` is called with `Trigger::SessionComplete`
- **THEN** only achievements that declare `SessionComplete` in their trigger set are evaluated

#### Scenario: Theme-created trigger only evaluates theme-based achievements
- **WHEN** `check_achievements` is called with `Trigger::ThemeCreated`
- **THEN** only achievements that declare `ThemeCreated` in their trigger set are evaluated

#### Scenario: Check fires after natural session completion
- **WHEN** a work round timer reaches zero and the session is marked complete
- **THEN** `check_achievements(SessionComplete)` runs; it SHALL NOT block the timer thread

#### Scenario: Check fires after skip
- **WHEN** a user skips a round (TimerEvent::Complete { skipped: true })
- **THEN** `check_achievements(SessionComplete)` still runs

#### Scenario: Check fires after custom theme is created
- **WHEN** a user saves a new custom theme via the appearance settings
- **THEN** `check_achievements(ThemeCreated)` runs from the theme creation command handler

#### Scenario: Newly earned achievements emitted as event
- **WHEN** one or more achievements are newly unlocked during any check
- **THEN** the system emits an `achievement:unlocked` event to all windows with `{ ids: Vec<String>, count: u32 }`

---

### Requirement: Corner toast window appears on achievement unlock
The system SHALL create a short-lived decoration-free `WebviewWindow` anchored at the bottom-right of the primary monitor when one or more achievements unlock, provided the `achievement_notifications` setting is `true`.

#### Scenario: Single achievement unlock shows dedicated toast
- **WHEN** exactly one achievement is newly unlocked
- **THEN** the toast window shows the achievement's badge icon, name, and "Achievement Unlocked!" header

#### Scenario: Multiple simultaneous unlocks show rollup toast
- **WHEN** two or more achievements unlock in the same check
- **THEN** the toast window shows up to 3 badge icons, the count, and "You unlocked N achievements!"

#### Scenario: Toast window auto-closes after ~4.5 seconds
- **WHEN** the toast window has been visible for approximately 4.5 seconds
- **THEN** the window closes itself

#### Scenario: Toast not shown when notifications disabled
- **WHEN** `achievement_notifications` is `false` and an achievement is newly unlocked
- **THEN** the achievement is recorded in the DB but no toast window is created

#### Scenario: Window creation failure does not affect achievement recording
- **WHEN** the toast window cannot be created (e.g., compositor limitation)
- **THEN** the error is logged and the achievement remains recorded in the DB

---

### Requirement: Toast celebrates the unlock with animation and sound
The toast window SHALL display a celebratory animation sequence and play a bundled chime on appearance.

#### Scenario: Toast slides up and fades in
- **WHEN** the toast window mounts
- **THEN** it animates from `translateY(40px) opacity(0)` to `translateY(0) opacity(1)` over 300ms ease-out

#### Scenario: Badge icon pops in with spring animation
- **WHEN** the toast mounts
- **THEN** the achievement badge icon scales from 0 → 1.2 → 0.95 → 1 over approximately 400ms

#### Scenario: Sparkle particles burst from badge
- **WHEN** the toast mounts
- **THEN** 6 small star-shaped elements radiate outward from the badge icon center and fade as they travel, using CSS keyframes

#### Scenario: Chime plays on mount
- **WHEN** the toast window appears
- **THEN** a soft bundled chime audio file plays once (not user-replaceable, separate from alert sounds)

#### Scenario: Toast border glows with achievement color
- **WHEN** the toast is visible
- **THEN** the toast has a `box-shadow` incorporating the achievement's badge color

---

### Requirement: Achievement gallery is a dedicated tab in the Statistics window
The system SHALL add a fourth tab labeled "Achievements" to the Statistics window tab bar. The gallery SHALL display all 15 achievements grouped into three category sections: Milestone, Habit, and Discovery.

#### Scenario: Earned achievement displays full detail
- **WHEN** a user views an earned achievement in the gallery
- **THEN** the badge renders at full color with the achievement's name, description, and unlock date

#### Scenario: Unearned visible achievement shows greyed badge with progress
- **WHEN** a user views an unearned non-secret achievement
- **THEN** the badge renders desaturated (`filter: grayscale(1) opacity(0.4)`), the name and description are shown, and milestone achievements include a `current / total` progress indicator

#### Scenario: Unearned secret achievement renders as locked placeholder
- **WHEN** a user views an unearned secret achievement
- **THEN** the badge renders with a neutral fill and lock icon, name shows "???", and no description or progress is shown

#### Scenario: Gallery loads current achievement state on tab open
- **WHEN** a user opens the Achievements tab
- **THEN** the frontend calls `achievements_get_all` and renders the returned state

---

### Requirement: Achievement notification toggle in Notifications settings
The system SHALL add an "Achievement Notifications" toggle to the Notifications section of the Settings window. The setting SHALL default to `true`. When disabled, achievement toast windows and chime are suppressed; achievements still unlock and appear in the gallery.

#### Scenario: Toggle defaults to enabled
- **WHEN** a user opens Pomotroid for the first time after migration v6
- **THEN** `achievement_notifications` is `true` and the toggle is on

#### Scenario: Disabling suppresses future toasts
- **WHEN** a user disables Achievement Notifications
- **THEN** subsequent achievement unlocks are recorded silently with no corner window or chime

#### Scenario: Re-enabling restores toast behavior
- **WHEN** a user re-enables Achievement Notifications
- **THEN** the next achievement unlock produces a corner toast window and chime

#### Scenario: Resetting all settings restores notification opt-in to default
- **WHEN** a user triggers "Reset all settings" from the About section
- **THEN** `achievement_notifications` is restored to `true` (included in `seed_defaults`)

---

### Requirement: Achievement badge uses consistent visual format
The system SHALL render all achievement badges as a 64×64 rounded square (rx=14), filled with the achievement's defined color, containing a white icon centered at 32,32. A single `AchievementBadge` component SHALL accept `color`, `icon_paths` (SVG path data), and optional `emoji` (fallback for MVP) props.

#### Scenario: Badge renders with color fill and white icon
- **WHEN** an earned achievement badge is rendered
- **THEN** the rounded square background uses the achievement's defined hex color and the icon paths are white

#### Scenario: Emoji fallback renders when no icon paths provided
- **WHEN** an achievement definition has no `icon_paths` and provides an `emoji` string
- **THEN** the emoji is centered within the rounded square background

#### Scenario: Unearned badge applies desaturation filter
- **WHEN** the badge wrapper has `earned = false`
- **THEN** a CSS filter of `grayscale(1) opacity(0.4)` is applied to the entire badge element
