use rusqlite::{Connection, Result};

/// Full schema for version 1. Tables use IF NOT EXISTS so the batch is
/// idempotent, but the schema_version check in `run()` prevents re-execution.
const MIGRATION_1: &str = "
    CREATE TABLE IF NOT EXISTS schema_version (
        version INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS settings (
        key   TEXT PRIMARY KEY NOT NULL,
        value TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS sessions (
        id            INTEGER PRIMARY KEY AUTOINCREMENT,
        started_at    INTEGER NOT NULL,
        ended_at      INTEGER,
        round_type    TEXT NOT NULL CHECK(round_type IN ('work', 'short-break', 'long-break')),
        duration_secs INTEGER NOT NULL CHECK(duration_secs > 0),
        completed     INTEGER NOT NULL DEFAULT 0 CHECK(completed IN (0, 1))
    );

    CREATE TABLE IF NOT EXISTS custom_themes (
        id     INTEGER PRIMARY KEY AUTOINCREMENT,
        name   TEXT NOT NULL UNIQUE,
        colors TEXT NOT NULL
    );

    CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at);
    CREATE INDEX IF NOT EXISTS idx_sessions_round_type ON sessions(round_type);

    INSERT INTO schema_version VALUES (1);
";

/// Migrates the single `theme` key to the three-field model (`theme_mode`,
/// `theme_light`, `theme_dark`). Safe to run on both existing installs
/// (propagates the old theme value) and fresh installs (falls back to
/// 'Pomotroid'). Uses INSERT OR IGNORE so a partially-applied run is harmless.
const MIGRATION_2: &str = "
    INSERT OR IGNORE INTO settings (key, value)
        VALUES ('theme_light',
                COALESCE((SELECT value FROM settings WHERE key = 'theme'), 'Pomotroid'));

    INSERT OR IGNORE INTO settings (key, value)
        VALUES ('theme_dark',
                COALESCE((SELECT value FROM settings WHERE key = 'theme'), 'Pomotroid'));

    INSERT OR IGNORE INTO settings (key, value) VALUES ('theme_mode', 'auto');

    DELETE FROM settings WHERE key = 'theme';

    INSERT INTO schema_version VALUES (2);
";

/// Adds the `language` setting (default `'auto'`) for users upgrading from
/// before localization support. Uses INSERT OR IGNORE so it is safe on fresh
/// installs where seed_defaults already wrote the row.
const MIGRATION_3: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('language', 'auto');

    INSERT INTO schema_version VALUES (3);
";

/// Adds the `verbose_logging` setting (default `'false'`) for users upgrading
/// from before diagnostic logging support.
const MIGRATION_4: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('verbose_logging', 'false');

    INSERT INTO schema_version VALUES (4);
";

/// Switches the default light theme from 'Pomotroid' to 'Pomotroid Light'.
/// Only updates users who still have the old default — custom selections are
/// left untouched.
const MIGRATION_5: &str = "
    UPDATE settings SET value = 'Pomotroid Light'
        WHERE key = 'theme_light' AND value = 'Pomotroid';

    INSERT INTO schema_version VALUES (5);
";

/// Apply any pending migrations. Each migration is wrapped in a transaction
/// so a partial failure leaves the database unchanged.
pub fn run(conn: &Connection) -> Result<()> {
    let version = current_version(conn)?;

    if version < 1 {
        conn.execute_batch(&format!("BEGIN; {MIGRATION_1} COMMIT;"))?;
    }

    if version < 2 {
        conn.execute_batch(&format!("BEGIN; {MIGRATION_2} COMMIT;"))?;
    }

    if version < 3 {
        conn.execute_batch(&format!("BEGIN; {MIGRATION_3} COMMIT;"))?;
    }

    if version < 4 {
        conn.execute_batch(&format!("BEGIN; {MIGRATION_4} COMMIT;"))?;
    }

    if version < 5 {
        conn.execute_batch(&format!("BEGIN; {MIGRATION_5} COMMIT;"))?;
    }

    Ok(())
}

/// Returns the current schema version, or 0 if the database is fresh.
fn current_version(conn: &Connection) -> Result<i64> {
    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='schema_version'",
        [],
        |row| row.get(0),
    )?;

    if !table_exists {
        return Ok(0);
    }

    conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_version",
        [],
        |row| row.get(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_is_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();
        // Second run must not error (version check prevents re-application).
        run(&conn).unwrap();
        let v: i64 = conn
            .query_row("SELECT MAX(version) FROM schema_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, 5);
    }

    #[test]
    fn all_tables_created() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();
        for table in &["settings", "sessions", "custom_themes", "schema_version"] {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "table '{table}' was not created");
        }
    }
}
