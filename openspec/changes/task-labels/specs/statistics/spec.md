## ADDED Requirements

### Requirement: Today tab — label breakdown pie chart
The Today tab SHALL display a label breakdown section below the hourly timeline. The section SHALL show a pie (or donut) chart visualising completed work session time by label for the current calendar day. Each distinct label SHALL be a separate slice; sessions with `label = NULL` SHALL be grouped into an "(unlabeled)" slice rendered last in a muted color. If more than 5 distinct non-null labels exist, the smallest slices beyond the top 4 SHALL be grouped into an "Other" slice. The section SHALL be hidden if no completed work sessions exist for today.

#### Scenario: Today has labeled sessions
- **WHEN** the user has completed work sessions today with one or more distinct labels
- **THEN** the pie chart renders with one slice per label (plus an unlabeled slice if applicable), with each slice labeled by name and duration

#### Scenario: Today has only unlabeled sessions
- **WHEN** all completed work sessions today have `label = NULL`
- **THEN** the chart renders a single full-circle slice labeled "(unlabeled)"

#### Scenario: Today has more than 5 distinct labels
- **WHEN** more than 5 distinct non-null labels exist for today's sessions
- **THEN** the top 4 labels by duration are shown as individual slices and the remainder are grouped into an "Other" slice

#### Scenario: No sessions today
- **WHEN** no completed work sessions exist for today
- **THEN** the label breakdown section is not rendered

### Requirement: This Week tab — label breakdown ranked list
The This Week tab SHALL display a label breakdown section showing total completed work session time by label for the current 7-day period. The section SHALL render as a ranked list sorted by total duration descending, with each row showing the label name and a proportional horizontal bar representing its share of total labeled time. Sessions with `label = NULL` SHALL appear as "(unlabeled)" at the bottom of the list. The section SHALL be hidden if no completed work sessions exist for the week.

#### Scenario: Week has labeled sessions
- **WHEN** the user has completed work sessions this week with at least one distinct label
- **THEN** the ranked list renders with labels sorted by descending total duration; each row shows the label name, duration, and a proportional bar

#### Scenario: Unlabeled sessions present
- **WHEN** some sessions this week have `label = NULL`
- **THEN** an "(unlabeled)" row appears at the bottom of the list

#### Scenario: No sessions this week
- **WHEN** no completed work sessions exist for the current 7-day period
- **THEN** the label breakdown section is not rendered

### Requirement: All Time tab — label breakdown ranked list
The All Time tab SHALL display a label breakdown section showing total completed work session time by label across all recorded history. The layout and rendering rules SHALL match the This Week label breakdown (ranked list, proportional bars, unlabeled at bottom). The section SHALL be hidden if no completed work sessions exist in the database.

#### Scenario: All-time data with multiple labels
- **WHEN** the user has completed sessions with multiple distinct labels across all time
- **THEN** the ranked list renders with all labels sorted by descending total duration

#### Scenario: Empty database
- **WHEN** no completed work sessions exist
- **THEN** the label breakdown section is not rendered

### Requirement: Label breakdown data loaded per tab
The statistics window SHALL request label breakdown data separately from existing stats data using a dedicated `stats_get_label_breakdown(period)` IPC command. The command SHALL accept a period string of `"today"`, `"week"`, or `"alltime"` and return `Vec<LabelStat>` where each entry has `label: Option<String>` and `duration_mins: u32`. The frontend SHALL call this command when switching to each tab and on `timer:round-change` if the tab is currently active.

#### Scenario: Label breakdown loaded on tab switch
- **WHEN** the user switches to the Today, This Week, or All Time tab
- **THEN** `statsGetLabelBreakdown` is called with the appropriate period and the breakdown section renders with fresh data

#### Scenario: Breakdown refreshes on round complete
- **WHEN** a timer round completes while the statistics window is open and showing a tab
- **THEN** the label breakdown for the active tab is refreshed alongside the existing stats refresh
