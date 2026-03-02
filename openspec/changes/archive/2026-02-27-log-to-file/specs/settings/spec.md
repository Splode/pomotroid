## ADDED Requirements

### Requirement: Verbose Logging setting persisted and applied on startup
The system SHALL store a `verbose_logging` boolean setting (DB key: `verbose_logging`, default `false`) in SQLite. On startup the log level SHALL be set to DEBUG when `verbose_logging` is `true`, and INFO otherwise, before any other application setup runs.

#### Scenario: Default value is false
- **WHEN** the application runs for the first time with no existing settings
- **THEN** `verbose_logging` is `false` and the log level is INFO

#### Scenario: Verbose logging persists across restarts
- **WHEN** the user enables Verbose Logging and restarts the application
- **THEN** the log level is set to DEBUG from the first log entry onward

---

### Requirement: Verbose Logging toggled at runtime via Settings → Advanced
The system SHALL provide a "Verbose Logging" toggle in Settings → Advanced. Toggling it SHALL take effect immediately (changing the global log level via `log::set_max_level()`) without requiring an application restart, and SHALL persist the new value to the database.

#### Scenario: Enabling Verbose Logging takes immediate effect
- **WHEN** the user enables Verbose Logging
- **THEN** subsequent log entries include DEBUG-level messages without restarting

#### Scenario: Disabling Verbose Logging takes immediate effect
- **WHEN** the user disables Verbose Logging
- **THEN** DEBUG-level messages are suppressed from subsequent log entries without restarting

#### Scenario: Toggle change recorded in log
- **WHEN** Verbose Logging is enabled or disabled
- **THEN** an INFO-level log entry records the change (e.g. "Verbose logging enabled — log level set to DEBUG")
