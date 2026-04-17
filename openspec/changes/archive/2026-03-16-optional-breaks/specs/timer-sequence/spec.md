## ADDED Requirements

### Requirement: Short breaks can be independently disabled

The system SHALL support a `short_breaks_enabled` setting (default `true`). When `false`, the sequence SHALL skip ShortBreak rounds entirely: a Work round that would normally advance to ShortBreak SHALL instead advance directly to the next Work round, incrementing `work_round_number` by one. All other sequence behaviour (long breaks, counter reset at the long-break point) SHALL be unaffected.

#### Scenario: Short breaks disabled — work rounds chain directly

- **WHEN** `short_breaks_enabled` is `false`
- **AND** a Work round completes before the long-break point
- **THEN** the next round SHALL be Work with `work_round_number` incremented by one
- **AND** no ShortBreak round SHALL occur

#### Scenario: Short breaks disabled — long breaks still fire

- **WHEN** `short_breaks_enabled` is `false`
- **AND** a Work round completes at the long-break point (`work_round_number >= work_rounds_total`)
- **AND** `long_breaks_enabled` is `true`
- **THEN** the next round SHALL be LongBreak

#### Scenario: Short breaks re-enabled — cycle resumes normally

- **WHEN** `short_breaks_enabled` is changed to `true`
- **THEN** the next Work-to-break transition SHALL produce a ShortBreak or LongBreak as determined by the current round position

### Requirement: Long breaks can be independently disabled

The system SHALL support a `long_breaks_enabled` setting (default `true`). When `false`, the sequence SHALL never advance to a LongBreak round. At the long-break point, the sequence SHALL advance to ShortBreak instead (if `short_breaks_enabled` is `true`) or directly to Work(1) (if `short_breaks_enabled` is also `false`). In both cases `work_round_number` SHALL reset to 1 at that boundary, preserving the cycle structure.

#### Scenario: Long breaks disabled — short break substituted at long-break point

- **WHEN** `long_breaks_enabled` is `false`
- **AND** `short_breaks_enabled` is `true`
- **AND** a Work round completes at the long-break point
- **THEN** the next round SHALL be ShortBreak
- **AND** `work_round_number` SHALL reset to 1 after that ShortBreak completes

#### Scenario: Long breaks disabled — cycle resets when both breaks disabled

- **WHEN** `long_breaks_enabled` is `false`
- **AND** `short_breaks_enabled` is `false`
- **AND** a Work round completes at the long-break point
- **THEN** the next round SHALL be Work with `work_round_number` reset to 1

#### Scenario: Long breaks disabled — short breaks still fire before the long-break point

- **WHEN** `long_breaks_enabled` is `false`
- **AND** `short_breaks_enabled` is `true`
- **AND** a Work round completes before the long-break point
- **THEN** the next round SHALL be ShortBreak as normal

#### Scenario: Both breaks disabled — pure work loop

- **WHEN** `short_breaks_enabled` is `false`
- **AND** `long_breaks_enabled` is `false`
- **THEN** the sequence SHALL consist entirely of Work rounds
- **AND** `work_round_number` SHALL increment each round and reset to 1 at `work_rounds_total`

### Requirement: Session work count

`SequenceState` SHALL expose a `session_work_count: u32` field that starts at 1 and increments by 1 each time `advance()` enters a Work round. Unlike `work_round_number`, it SHALL never reset at cycle boundaries — only a call to `reset()` returns it to 1. It is included in `TimerSnapshot` and surfaced to the frontend as a session counter.

#### Scenario: session_work_count increments across cycle boundaries

- **WHEN** `long_breaks_enabled` is `false`
- **AND** multiple work rounds complete across what would have been a long-break boundary
- **THEN** `session_work_count` SHALL continue incrementing without resetting

#### Scenario: session_work_count resets on timer reset

- **WHEN** the user triggers a timer reset
- **THEN** `session_work_count` SHALL be reset to 1

#### Scenario: Round counter display adapts to long_breaks_enabled

- **WHEN** `long_breaks_enabled` is `true`
- **THEN** the round counter SHALL display `work_round_number / work_rounds_total`
- **WHEN** `long_breaks_enabled` is `false`
- **THEN** the round counter SHALL display a localised "round N" label using `session_work_count`
