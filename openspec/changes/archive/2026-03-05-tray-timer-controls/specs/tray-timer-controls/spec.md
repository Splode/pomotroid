## ADDED Requirements

### Requirement: Tray menu exposes timer control actions

The tray context menu SHALL include three timer control items — Toggle (Start/Pause/Resume), Skip, and Reset Round — positioned above the existing Show and Exit items.

#### Scenario: Menu items are present when tray is created

- **WHEN** the tray icon is created
- **THEN** the menu SHALL contain items with ids "toggle", "skip", and "reset-round"

#### Scenario: Timer control items appear above Show and Exit

- **WHEN** the user opens the tray context menu
- **THEN** the toggle, skip, and reset-round items SHALL appear before the separator, Show, and Exit items

### Requirement: Toggle item label reflects current timer state

The toggle item SHALL display "Start" when the timer is idle, "Pause" when it is running, and "Resume" when it is paused.

#### Scenario: Label is "Start" on initial tray creation

- **WHEN** the tray icon is first created and the timer is idle
- **THEN** the toggle item label SHALL be "Start"

#### Scenario: Label changes to "Pause" when timer starts

- **WHEN** the timer transitions from idle or paused to running
- **THEN** the toggle item label SHALL update to "Pause"

#### Scenario: Label changes to "Resume" when timer is paused

- **WHEN** the timer is paused
- **THEN** the toggle item label SHALL update to "Resume"

#### Scenario: Label resets to "Start" when timer is reset

- **WHEN** the timer is reset to idle
- **THEN** the toggle item label SHALL update to "Start"

### Requirement: Skip and Reset Round are disabled when the timer is idle

Skip and Reset Round SHALL be enabled only when the timer is running or paused.

#### Scenario: Skip and Reset Round disabled on initial tray creation

- **WHEN** the tray icon is first created and the timer is idle
- **THEN** the skip and reset-round items SHALL be disabled (non-interactive)

#### Scenario: Skip and Reset Round enabled when timer is running

- **WHEN** the timer transitions to running
- **THEN** the skip and reset-round items SHALL become enabled

#### Scenario: Skip and Reset Round enabled when timer is paused

- **WHEN** the timer is paused
- **THEN** the skip and reset-round items SHALL remain enabled

#### Scenario: Skip and Reset Round disabled after reset

- **WHEN** the timer is reset to idle
- **THEN** the skip and reset-round items SHALL become disabled

### Requirement: Toggle item dispatches the correct timer command

Clicking the toggle item SHALL call `timer.toggle()`, which starts the timer if idle, pauses it if running, or resumes it if paused.

#### Scenario: Toggle starts an idle timer

- **WHEN** the timer is idle and the user clicks the toggle item
- **THEN** the timer SHALL start running

#### Scenario: Toggle pauses a running timer

- **WHEN** the timer is running and the user clicks the toggle item
- **THEN** the timer SHALL pause

#### Scenario: Toggle resumes a paused timer

- **WHEN** the timer is paused and the user clicks the toggle item
- **THEN** the timer SHALL resume running

### Requirement: Skip item advances to the next round

Clicking the skip item SHALL call `timer.skip()`, immediately completing the current round and advancing the sequence.

#### Scenario: Skip advances the round

- **WHEN** the timer is running or paused and the user clicks the skip item
- **THEN** the current round SHALL complete and the sequence SHALL advance to the next round

### Requirement: Reset Round item resets the current round only

Clicking the reset-round item SHALL call `timer.restart_round()`, zeroing elapsed time for the current round without changing the round type or position in the work/break cycle.

#### Scenario: Reset Round zeroes elapsed time

- **WHEN** the timer is running or paused and the user clicks the reset-round item
- **THEN** the current round's elapsed time SHALL be zeroed and the timer SHALL return to idle for the same round

#### Scenario: Reset Round preserves sequence position

- **WHEN** the user clicks the reset-round item during round 2 of a work/break cycle
- **THEN** the round type and work round number SHALL remain unchanged after the reset
