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

/// Migrates timer duration storage from minute-resolution keys to second-resolution keys.
/// Reads existing `time_*_mins` rows, multiplies by 60, writes `time_*_secs`, then deletes
/// the old keys so key names align with the Settings struct field names.
const MIGRATION_2: &str = "
    INSERT OR IGNORE INTO settings (key, value)
        SELECT 'time_work_secs', CAST(CAST(value AS INTEGER) * 60 AS TEXT)
          FROM settings WHERE key = 'time_work_mins';
    INSERT OR IGNORE INTO settings (key, value)
        SELECT 'time_short_break_secs', CAST(CAST(value AS INTEGER) * 60 AS TEXT)
          FROM settings WHERE key = 'time_short_break_mins';
    INSERT OR IGNORE INTO settings (key, value)
        SELECT 'time_long_break_secs', CAST(CAST(value AS INTEGER) * 60 AS TEXT)
          FROM settings WHERE key = 'time_long_break_mins';
    DELETE FROM settings WHERE key IN
        ('time_work_mins', 'time_short_break_mins', 'time_long_break_mins');
    INSERT INTO schema_version VALUES (2);
";

/// Seeds the `check_for_updates` setting for users upgrading from a version
/// that did not have this setting. Fresh installs get it via seed_defaults.
const MIGRATION_3: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('check_for_updates', 'true');
    INSERT INTO schema_version VALUES (3);
";

/// Seeds the `global_shortcuts_enabled` setting for all installs. Defaults to
/// 'false' — global shortcuts are now opt-in. This is a breaking change for
/// existing users who relied on shortcuts being active by default; they must
/// re-enable them in Settings → Shortcuts.
const MIGRATION_4: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('global_shortcuts_enabled', 'false');
    INSERT INTO schema_version VALUES (4);
";

/// Seeds the `short_breaks_enabled` and `long_breaks_enabled` settings for
/// users upgrading from a version that did not have these settings.
/// Both default to 'true' — existing behaviour is preserved.
const MIGRATION_5: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('short_breaks_enabled', 'true');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('long_breaks_enabled', 'true');
    INSERT INTO schema_version VALUES (5);
";

/// Seeds the seven local shortcut key bindings for users upgrading from a version
/// that did not have this feature. These shortcuts are handled entirely by the frontend
/// (keydown listeners) and require no Rust-side dispatch logic.
const MIGRATION_6: &str = "
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_toggle', ' ');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_reset', 'ArrowLeft');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_skip', 'ArrowRight');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_volume_down', 'ArrowDown');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_volume_up', 'ArrowUp');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_mute', 'm');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('local_shortcut_fullscreen', 'F11');
    INSERT INTO schema_version VALUES (6);
";

/// Adds a nullable `label` column to the `sessions` table to support task labeling.
/// Existing rows default to NULL (no label). No constraints — labels are free-form text.
const MIGRATION_7: &str = "
    ALTER TABLE sessions ADD COLUMN label TEXT;
    INSERT INTO schema_version VALUES (7);
";

/// Apply any pending migrations. Each migration is wrapped in a transaction
/// so a partial failure leaves the database unchanged.
pub fn run(conn: &Connection) -> Result<()> {
    let version = current_version(conn)?;

    if version < 1 {
        log::info!("[db/migrations] applying MIGRATION_1: initial schema");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_1} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_1 complete");
    }

    if version < 2 {
        log::info!("[db/migrations] applying MIGRATION_2: timer durations minutes → seconds");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_2} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_2 complete");
    }

    if version < 3 {
        log::info!("[db/migrations] applying MIGRATION_3: seed check_for_updates setting");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_3} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_3 complete");
    }

    if version < 4 {
        log::info!("[db/migrations] applying MIGRATION_4: seed global_shortcuts_enabled setting");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_4} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_4 complete");
    }

    if version < 5 {
        log::info!("[db/migrations] applying MIGRATION_5: seed short_breaks_enabled and long_breaks_enabled");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_5} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_5 complete");
    }

    if version < 6 {
        log::info!("[db/migrations] applying MIGRATION_6: seed local shortcut key bindings");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_6} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_6 complete");
    }

    if version < 7 {
        log::info!("[db/migrations] applying MIGRATION_7: add label column to sessions");
        conn.execute_batch(&format!("BEGIN; {MIGRATION_7} COMMIT;"))?;
        log::info!("[db/migrations] MIGRATION_7 complete");
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
        assert_eq!(v, 7);
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
