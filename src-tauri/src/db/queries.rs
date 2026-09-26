use rusqlite::{named_params, params, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Session CRUD (DATA-03)
// ---------------------------------------------------------------------------

/// Inserts a new session row when a round begins.
/// Returns the row ID so it can be passed to `complete_session` later.
pub fn insert_session(
    conn: &Connection,
    round_type: &str,
    duration_secs: u32,
    category_id: Option<i64>,
) -> Result<i64> {
    let started_at = unix_now();
    conn.execute(
        "INSERT INTO sessions (started_at, round_type, duration_secs, completed, category_id)
         VALUES (?1, ?2, ?3, 0, ?4)",
        params![started_at, round_type, duration_secs, category_id],
    )?;
    let id = conn.last_insert_rowid();
    log::debug!(
        "[db] session started: id={id} type={round_type} duration={duration_secs}s category={category_id:?}"
    );
    Ok(id)
}

/// Updates a session when the round ends (by completion or skip).
///
/// `category_id` is the category active when the round ended; `None` leaves
/// the session uncategorized (breaks, or the categories feature is off).
pub fn complete_session(
    conn: &Connection,
    session_id: i64,
    completed: bool,
    category_id: Option<i64>,
) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET ended_at = ?1, completed = ?2, category_id = ?3 WHERE id = ?4",
        params![unix_now(), completed as i64, category_id, session_id],
    )?;
    log::debug!(
        "[db] session ended: id={session_id} completed={completed} category={category_id:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------

/// Maximum number of categories that can exist at once.
pub const MAX_CATEGORIES: i64 = 10;

/// Maximum length of a category name, in characters.
pub const MAX_CATEGORY_NAME_CHARS: usize = 20;

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub id: i64,
    /// User-provided name. `None` for a built-in category that has not been
    /// renamed — the frontend shows a localized name for `builtin_key` instead.
    pub name: Option<String>,
    /// "work" | "study" | "leisure" for the built-in categories, `None` otherwise.
    pub builtin_key: Option<String>,
    /// Hex color, e.g. "#4A9FF5".
    pub color: String,
    pub position: i64,
}

pub fn list_categories(conn: &Connection) -> Result<Vec<Category>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, builtin_key, color, position FROM categories ORDER BY position, id",
    )?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                builtin_key: r.get(2)?,
                color: r.get(3)?,
                position: r.get(4)?,
            })
        })?
        .collect();
    rows
}

pub fn count_categories(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0))
}

/// Appends a user-created category at the end of the list and returns its id.
pub fn create_category(conn: &Connection, name: &str, color: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO categories (name, color, position)
         VALUES (?1, ?2, (SELECT COALESCE(MAX(position), -1) + 1 FROM categories))",
        params![name, color],
    )?;
    let id = conn.last_insert_rowid();
    log::debug!("[db] category created: id={id}");
    Ok(id)
}

/// Renames and recolors a category. A `None` name on a built-in category
/// restores its localized default name.
pub fn update_category(conn: &Connection, id: i64, name: Option<&str>, color: &str) -> Result<()> {
    conn.execute(
        "UPDATE categories SET name = ?1, color = ?2 WHERE id = ?3",
        params![name, color, id],
    )?;
    log::debug!("[db] category updated: id={id}");
    Ok(())
}

/// Deletes a category. Its sessions become uncategorized rather than being
/// deleted, so no history is lost. Returns the number of sessions moved.
pub fn delete_category(conn: &Connection, id: i64) -> Result<usize> {
    let tx = conn.unchecked_transaction()?;
    let moved = tx.execute(
        "UPDATE sessions SET category_id = NULL WHERE category_id = ?1",
        [id],
    )?;
    tx.execute("DELETE FROM categories WHERE id = ?1", [id])?;
    tx.commit()?;
    log::debug!("[db] category deleted: id={id} sessions_moved={moved}");
    Ok(moved)
}

/// Returns true for the built-in categories (work, study, leisure).
pub fn is_builtin_category(conn: &Connection, id: i64) -> Result<bool> {
    conn.query_row(
        "SELECT builtin_key IS NOT NULL FROM categories WHERE id = ?1",
        [id],
        |r| r.get(0),
    )
}

