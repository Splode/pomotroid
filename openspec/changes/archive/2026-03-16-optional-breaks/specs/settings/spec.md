## ADDED Requirements

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
