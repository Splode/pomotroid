use rusqlite::{params, Connection, Result};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Session CRUD (DATA-03)
// ---------------------------------------------------------------------------

/// Inserts a new session row when a round begins.
/// Returns the row ID so it can be passed to `complete_session` later.
pub fn insert_session(
    conn: &Connection,
    round_type: &str,
    duration_secs: u32,
) -> Result<i64> {
    let started_at = unix_now();
    conn.execute(
        "INSERT INTO sessions (started_at, round_type, duration_secs, completed)
         VALUES (?1, ?2, ?3, 0)",
        params![started_at, round_type, duration_secs],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Updates a session when the round ends (by completion or skip).
pub fn complete_session(
    conn: &Connection,
    session_id: i64,
    completed: bool,
) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET ended_at = ?1, completed = ?2 WHERE id = ?3",
        params![unix_now(), completed as i64, session_id],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Stats queries
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct SessionStats {
    pub total_work_sessions: i64,
    pub completed_work_sessions: i64,
    /// Sum of duration_secs for all *completed* work sessions.
    pub total_work_secs: i64,
}

pub fn get_all_time_stats(conn: &Connection) -> Result<SessionStats> {
    let total_work_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions WHERE round_type = 'work'",
        [],
        |r| r.get(0),
    )?;

    let completed_work_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions WHERE round_type = 'work' AND completed = 1",
        [],
        |r| r.get(0),
    )?;

    let total_work_secs: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0)
         FROM sessions WHERE round_type = 'work' AND completed = 1",
        [],
        |r| r.get(0),
    )?;

    Ok(SessionStats {
        total_work_sessions,
        completed_work_sessions,
        total_work_secs,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        conn
    }

    #[test]
    fn insert_and_complete_session() {
        let conn = setup();
        let id = insert_session(&conn, "work", 1500).unwrap();
        assert!(id > 0);

        complete_session(&conn, id, true).unwrap();

        let completed: i64 = conn
            .query_row(
                "SELECT completed FROM sessions WHERE id = ?1",
                [id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(completed, 1);
    }

    #[test]
    fn stats_empty_db() {
        let conn = setup();
        let stats = get_all_time_stats(&conn).unwrap();
        assert_eq!(stats.total_work_sessions, 0);
        assert_eq!(stats.completed_work_sessions, 0);
        assert_eq!(stats.total_work_secs, 0);
    }

    #[test]
    fn stats_counts_correctly() {
        let conn = setup();

        let id1 = insert_session(&conn, "work", 1500).unwrap();
        complete_session(&conn, id1, true).unwrap();

        let id2 = insert_session(&conn, "work", 1500).unwrap();
        complete_session(&conn, id2, false).unwrap(); // skipped

        let _id3 = insert_session(&conn, "short-break", 300).unwrap();

        let stats = get_all_time_stats(&conn).unwrap();
        assert_eq!(stats.total_work_sessions, 2);
        assert_eq!(stats.completed_work_sessions, 1);
        assert_eq!(stats.total_work_secs, 1500);
    }
}