/// Completed focus rounds recorded under a category.
pub fn count_category_rounds(conn: &Connection, id: i64) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1 AND category_id = ?1",
        [id],
        |r| r.get(0),
    )
}

/// Completed focus rounds that have no category.
pub fn count_uncategorized_rounds(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1 AND category_id IS NULL",
        [],
        |r| r.get(0),
    )
}

/// Resolves the category a finished focus round belongs to: the stored
/// selection if it still exists, otherwise the first category. `None` when
/// every category has been deleted.
pub fn resolve_active_category(conn: &Connection, stored_id: i64) -> Result<Option<i64>> {
    let stored: Option<i64> = conn
        .query_row("SELECT id FROM categories WHERE id = ?1", [stored_id], |r| r.get(0))
        .optional()?;
    if stored.is_some() {
        return Ok(stored);
    }
    conn.query_row("SELECT id FROM categories ORDER BY position, id LIMIT 1", [], |r| r.get(0))
        .optional()
}

/// The category a round of `round_type` is recorded under: the active
/// category for focus rounds while categories are enabled, otherwise none.
pub fn round_category(
    conn: &Connection,
    categories_enabled: bool,
    active_category_id: i64,
    round_type: &str,
) -> Result<Option<i64>> {
    if !categories_enabled || round_type != "work" {
        return Ok(None);
    }
    resolve_active_category(conn, active_category_id)
}

/// Trims a category name and checks it is non-empty and short enough.
pub fn validate_category_name(name: &str) -> std::result::Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("category name must not be empty".to_string());
    }
    if trimmed.chars().count() > MAX_CATEGORY_NAME_CHARS {
        return Err(format!(
            "category name must be at most {MAX_CATEGORY_NAME_CHARS} characters"
        ));
    }
    Ok(trimmed.to_string())
}

/// Accepts only `#RRGGBB` hex colors.
pub fn validate_category_color(color: &str) -> std::result::Result<(), String> {
    let valid = color.len() == 7
        && color.starts_with('#')
        && color[1..].chars().all(|c| c.is_ascii_hexdigit());
    if valid {
        Ok(())
    } else {
        Err(format!("invalid category color: '{color}'"))
    }
}

// ---------------------------------------------------------------------------
// Stats filter
// ---------------------------------------------------------------------------

/// Categories to leave out of the stats queries. Passing `None` instead of a
/// filter means no filtering at all (the categories feature is off).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct StatsFilter {
    /// Category ids to leave out.
    #[serde(default)]
    pub hidden_ids: Vec<i64>,
    /// Leave out sessions that have no category.
    #[serde(default)]
    pub hide_uncategorized: bool,
}

/// SQL fragment appended to every stats query's WHERE clause. Binds
/// `:filter_on`, `:hide_uncat` and `:hidden_ids` (a JSON array of ids).
macro_rules! category_filter_sql {
    () => {
        " AND (:filter_on = 0
               OR (category_id IS NULL AND :hide_uncat = 0)
               OR (category_id IS NOT NULL
                   AND category_id NOT IN (SELECT value FROM json_each(:hidden_ids))))"
    };
}

/// Bound values for `category_filter_sql!`.
struct FilterParams {
    on: bool,
    hide_uncat: bool,
    hidden_ids: String,
}

