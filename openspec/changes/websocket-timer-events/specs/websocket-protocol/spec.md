## ADDED Requirements

### Requirement: Server broadcasts paused event when timer pauses
When the timer transitions to a paused state, the server SHALL broadcast a `paused` message to all connected WebSocket clients carrying the elapsed time in seconds.

#### Scenario: Client receives paused message on pause
- **WHEN** the user pauses a running timer
- **THEN** all connected WebSocket clients SHALL receive `{ "type": "paused", "payload": { "elapsed_secs": <n> } }` where `n` is the number of seconds elapsed in the current round

#### Scenario: No clients connected on pause
- **WHEN** the timer is paused and no WebSocket clients are connected
- **THEN** the broadcast SHALL be silently discarded with no error

### Requirement: Server broadcasts resumed event when timer resumes
When the timer transitions from paused to running, the server SHALL broadcast a `resumed` message to all connected WebSocket clients carrying the elapsed time in seconds.

#### Scenario: Client receives resumed message on resume
- **WHEN** the user resumes a paused timer
- **THEN** all connected WebSocket clients SHALL receive `{ "type": "resumed", "payload": { "elapsed_secs": <n> } }` where `n` is the number of seconds elapsed in the current round at the moment of resume

#### Scenario: No clients connected on resume
- **WHEN** the timer is resumed and no WebSocket clients are connected
- **THEN** the broadcast SHALL be silently discarded with no error

### Requirement: Server broadcasts reset event when timer resets
When the timer resets to idle, the server SHALL broadcast a `reset` message to all connected WebSocket clients with no payload.

#### Scenario: Client receives reset message on reset
- **WHEN** the timer is reset (via stop command or settings change)
- **THEN** all connected WebSocket clients SHALL receive `{ "type": "reset" }`

#### Scenario: Client requests state after reset
- **WHEN** a client receives a `reset` message and sends `{ "type": "getState" }`
- **THEN** the server SHALL respond with a `state` message reflecting the idle timer snapshot

### Requirement: New broadcast message types are additive and non-breaking
The `paused`, `resumed`, and `reset` message types SHALL be added to the existing protocol without altering the `roundChange` or `state` message formats. Clients that do not handle the new types SHALL be unaffected.

#### Scenario: Existing roundChange is unaffected
- **WHEN** a round completes and a new round begins
- **THEN** connected clients SHALL still receive `{ "type": "roundChange", "payload": <TimerSnapshot> }` in the same format as before this change
