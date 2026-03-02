### Requirement: Mini controls displayed in compact mode
When the timer window is in compact mode (either dimension < 300px), the system SHALL display a slim row of three icon-only buttons below the timer dial: restart current round, play/pause, and skip round. These controls SHALL be rendered at a fixed size independent of the dial's zoom scale.

#### Scenario: Controls appear in compact mode
- **WHEN** the window width or height drops below 300px
- **THEN** the restart, play/pause, and skip buttons SHALL be visible below the dial

#### Scenario: Controls absent in normal mode
- **WHEN** the window is at or above the compact threshold in both dimensions
- **THEN** the mini controls SHALL NOT be rendered (the full controls row is shown instead)

#### Scenario: Play/pause icon reflects running state
- **WHEN** the timer is running
- **THEN** the play/pause button SHALL show a pause icon
- **WHEN** the timer is paused or idle
- **THEN** the play/pause button SHALL show a play icon

#### Scenario: Restart button restarts current round
- **WHEN** the user clicks the restart button in compact mode
- **THEN** the current round's elapsed time SHALL reset to zero without advancing the sequence

#### Scenario: Skip button advances to next round
- **WHEN** the user clicks the skip button in compact mode
- **THEN** the timer SHALL advance to the next round in the sequence

#### Scenario: Controls scale independently of dial
- **WHEN** the dial zoom scale is at its minimum (0.4)
- **THEN** the mini control buttons SHALL still render at their full defined size (24×24px)
