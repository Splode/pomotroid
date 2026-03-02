## ADDED Requirements

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
