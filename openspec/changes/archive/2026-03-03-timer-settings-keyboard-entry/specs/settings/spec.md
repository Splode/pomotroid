## ADDED Requirements

### Requirement: Timer duration DB keys store whole seconds

The system SHALL migrate timer duration storage from minute-resolution keys (`time_work_mins`, `time_short_break_mins`, `time_long_break_mins`) to second-resolution keys (`time_work_secs`, `time_short_break_secs`, `time_long_break_secs`). MIGRATION_2 SHALL convert existing minute values to seconds, insert them under the new keys, and delete the old keys. The `load()` function SHALL read from the new keys without a `* 60` conversion. Default values seeded on first launch SHALL be expressed in seconds.

#### Scenario: Existing installation is migrated non-destructively

- **WHEN** a user with `time_work_mins = "30"` upgrades to the new version
- **THEN** MIGRATION_2 SHALL insert `time_work_secs = "1800"` and delete `time_work_mins`, preserving the user's 30-minute preference

#### Scenario: Default seed uses second-resolution keys

- **WHEN** the application runs on a fresh install
- **THEN** the settings table SHALL contain `time_work_secs = "1500"`, `time_short_break_secs = "300"`, `time_long_break_secs = "900"` and SHALL NOT contain any `*_mins` keys

#### Scenario: Sub-minute value persists across restarts

- **WHEN** the user sets the Focus duration to 5:39 (339 seconds) and restarts the app
- **THEN** `time_work_secs` SHALL equal `"339"` in the database and the Settings struct SHALL expose `time_work_secs = 339`

#### Scenario: Reset to defaults restores second-resolution defaults

- **WHEN** the user triggers Reset All Settings
- **THEN** the settings table SHALL contain `time_work_secs = "1500"` and SHALL NOT contain `time_work_mins`

#### Scenario: Old `*_mins` keys are absent after migration

- **WHEN** MIGRATION_2 has run on a database that previously had `*_mins` keys
- **THEN** `SELECT key FROM settings WHERE key LIKE '%_mins'` SHALL return zero rows
