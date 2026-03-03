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

---

### Requirement: Language setting
The system SHALL store a `language` setting in the settings database with a default value of `'auto'`. The value SHALL be either `'auto'` or a supported IETF BCP 47 locale tag (`'en'`, `'es'`, `'fr'`, `'de'`, `'ja'`). The `Settings` struct SHALL expose a `language: String` field, and `types.ts` SHALL mirror this field.

#### Scenario: Default language is auto
- **WHEN** a new user launches the app for the first time
- **THEN** the `language` setting SHALL be `'auto'`

#### Scenario: Language setting persists across restarts
- **WHEN** the user sets `language` to `'de'` and restarts the app
- **THEN** the `language` setting SHALL be `'de'` after restart

#### Scenario: Language setting migration for existing users
- **WHEN** an existing user upgrades from a version without the `language` setting
- **THEN** MIGRATION_3 SHALL insert `language = 'auto'` as a default row

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

---

### Requirement: Reset to factory defaults via Settings → About
The system SHALL provide a "Reset All Settings" action in Settings → About. The action SHALL use an inline two-step confirmation: the first interaction reveals a confirmation prompt with Cancel and Reset controls; only the second (Reset) interaction fires `settings_reset_defaults`. Cancelling at any point SHALL restore the original button without performing a reset. The action resets all settings globally (all 26 keys) to factory defaults, including clearing any custom alert sounds from disk and from the audio engine's in-memory state.

#### Scenario: Reset button is visible in About section
- **WHEN** the user navigates to Settings → About
- **THEN** a "Reset All Settings" row SHALL be visible below the navigation links

#### Scenario: First click enters confirmation state
- **WHEN** the user clicks "Reset All Settings"
- **THEN** the row SHALL replace the button with a confirmation prompt and Cancel / Reset buttons

#### Scenario: Cancel dismisses confirmation without resetting
- **WHEN** the user clicks Cancel in the confirmation state
- **THEN** the row SHALL return to the initial "Reset All Settings" button and no settings SHALL be changed

#### Scenario: Confirm fires global reset
- **WHEN** the user clicks Reset in the confirmation state
- **THEN** `settings_reset_defaults` SHALL be invoked, all settings SHALL revert to factory defaults, and the row SHALL return to the initial button

#### Scenario: Custom alert sounds are cleared on reset
- **WHEN** the user confirms a full settings reset
- **THEN** any custom alert sound files SHALL be deleted from disk and the audio engine SHALL revert to built-in sounds immediately, without requiring a restart

#### Scenario: Reset no longer available in Timer section
- **WHEN** the user navigates to Settings → Timer
- **THEN** no reset button SHALL be present in that section

---

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
