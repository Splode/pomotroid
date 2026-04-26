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

### Requirement: Reset All Settings action is located in Settings → System

The system SHALL provide a "Reset All Settings" action in Settings → System. The action SHALL use an inline two-step confirmation: the first interaction reveals a confirmation prompt with Cancel and Reset controls; only the second (Reset) interaction fires `settings_reset_defaults`. Cancelling at any point SHALL restore the original button without performing a reset. The action resets all settings globally (all 26 keys) to factory defaults, including clearing any custom alert sounds from disk and from the audio engine's in-memory state.

#### Scenario: Cancel dismisses confirmation without resetting

- **WHEN** the user clicks Reset All Settings and then clicks Cancel
- **THEN** no settings SHALL be changed and the row SHALL return to its initial button state

#### Scenario: Confirm fires global reset

- **WHEN** the user clicks Reset All Settings and then clicks Confirm
- **THEN** `settings_reset_defaults` SHALL be invoked, all settings SHALL revert to factory defaults, and the row SHALL return to the initial button state

#### Scenario: Custom alert sounds are cleared on reset

- **WHEN** the user confirms a full settings reset
- **THEN** any custom alert sound files SHALL be deleted from disk and the audio engine's in-memory custom paths SHALL be cleared

#### Scenario: About section has no reset button

- **WHEN** the user navigates to Settings → About
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

---

### Requirement: `check_for_updates` setting persisted with default true

The system SHALL store a `check_for_updates` boolean setting (DB key: `check_for_updates`, default `'true'`) in SQLite. A new DB migration SHALL insert this default row for both fresh installs and existing users upgrading. The `Settings` struct SHALL expose a `check_for_updates: bool` field and `types.ts` SHALL mirror this field.

#### Scenario: Default value is true on fresh install

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `check_for_updates` SHALL be `true`

#### Scenario: Setting persists across restarts

- **WHEN** the user disables automatic update checks and restarts the application
- **THEN** `check_for_updates` SHALL be `false` after restart

#### Scenario: Migration inserts default for existing users

- **WHEN** an existing user upgrades from a version without `check_for_updates`
- **THEN** the migration SHALL insert `check_for_updates = 'true'` without modifying other settings

---

### Requirement: `check_for_updates` toggle in Settings → System

The system SHALL provide a "Check for Updates Automatically" toggle in Settings → System. Toggling it SHALL persist the new value immediately via `settings_set` and take effect on the next settings window open (no restart required).

#### Scenario: Toggle visible in System section

- **WHEN** the user navigates to Settings → System
- **THEN** a "Check for Updates Automatically" toggle SHALL be visible

#### Scenario: Disabling prevents update check

- **WHEN** the user disables the toggle and reopens Settings
- **THEN** `check_update` SHALL NOT be called on About section mount

---

### Requirement: global_shortcuts_enabled setting

The system SHALL store a `global_shortcuts_enabled` boolean setting (DB key: `global_shortcuts_enabled`, default `'false'`) in SQLite. The `Settings` struct SHALL expose a `global_shortcuts_enabled: bool` field, and `types.ts` SHALL mirror this as `global_shortcuts_enabled: boolean`.

#### Scenario: Default value is false

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `global_shortcuts_enabled` SHALL be `false`

#### Scenario: Setting persists across restarts

- **WHEN** the user enables global shortcuts and restarts the application
- **THEN** `global_shortcuts_enabled` SHALL be `true` after restart

#### Scenario: Migration adds setting for existing users

- **WHEN** an existing user upgrades from a version without the `global_shortcuts_enabled` setting
- **THEN** the setting SHALL be inserted with value `'false'` via `INSERT OR IGNORE`
- **AND** the user's other settings SHALL be unchanged

---

### Requirement: short_breaks_enabled setting

The system SHALL store a `short_breaks_enabled` boolean setting (DB key: `short_breaks_enabled`, default `'true'`) in SQLite. The `Settings` struct SHALL expose a `short_breaks_enabled: bool` field, and `types.ts` SHALL mirror this as `short_breaks_enabled: boolean`.

#### Scenario: Default value is true

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `short_breaks_enabled` SHALL be `true`

#### Scenario: Setting persists across restarts

- **WHEN** the user disables short breaks and restarts the application
- **THEN** `short_breaks_enabled` SHALL be `false` after restart

#### Scenario: Migration seeds setting for existing users

- **WHEN** an existing user upgrades from a version without `short_breaks_enabled`
- **THEN** the setting SHALL be inserted with value `'true'` via `INSERT OR IGNORE`
- **AND** the user's other settings SHALL be unchanged

---

### Requirement: long_breaks_enabled setting

The system SHALL store a `long_breaks_enabled` boolean setting (DB key: `long_breaks_enabled`, default `'true'`) in SQLite. The `Settings` struct SHALL expose a `long_breaks_enabled: bool` field, and `types.ts` SHALL mirror this as `long_breaks_enabled: boolean`.

#### Scenario: Default value is true

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `long_breaks_enabled` SHALL be `true`

#### Scenario: Setting persists across restarts

- **WHEN** the user disables long breaks and restarts the application
- **THEN** `long_breaks_enabled` SHALL be `false` after restart

#### Scenario: Migration seeds setting for existing users

- **WHEN** an existing user upgrades from a version without `long_breaks_enabled`
- **THEN** the setting SHALL be inserted with value `'true'` via `INSERT OR IGNORE`
- **AND** the user's other settings SHALL be unchanged

---

### Requirement: window_x and window_y settings

The system SHALL store optional integer settings `window_x` and `window_y` (physical pixel coordinates) in the settings database. These keys are absent on a fresh install and are only written after the first time the user moves the window. Absent keys SHALL be treated as "no saved position."

#### Scenario: Keys absent on first run

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `window_x` and `window_y` SHALL NOT be present in the settings database

#### Scenario: Keys written after window move

- **WHEN** the user moves the main window
- **THEN** `window_x` and `window_y` SHALL be present in the settings database with the new coordinates

---

### Requirement: window_width and window_height settings

The system SHALL store optional unsigned integer settings `window_width` and `window_height` (physical pixels) in the settings database. These keys are absent on a fresh install and are written alongside `window_x`/`window_y` after the first move or resize.

#### Scenario: Keys absent on first run

- **WHEN** the application runs for the first time with no existing settings
- **THEN** `window_width` and `window_height` SHALL NOT be present in the settings database

#### Scenario: Keys written after window resize

- **WHEN** the user resizes the main window
- **THEN** `window_width` and `window_height` SHALL be present in the settings database with the new dimensions

---

### Requirement: Window position and size cleared by Reset All Settings

When the user triggers Reset All Settings, the four window geometry keys (`window_x`, `window_y`, `window_width`, `window_height`) SHALL be removed from the database along with all other settings. The reset takes effect on the next launch — the current session's window position is not disturbed. On the following launch the window opens at the OS default position and size.

#### Scenario: Reset All Settings removes geometry keys

- **WHEN** the user triggers Reset All Settings
- **THEN** `window_x`, `window_y`, `window_width`, and `window_height` SHALL NOT be present in the settings database

#### Scenario: Window opens at OS default after reset

- **WHEN** the application starts after a Reset All Settings has been performed
- **THEN** the main window SHALL open at the OS default position and size
