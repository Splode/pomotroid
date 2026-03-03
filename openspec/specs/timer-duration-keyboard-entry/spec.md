## ADDED Requirements

### Requirement: Timer duration badge is an editable MM:SS input
The Settings → Timer section SHALL replace the static time display badge for Focus, Short Break, and Long Break rows with an editable text input. The input SHALL display the current duration in `MM:SS` format when not focused and SHALL accept user input in `MM:SS` or bare-integer-minutes format. The valid range SHALL be 1:00 (60 seconds) minimum to 90:00 (5400 seconds) maximum.

#### Scenario: Badge displays current value in MM:SS format
- **WHEN** the settings window opens
- **THEN** each timer badge SHALL display the stored duration as `MM:SS` (e.g. 25:00 for 1500 s, 5:39 for 339 s)

#### Scenario: Clicking the badge activates edit mode
- **WHEN** the user clicks the timer badge
- **THEN** the input SHALL receive focus and its text SHALL be selected

#### Scenario: Entering a valid MM:SS value and pressing Enter saves it
- **WHEN** the user types `5:39` and presses Enter
- **THEN** the duration SHALL be saved as 339 seconds and the badge SHALL display `5:39`

#### Scenario: Entering a bare integer is interpreted as whole minutes
- **WHEN** the user types `25` and commits
- **THEN** the duration SHALL be saved as 1500 seconds and the badge SHALL display `25:00`

#### Scenario: Value below minimum is clamped to 1:00
- **WHEN** the user enters a value that resolves to fewer than 60 seconds (e.g. `0:30`)
- **THEN** the duration SHALL be clamped to 60 seconds and the badge SHALL display `1:00`

#### Scenario: Value above maximum is clamped to 90:00
- **WHEN** the user enters a value that resolves to more than 5400 seconds (e.g. `91:00`)
- **THEN** the duration SHALL be clamped to 5400 seconds and the badge SHALL display `90:00`

#### Scenario: Invalid input reverts to previous value
- **WHEN** the user enters text that cannot be parsed (e.g. `abc`, `::`)
- **THEN** the duration SHALL remain unchanged and the badge SHALL display the previous value

#### Scenario: Blurring the field commits the value
- **WHEN** the user edits the badge and clicks elsewhere without pressing Enter
- **THEN** the value SHALL be committed (parsed, clamped, saved) identically to pressing Enter

#### Scenario: Pressing Tab commits the value and moves focus
- **WHEN** the user edits the badge and presses Tab
- **THEN** the value SHALL be committed and focus SHALL move to the next focusable element

### Requirement: Slider and editable badge remain synchronised
The timer duration slider SHALL continue to operate at 1-minute granularity. Moving the slider SHALL update the badge display and save the new whole-minute value in seconds. Editing the badge SHALL update the slider thumb to the nearest whole minute.

#### Scenario: Moving the slider updates the badge
- **WHEN** the user drags the Focus slider to 30
- **THEN** the badge SHALL display `30:00` and 1800 seconds SHALL be saved

#### Scenario: Entering a sub-minute value updates the slider to nearest minute
- **WHEN** the user enters `5:39` in the Focus badge
- **THEN** the slider thumb SHALL move to position 6 (nearest whole minute) and 339 seconds SHALL be saved

#### Scenario: Whole-minute badge entry leaves slider aligned
- **WHEN** the user enters `25:00` in the Focus badge
- **THEN** the slider thumb SHALL sit at position 25 and 1500 seconds SHALL be saved

### Requirement: Timer dial reflects new duration immediately when idle
When a timer duration setting is changed while the timer is idle (not running, no elapsed progress), the main window's timer dial SHALL update to show the new total duration without requiring the user to manually reset the timer. This is achieved by emitting a `timer:reset` event with the updated snapshot after `apply_settings` is called.

#### Scenario: Changing Focus duration while idle updates the dial immediately
- **WHEN** the timer is idle and the user changes the Focus duration from 25:00 to 5:39 in Settings
- **THEN** the main window dial SHALL update to display a full 5:39 ring without any user action on the main window

#### Scenario: Changing duration while timer is running takes effect next round
- **WHEN** the timer is actively counting down and the user changes the Focus duration
- **THEN** the current countdown SHALL complete unchanged and the new duration SHALL apply from the next round onward

### Requirement: Statistics focus time rounds to nearest minute
The daily focus time reported in the Statistics view SHALL be rounded to the nearest minute (not truncated). A session of 5:39 (339 s) SHALL count as 6 minutes of focus time, not 5.

#### Scenario: Sub-minute remainder rounds up at 30 seconds
- **WHEN** completed work sessions total 339 seconds (5:39) for the day
- **THEN** the Statistics view SHALL display `6m` as the focus time

#### Scenario: Sub-minute remainder rounds down below 30 seconds
- **WHEN** completed work sessions total 324 seconds (5:24) for the day
- **THEN** the Statistics view SHALL display `5m` as the focus time

#### Scenario: Exact minute boundary is unchanged
- **WHEN** completed work sessions total exactly 1500 seconds (25:00) for the day
- **THEN** the Statistics view SHALL display `25m` as the focus time
