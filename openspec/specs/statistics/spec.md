### Requirement: Statistics window accessible from main titlebar
The system SHALL provide a chart/stats icon button in the main window titlebar. Activating it SHALL open a dedicated statistics window (Tauri `WebviewWindow`, label `"stats"`, route `/stats`). If the window is already open, it SHALL be focused rather than opened a second time. The window SHALL be non-resizable, decoration-free on Linux/Windows (matching settings window behavior), and sized at approximately 820×540.

#### Scenario: Opening stats window for the first time
- **WHEN** the user clicks the stats icon in the main titlebar
- **THEN** a new statistics window opens at the `/stats` route

#### Scenario: Stats window already open
- **WHEN** the user clicks the stats icon while the statistics window is already open
- **THEN** the existing window is focused (no duplicate window created)

#### Scenario: Theme applied to stats window
- **WHEN** the statistics window opens
- **THEN** it applies the same active theme as the main window via CSS custom properties

---

### Requirement: Three-tab navigation within the statistics window
The statistics window SHALL display three tabs: **Today**, **This Week**, and **All Time**. The default active tab SHALL be **Today**. Switching tabs SHALL load the corresponding data view without reopening the window.

#### Scenario: Default tab on open
- **WHEN** the statistics window is first opened
- **THEN** the Today tab is active and Today data is displayed

#### Scenario: Switching tabs
- **WHEN** the user clicks a tab
- **THEN** the corresponding view is rendered with its data

---

### Requirement: Today tab — stat cards
The Today tab SHALL display three summary stat cards:
1. **Rounds** — count of completed work sessions today (local date)
2. **Focus Time** — total minutes of completed work sessions today, formatted as `Xh Ym` (hours and minutes)
3. **Completion Rate** — percentage of started work sessions that were completed today, shown as `N%`

#### Scenario: Day with sessions
- **WHEN** the user has completed work sessions today
- **THEN** all three stat cards reflect today's accurate counts

#### Scenario: No sessions today
- **WHEN** no sessions have been recorded today
- **THEN** cards show `0` / `0h 0m` / `—` (completion rate shown as dash when no sessions started)

---

### Requirement: Today tab — hourly timeline
The Today tab SHALL display an hourly breakdown of sessions across the 24-hour day as a bar chart, where bar height represents the number of completed work rounds in that hour. Hours with no sessions SHALL render as a minimal baseline (not zero-height bars that are invisible).

#### Scenario: Sessions at various hours
- **WHEN** the user has sessions at multiple hours of the day
- **THEN** bars are taller for hours with more sessions

#### Scenario: Empty day
- **WHEN** no sessions exist for today
- **THEN** all bars appear at minimal height with an empty-state label

---

### Requirement: This Week tab — daily bar chart
The This Week tab SHALL display a bar chart of completed work rounds for each of the last 7 calendar days (local date), with day labels (Mon–Sun or equivalent). The bar for today SHALL be visually distinguished (e.g., accent color or highlight).

#### Scenario: Week with mixed activity
- **WHEN** some days have sessions and others do not
- **THEN** bars reflect per-day completed round counts; empty days show zero-height bars

#### Scenario: Today's bar
- **WHEN** viewing the weekly chart
- **THEN** today's bar is visually distinct from past days

---

### Requirement: This Week tab — streak counter
The This Week tab SHALL display the **current streak**: the number of consecutive calendar days (local date) ending today or yesterday on which the user completed at least one work session. A streak SHALL remain active until midnight of the current day — if yesterday had sessions but today does not yet, the streak is still displayed as active.

#### Scenario: Active streak including yesterday
- **WHEN** the user completed sessions yesterday but not yet today
- **THEN** the streak counter shows the consecutive-day count including yesterday (streak still live)

#### Scenario: Active streak including today
- **WHEN** the user has completed at least one session today
- **THEN** today is included in the streak count

#### Scenario: Streak broken
- **WHEN** neither today nor yesterday has a completed session
- **THEN** streak counter shows 0

---

### Requirement: All Time tab — annual heatmap
The All Time tab SHALL display a heatmap grid (GitHub contribution graph style) where each cell represents one calendar day, navigable by year. Cell color intensity SHALL reflect the number of completed work rounds on that day using a fixed 4-level scale: 0 rounds (background), 1–3 (level 1), 4–7 (level 2), 8+ (level 3). Colors SHALL be derived from the active theme's `--color-focus-round` at varying opacity levels blended into `--color-background`.

#### Scenario: Day with high round count
- **WHEN** a day has 8 or more completed work rounds
- **THEN** its cell displays at maximum intensity (level 3)

#### Scenario: Day with no rounds
- **WHEN** a day has no completed work rounds
- **THEN** its cell displays at background intensity (level 0, visually empty)

#### Scenario: Theme change while stats window is open
- **WHEN** the active theme changes while the stats window is open
- **THEN** heatmap cell colors update to reflect the new theme colors

---

### Requirement: All Time tab — lifetime totals
The All Time tab SHALL display total lifetime stats: total completed work rounds, total focus time (formatted as hours), and longest streak ever recorded.

#### Scenario: All-time data display
- **WHEN** the user opens the All Time tab
- **THEN** lifetime totals are shown accurately derived from the full sessions table

#### Scenario: First launch (empty database)
- **WHEN** no sessions exist in the database
- **THEN** all totals show zero values with an appropriate empty state message

---

### Requirement: Statistics window reacts to timer events
The statistics window, when open, SHALL update its data views in real time when a timer round completes. The Today and This Week tabs SHALL refresh immediately; the All Time tab SHALL refresh only if it has been previously loaded.

#### Scenario: Round completes while stats window is open
- **WHEN** a timer round completes and the statistics window is open
- **THEN** the Today and This Week views update to reflect the newly recorded session without requiring the window to be closed and reopened
