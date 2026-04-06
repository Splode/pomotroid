## ADDED Requirements

### Requirement: Sessions table stores a nullable task label per row
The `sessions` table SHALL have a `label TEXT` column (nullable, no uniqueness or length constraint at the DB level). The column SHALL be added via a versioned DB migration. All existing rows SHALL default to `NULL`. The `complete_session` query function SHALL accept an `Option<&str>` label parameter and bind it when updating the row.

#### Scenario: Migration adds column to existing database
- **WHEN** the application starts and the database schema version is below the migration that adds `label`
- **THEN** the migration executes `ALTER TABLE sessions ADD COLUMN label TEXT`, the schema version is incremented, and existing rows have `label = NULL`

#### Scenario: New session row stores label on completion
- **WHEN** `complete_session` is called with a non-null label value
- **THEN** the `sessions` row is updated with the provided label string alongside `ended_at` and `completed = 1`

#### Scenario: New session row stores NULL when no label set
- **WHEN** `complete_session` is called with `label = None`
- **THEN** the `sessions` row is updated with `label = NULL`