impl FilterParams {
    fn new(filter: Option<&StatsFilter>) -> Self {
        match filter {
            Some(f) => Self {
                on: true,
                hide_uncat: f.hide_uncategorized,
                hidden_ids: serde_json::to_string(&f.hidden_ids).unwrap_or_else(|_| "[]".into()),
            },
            None => Self { on: false, hide_uncat: false, hidden_ids: "[]".into() },
        }
    }
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

pub fn get_all_time_stats(conn: &Connection, filter: Option<&StatsFilter>) -> Result<SessionStats> {
    let f = FilterParams::new(filter);
    let fp = named_params! {
        ":filter_on": f.on,
        ":hide_uncat": f.hide_uncat,
        ":hidden_ids": f.hidden_ids,
    };

    let total_work_sessions: i64 = conn.query_row(
        concat!("SELECT COUNT(*) FROM sessions WHERE round_type = 'work'", category_filter_sql!()),
        fp,
        |r| r.get(0),
    )?;

    let completed_work_sessions: i64 = conn.query_row(
        concat!(
            "SELECT COUNT(*) FROM sessions WHERE round_type = 'work' AND completed = 1",
            category_filter_sql!()
        ),
        fp,
        |r| r.get(0),
    )?;

    let total_work_secs: i64 = conn.query_row(
        concat!(
            "SELECT COALESCE(SUM(duration_secs), 0)
             FROM sessions WHERE round_type = 'work' AND completed = 1",
            category_filter_sql!()
        ),
        fp,
        |r| r.get(0),
    )?;

    Ok(SessionStats {
        total_work_sessions,
        completed_work_sessions,
        total_work_secs,
    })
}

// ---------------------------------------------------------------------------
// Detailed stats queries (DATA-04)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct DailyStats {
    pub rounds: u32,
    pub focus_mins: u32,
    /// None when no work sessions were started today (avoids 0/0).
    pub completion_rate: Option<f32>,
    /// Completed work rounds per hour of the day (index 0 = midnight).
    pub by_hour: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct DayStat {
    /// Local calendar date in "YYYY-MM-DD" format.
    pub date: String,
    pub rounds: u32,
}

#[derive(Debug, Serialize)]
pub struct HeatmapEntry {
    /// Local calendar date in "YYYY-MM-DD" format.
    pub date: String,
    pub count: u32,
}

#[derive(Debug, Serialize)]
pub struct StreakInfo {
    pub current: u32,
    pub longest: u32,
}

/// Completed work rounds and focus time for today (local calendar date).
pub fn get_daily_stats(conn: &Connection, filter: Option<&StatsFilter>) -> Result<DailyStats> {
    let today: String = conn.query_row(
        "SELECT date('now', 'localtime')",
        [],
        |r| r.get(0),
    )?;

    let f = FilterParams::new(filter);
    let fp = named_params! {
        ":today": today,
        ":filter_on": f.on,
        ":hide_uncat": f.hide_uncat,
        ":hidden_ids": f.hidden_ids,
    };

    let total: i64 = conn.query_row(
        concat!(
            "SELECT COUNT(*) FROM sessions
             WHERE round_type = 'work'
             AND date(started_at, 'unixepoch', 'localtime') = :today",
            category_filter_sql!()
        ),
        fp,
        |r| r.get(0),
    )?;

    let completed: i64 = conn.query_row(
        concat!(
            "SELECT COUNT(*) FROM sessions
             WHERE round_type = 'work' AND completed = 1
             AND date(started_at, 'unixepoch', 'localtime') = :today",
            category_filter_sql!()
        ),
        fp,
        |r| r.get(0),
    )?;

    let focus_secs: i64 = conn.query_row(
        concat!(
            "SELECT COALESCE(SUM(duration_secs), 0) FROM sessions
             WHERE round_type = 'work' AND completed = 1
             AND date(started_at, 'unixepoch', 'localtime') = :today",
            category_filter_sql!()
        ),
        fp,
        |r| r.get(0),
    )?;

    let mut by_hour = vec![0u32; 24];
    let mut stmt = conn.prepare(concat!(
        "SELECT CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) as h,
                COUNT(*) as cnt
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') = :today",
        category_filter_sql!(),
        " GROUP BY h"
    ))?;
    let rows = stmt.query_map(fp, |r| Ok((r.get::<_, i64>(0)?, r.get::<_, u32>(1)?)))?;
    for row in rows.flatten() {
        let (h, cnt) = row;
        if (0..24).contains(&h) {
            by_hour[h as usize] = cnt;
        }
    }

    Ok(DailyStats {
        rounds: completed as u32,
        focus_mins: ((focus_secs + 30) / 60) as u32,
        completion_rate: if total > 0 { Some(completed as f32 / total as f32) } else { None },
        by_hour,
    })
}

/// Completed work rounds per local calendar day for the last 7 days.
pub fn get_weekly_stats(conn: &Connection, filter: Option<&StatsFilter>) -> Result<Vec<DayStat>> {
    let f = FilterParams::new(filter);
    let mut stmt = conn.prepare(concat!(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day,
                COUNT(*) as rounds
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') >= date('now', 'localtime', '-6 days')",
        category_filter_sql!(),
        " GROUP BY day ORDER BY day"
    ))?;
    let rows = stmt
        .query_map(
            named_params! {
                ":filter_on": f.on,
                ":hide_uncat": f.hide_uncat,
                ":hidden_ids": f.hidden_ids,
            },
            |r| Ok(DayStat { date: r.get(0)?, rounds: r.get(1)? }),
        )?
        .collect();
    rows
}

/// Completed work rounds per local calendar day, all time (no date limit).
/// The frontend slices this into per-year views for navigation.
pub fn get_heatmap_data(conn: &Connection, filter: Option<&StatsFilter>) -> Result<Vec<HeatmapEntry>> {
    let f = FilterParams::new(filter);
    let mut stmt = conn.prepare(concat!(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day,
                COUNT(*) as cnt
         FROM sessions
         WHERE round_type = 'work' AND completed = 1",
        category_filter_sql!(),
        " GROUP BY day ORDER BY day"
    ))?;
    let rows = stmt
        .query_map(
            named_params! {
                ":filter_on": f.on,
                ":hide_uncat": f.hide_uncat,
                ":hidden_ids": f.hidden_ids,
            },
            |r| Ok(HeatmapEntry { date: r.get(0)?, count: r.get(1)? }),
        )?
        .collect();
    rows
}

/// Current and longest work-session streaks (consecutive local calendar days).
/// A streak stays active until midnight: if yesterday had sessions but today does not,
/// the streak is still counted as current.
pub fn get_streak(conn: &Connection, filter: Option<&StatsFilter>) -> Result<StreakInfo> {
    let today: String = conn.query_row(
        "SELECT date('now', 'localtime')",
        [],
        |r| r.get(0),
    )?;

    let f = FilterParams::new(filter);
    let mut stmt = conn.prepare(concat!(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day
         FROM sessions
         WHERE round_type = 'work' AND completed = 1",
        category_filter_sql!(),
        " GROUP BY day ORDER BY day"
    ))?;
    let days: Vec<String> = stmt
        .query_map(
            named_params! {
                ":filter_on": f.on,
                ":hide_uncat": f.hide_uncat,
                ":hidden_ids": f.hidden_ids,
            },
            |r| r.get(0),
        )?
        .flatten()
        .collect();

    Ok(compute_streak(&days, &today))
}

// ---------------------------------------------------------------------------
// Streak helpers
// ---------------------------------------------------------------------------

/// Convert a "YYYY-MM-DD" string to a day number for arithmetic comparison.
/// Uses the proleptic Gregorian calendar; absolute value is arbitrary — only
/// differences between dates matter.
fn date_to_day_num(s: &str) -> Option<i32> {
    let mut parts = s.splitn(3, '-');
    let y: i32 = parts.next()?.parse().ok()?;
    let m: i32 = parts.next()?.parse().ok()?;
    let d: i32 = parts.next()?.parse().ok()?;
    let y = if m <= 2 { y - 1 } else { y };
    let m = if m <= 2 { m + 12 } else { m };
    Some(y * 365 + y / 4 - y / 100 + y / 400 + (153 * m - 457) / 5 + d)
}

pub fn compute_streak(days: &[String], today: &str) -> StreakInfo {
    let nums: Vec<i32> = days.iter().filter_map(|s| date_to_day_num(s)).collect();
    if nums.is_empty() {
        return StreakInfo { current: 0, longest: 0 };
    }

    let today_n = match date_to_day_num(today) {
        Some(n) => n,
        None => return StreakInfo { current: 0, longest: 0 },
    };

    // Current streak — alive if most recent session day is today or yesterday.
    let last = *nums.last().unwrap();
    let current = if last == today_n || last == today_n - 1 {
        let mut count = 0u32;
        let mut expected = last;
        for &n in nums.iter().rev() {
            if n == expected {
                count += 1;
                expected -= 1;
            } else {
                break;
            }
        }
        count
    } else {
        0
    };

    // Longest streak.
    let mut longest = 1u32;
    let mut run = 1u32;
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] + 1 {
            run += 1;
            if run > longest { longest = run; }
        } else {
            run = 1;
        }
    }

    StreakInfo { current, longest }
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
        let id = insert_session(&conn, "work", 1500, None).unwrap();
        assert!(id > 0);

        complete_session(&conn, id, true, None).unwrap();

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
        let stats = get_all_time_stats(&conn, None).unwrap();
        assert_eq!(stats.total_work_sessions, 0);
        assert_eq!(stats.completed_work_sessions, 0);
        assert_eq!(stats.total_work_secs, 0);
    }

    #[test]
    fn compute_streak_empty() {
        let info = compute_streak(&[], "2024-03-15");
        assert_eq!(info.current, 0);
        assert_eq!(info.longest, 0);
    }

    #[test]
    fn compute_streak_active_today() {
        let days = vec!["2024-03-13".to_string(), "2024-03-14".to_string(), "2024-03-15".to_string()];
        let info = compute_streak(&days, "2024-03-15");
        assert_eq!(info.current, 3);
        assert_eq!(info.longest, 3);
    }

    #[test]
    fn compute_streak_active_until_midnight() {
        // Yesterday had sessions, today does not — streak still live.
        let days = vec!["2024-03-13".to_string(), "2024-03-14".to_string()];
        let info = compute_streak(&days, "2024-03-15");
        assert_eq!(info.current, 2);
    }

    #[test]
    fn compute_streak_broken() {
        // Last session was 2 days ago — streak is broken.
        let days = vec!["2024-03-12".to_string(), "2024-03-13".to_string()];
        let info = compute_streak(&days, "2024-03-15");
        assert_eq!(info.current, 0);
    }

    #[test]
    fn compute_streak_longest_across_break() {
        let days = vec![
            "2024-03-01".to_string(), "2024-03-02".to_string(), "2024-03-03".to_string(),
            "2024-03-10".to_string(), "2024-03-11".to_string(),
        ];
        let info = compute_streak(&days, "2024-03-11");
        assert_eq!(info.current, 2);
        assert_eq!(info.longest, 3);
    }

    #[test]
    fn get_daily_stats_empty() {
        let conn = setup();
        let stats = get_daily_stats(&conn, None).unwrap();
        assert_eq!(stats.rounds, 0);
        assert_eq!(stats.focus_mins, 0);
        assert!(stats.completion_rate.is_none());
        assert_eq!(stats.by_hour.len(), 24);
    }

    #[test]
    fn get_weekly_stats_empty() {
        let conn = setup();
        let stats = get_weekly_stats(&conn, None).unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn get_heatmap_data_empty() {
        let conn = setup();
        let entries = get_heatmap_data(&conn, None).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn focus_mins_rounds_to_nearest_minute() {
        let conn = setup();

        // 339 s = 5:39 → rounds up to 6 min (remainder 39 ≥ 30).
        let id1 = insert_session(&conn, "work", 339, None).unwrap();
        complete_session(&conn, id1, true, None).unwrap();
        let stats = get_daily_stats(&conn, None).unwrap();
        assert_eq!(stats.focus_mins, 6, "339 s should round to 6 min");

        // Reset and test round-down: 324 s = 5:24 → rounds down to 5 min (remainder 24 < 30).
        let conn2 = setup();
        let id2 = insert_session(&conn2, "work", 324, None).unwrap();
        complete_session(&conn2, id2, true, None).unwrap();
        let stats2 = get_daily_stats(&conn2, None).unwrap();
        assert_eq!(stats2.focus_mins, 5, "324 s should round to 5 min");

        // Exact minute boundary: 1500 s = 25:00 → stays 25 min.
        let conn3 = setup();
        let id3 = insert_session(&conn3, "work", 1500, None).unwrap();
        complete_session(&conn3, id3, true, None).unwrap();
        let stats3 = get_daily_stats(&conn3, None).unwrap();
        assert_eq!(stats3.focus_mins, 25, "1500 s should be exactly 25 min");
    }

    #[test]
    fn stats_counts_correctly() {
        let conn = setup();

        let id1 = insert_session(&conn, "work", 1500, None).unwrap();
        complete_session(&conn, id1, true, None).unwrap();

        let id2 = insert_session(&conn, "work", 1500, None).unwrap();
        complete_session(&conn, id2, false, None).unwrap(); // skipped

        let _id3 = insert_session(&conn, "short-break", 300, None).unwrap();

        let stats = get_all_time_stats(&conn, None).unwrap();
        assert_eq!(stats.total_work_sessions, 2);
        assert_eq!(stats.completed_work_sessions, 1);
        assert_eq!(stats.total_work_secs, 1500);
    }

    // --- Categories ---

    fn builtin_id(conn: &Connection, key: &str) -> i64 {
        conn.query_row("SELECT id FROM categories WHERE builtin_key = ?1", [key], |r| r.get(0))
            .unwrap()
    }

    /// Records one completed focus round of `secs` seconds in `category`.
    fn record_round(conn: &Connection, secs: u32, category: Option<i64>) {
        let id = insert_session(conn, "work", secs, category).unwrap();
        complete_session(conn, id, true, category).unwrap();
    }

    #[test]
    fn builtin_categories_are_listed_in_order() {
        let conn = setup();
        let cats = list_categories(&conn).unwrap();
        let keys: Vec<_> = cats.iter().map(|c| c.builtin_key.as_deref()).collect();
        assert_eq!(keys, [Some("work"), Some("study"), Some("leisure")]);
        assert!(cats.iter().all(|c| c.name.is_none()), "built-ins start with localized names");
    }

    #[test]
    fn create_update_and_count_categories() {
        let conn = setup();
        let id = create_category(&conn, "Film", "#F06595").unwrap();
        assert_eq!(count_categories(&conn).unwrap(), 4);

        let cats = list_categories(&conn).unwrap();
        let film = cats.last().unwrap();
        assert_eq!(film.id, id);
        assert_eq!(film.name.as_deref(), Some("Film"));
        assert_eq!(film.position, 3, "new categories are appended");
        assert!(!is_builtin_category(&conn, id).unwrap());

        update_category(&conn, id, Some("Movies"), "#2EC4B6").unwrap();
        let film = list_categories(&conn).unwrap().pop().unwrap();
        assert_eq!(film.name.as_deref(), Some("Movies"));
        assert_eq!(film.color, "#2EC4B6");
    }

    #[test]
    fn delete_category_moves_rounds_to_uncategorized() {
        let conn = setup();
        let film = create_category(&conn, "Film", "#F06595").unwrap();
        record_round(&conn, 1800, Some(film));
        record_round(&conn, 1800, Some(film));
        record_round(&conn, 1800, None);
        assert_eq!(count_category_rounds(&conn, film).unwrap(), 2);

        let moved = delete_category(&conn, film).unwrap();
        assert_eq!(moved, 2);
        assert_eq!(count_uncategorized_rounds(&conn).unwrap(), 3);
        assert_eq!(count_categories(&conn).unwrap(), 3);

        // History is kept: the rounds still count toward the unfiltered totals.
        let stats = get_all_time_stats(&conn, None).unwrap();
        assert_eq!(stats.completed_work_sessions, 3);
    }

    #[test]
    fn resolve_active_category_falls_back_to_first() {
        let conn = setup();
        let work = builtin_id(&conn, "work");
        let study = builtin_id(&conn, "study");

        assert_eq!(resolve_active_category(&conn, study).unwrap(), Some(study));
        // Unset (0) or a deleted id falls back to the first category.
        assert_eq!(resolve_active_category(&conn, 0).unwrap(), Some(work));
        delete_category(&conn, study).unwrap();
        assert_eq!(resolve_active_category(&conn, study).unwrap(), Some(work));

        // With every category deleted there is nothing to resolve to.
        for c in list_categories(&conn).unwrap() {
            delete_category(&conn, c.id).unwrap();
        }
        assert_eq!(resolve_active_category(&conn, work).unwrap(), None);
    }

    #[test]
    fn stats_filter_hides_categories_and_uncategorized() {
        let conn = setup();
        let work = builtin_id(&conn, "work");
        let leisure = builtin_id(&conn, "leisure");
        record_round(&conn, 1800, Some(work));
        record_round(&conn, 1800, Some(work));
        record_round(&conn, 1800, Some(leisure));
        record_round(&conn, 1800, None);

        let rounds = |filter: Option<&StatsFilter>| {
            get_all_time_stats(&conn, filter).unwrap().completed_work_sessions
        };

        // No filter (feature off): everything counts.
        assert_eq!(rounds(None), 4);
        // Empty filter: nothing hidden.
        assert_eq!(rounds(Some(&StatsFilter::default())), 4);

        let no_leisure = StatsFilter { hidden_ids: vec![leisure], hide_uncategorized: false };
        assert_eq!(rounds(Some(&no_leisure)), 3);

        let work_only = StatsFilter { hidden_ids: vec![leisure], hide_uncategorized: true };
        assert_eq!(rounds(Some(&work_only)), 2);
        assert_eq!(get_daily_stats(&conn, Some(&work_only)).unwrap().rounds, 2);
        assert_eq!(get_weekly_stats(&conn, Some(&work_only)).unwrap()[0].rounds, 2);
        assert_eq!(get_heatmap_data(&conn, Some(&work_only)).unwrap()[0].count, 2);
        assert_eq!(get_streak(&conn, Some(&work_only)).unwrap().current, 1);

        let hide_all = StatsFilter { hidden_ids: vec![work, leisure], hide_uncategorized: true };
        assert_eq!(rounds(Some(&hide_all)), 0);
        assert_eq!(get_streak(&conn, Some(&hide_all)).unwrap().current, 0);
    }

    #[test]
    fn round_category_covers_focus_rounds_only_while_enabled() {
        let conn = setup();
        let work = builtin_id(&conn, "work");
        let study = builtin_id(&conn, "study");

        assert_eq!(round_category(&conn, true, study, "work").unwrap(), Some(study));
        assert_eq!(round_category(&conn, false, study, "work").unwrap(), None);
        assert_eq!(round_category(&conn, true, study, "short-break").unwrap(), None);
        assert_eq!(round_category(&conn, true, study, "long-break").unwrap(), None);
        // A deleted active category falls back to the first one.
        delete_category(&conn, study).unwrap();
        assert_eq!(round_category(&conn, true, study, "work").unwrap(), Some(work));
    }

    #[test]
    fn unfinished_rounds_keep_their_category_in_daily_stats() {
        let conn = setup();
        let work = builtin_id(&conn, "work");
        let leisure = builtin_id(&conn, "leisure");
        record_round(&conn, 1500, Some(work));
        record_round(&conn, 1500, Some(leisure));
        // A focus round that was reset before it finished.
        insert_session(&conn, "work", 1500, Some(work)).unwrap();

        let completion_with_hidden = |hidden: i64| {
            let filter = StatsFilter { hidden_ids: vec![hidden], hide_uncategorized: true };
            get_daily_stats(&conn, Some(&filter)).unwrap().completion_rate
        };
        // The unfinished round lowers Work's completion rate, not Leisure's.
        assert_eq!(completion_with_hidden(leisure), Some(0.5));
        assert_eq!(completion_with_hidden(work), Some(1.0));
    }

    #[test]
    fn category_name_and_color_validation() {
        assert_eq!(validate_category_name("  Film  ").unwrap(), "Film");
        assert!(validate_category_name("   ").is_err());
        assert!(validate_category_name(&"x".repeat(MAX_CATEGORY_NAME_CHARS)).is_ok());
        assert!(validate_category_name(&"x".repeat(MAX_CATEGORY_NAME_CHARS + 1)).is_err());
        // Multi-byte characters count as one each.
        assert!(validate_category_name("Çalışma ve Öğrenim").is_ok());

        assert!(validate_category_color("#4A9FF5").is_ok());
        assert!(validate_category_color("#4a9ff5").is_ok());
        assert!(validate_category_color("4A9FF5").is_err());
        assert!(validate_category_color("#4A9FF").is_err());
        assert!(validate_category_color("#4A9FFZ").is_err());
    }
}
