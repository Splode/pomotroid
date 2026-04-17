## ADDED Requirements

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

### Requirement: `check_for_updates` toggle in Settings → System

The system SHALL provide a "Check for Updates Automatically" toggle in Settings → System. Toggling it SHALL persist the new value immediately via `settings_set` and take effect on the next settings window open (no restart required).

#### Scenario: Toggle visible in System section

- **WHEN** the user navigates to Settings → System
- **THEN** a "Check for Updates Automatically" toggle SHALL be visible

#### Scenario: Disabling prevents update check

- **WHEN** the user disables the toggle and reopens Settings
- **THEN** `check_update` SHALL NOT be called on About section mount
