pub mod migrations;
pub mod queries;

use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

/// Thread-safe handle to the SQLite connection.
/// Registered as Tauri managed state so commands can access it.
pub type DbState = Arc<Mutex<Connection>>;

/// Open (or create) the `pomotroid.db` file inside `app_data_dir`,
/// enable WAL mode for better concurrent read performance,
/// and run any pending schema migrations.
pub fn open(app_data_dir: &std::path::Path) -> Result<DbState> {
    let db_path = app_data_dir.join("pomotroid.db");
    let conn = Connection::open(&db_path)?;

    // WAL mode: readers don't block writers and vice-versa.
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    migrations::run(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}
