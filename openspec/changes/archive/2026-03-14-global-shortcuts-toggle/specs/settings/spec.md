## ADDED Requirements

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
