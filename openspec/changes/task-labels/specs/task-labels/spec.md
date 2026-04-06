## ADDED Requirements

### Requirement: Sticky task label input on the timer UI
The system SHALL display a text input below the round type label ("WORK #1") in the timer window during work rounds. The input SHALL be hidden during break rounds (short break, long break) and hidden when the window is in compact mode (`width < 300 || height < 300`). The input SHALL have no visible border in its default (unfocused) state; a subtle border or outline SHALL appear on focus. The input SHALL have a placeholder text indicating optional entry (e.g., "what are you working on?"). The maximum character length SHALL be 48 characters.

#### Scenario: Input visible during work round in normal mode
- **WHEN** the current round type is "work" and the window is not in compact mode
- **THEN** the task label input is visible below the round type label

#### Scenario: Input hidden during break rounds
- **WHEN** the current round type is "short-break" or "long-break"
- **THEN** the task label input is not rendered

#### Scenario: Input hidden in compact mode
- **WHEN** the window width is less than 300px or the window height is less than 300px
- **THEN** the task label input is not rendered

#### Scenario: Input has no border when unfocused
- **WHEN** the task label input is visible but not focused
- **THEN** no border or outline is rendered around the input

#### Scenario: Input shows border on focus
- **WHEN** the user clicks or tabs into the task label input
- **THEN** a subtle border or outline appears to indicate focus

### Requirement: Label value is sticky across round transitions
The task label value SHALL persist in application state across round completions, round skips, and break rounds. The label SHALL remain set to the last user-entered value until explicitly cleared or changed by the user.

#### Scenario: Label persists after work round completes
- **WHEN** a work round completes naturally with an active label
- **THEN** the label input on the next work round is pre-filled with the same value

#### Scenario: Label persists through break rounds
- **WHEN** a break round begins after a labeled work round
- **THEN** the label value is retained in state and reappears when the next work round starts

#### Scenario: Label persists after skipping a round
- **WHEN** the user skips a round (work or break) with an active label
- **THEN** the label value is unchanged

### Requirement: Label is cleared on explicit timer reset or settings defaults reset
The system SHALL emit a `label:clear` Tauri event when the `timer_reset` command executes and when the `settings_reset_defaults` command executes. The frontend SHALL listen for `label:clear` and clear the task label input to empty. Settings changes (timer durations, round counts) SHALL NOT emit `label:clear`.

#### Scenario: Reset button clears label
- **WHEN** the user activates the timer reset (via footer button or keyboard shortcut)
- **THEN** the `label:clear` event is emitted and the task label input is cleared

#### Scenario: Reset all settings clears label
- **WHEN** the user confirms "Reset All Settings" in Settings → System
- **THEN** the `label:clear` event is emitted and the task label input is cleared

#### Scenario: Changing timer duration does not clear label
- **WHEN** the user changes a timer duration or round count in Settings → Timer
- **THEN** no `label:clear` event is emitted and the task label input value is unchanged

### Requirement: Active label is stored with each completed session
The system SHALL store the task label value (or NULL if no label is set) on the `sessions` DB row when a session completes. The label SHALL be read from `TimerController.current_label` at completion time, capturing any mid-round label changes. Skipped sessions (completed = 0) SHALL also have the label written to the DB row, but these sessions are excluded from stats by existing query filters.

#### Scenario: Session completes with active label
- **WHEN** a work round completes naturally and the task label is "fix login bug"
- **THEN** the `sessions` row for that round has `label = 'fix login bug'` and `completed = 1`

#### Scenario: Session completes with no label
- **WHEN** a work round completes naturally and no task label is set
- **THEN** the `sessions` row for that round has `label = NULL` and `completed = 1`

#### Scenario: Skipped session records label
- **WHEN** a work round is skipped after elapsed ≥ 1 second with an active label
- **THEN** the `sessions` row has the label written and `completed = 0`

#### Scenario: Instant skip records no session
- **WHEN** a work round is skipped at elapsed = 0 (before the first tick)
- **THEN** no `sessions` row is created and no label is stored

### Requirement: Timer set label IPC command
The system SHALL expose a `timer_set_label` Tauri command that accepts an optional string (`label: Option<String>`) and stores it in `TimerController.current_label`. An empty string passed from the frontend SHALL be normalized to `None`. The frontend IPC module SHALL expose a typed `timerSetLabel(label: string | null)` wrapper. The frontend SHALL call this wrapper on label input change, debounced by approximately 300–400ms.

#### Scenario: Label updated via IPC
- **WHEN** the user types in the task label input and the debounce period elapses
- **THEN** `timerSetLabel` is called and `TimerController.current_label` is updated to the new value

#### Scenario: Empty string normalised to null
- **WHEN** the user clears the task label input
- **THEN** `timerSetLabel` is called with an empty string, which the Rust command stores as `None`
