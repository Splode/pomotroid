use rusqlite::{params, Connection, OptionalExtension, Result};
use serde::Serialize;
use std::collections::HashSet;

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
    let id = conn.last_insert_rowid();
    log::debug!("[db] session started: id={id} type={round_type} duration={duration_secs}s");
    Ok(id)
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
    log::debug!("[db] session ended: id={session_id} completed={completed}");
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
pub fn get_daily_stats(conn: &Connection) -> Result<DailyStats> {
    let today: String = conn.query_row(
        "SELECT date('now', 'localtime')",
        [],
        |r| r.get(0),
    )?;

    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work'
         AND date(started_at, 'unixepoch', 'localtime') = ?1",
        [&today],
        |r| r.get(0),
    )?;

    let completed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') = ?1",
        [&today],
        |r| r.get(0),
    )?;

    let focus_secs: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0) FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') = ?1",
        [&today],
        |r| r.get(0),
    )?;

    let mut by_hour = vec![0u32; 24];
    let mut stmt = conn.prepare(
        "SELECT CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) as h,
                COUNT(*) as cnt
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') = ?1
         GROUP BY h",
    )?;
    let rows = stmt.query_map([&today], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, u32>(1)?)))?;
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
pub fn get_weekly_stats(conn: &Connection) -> Result<Vec<DayStat>> {
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day,
                COUNT(*) as rounds
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND date(started_at, 'unixepoch', 'localtime') >= date('now', 'localtime', '-6 days')
         GROUP BY day
         ORDER BY day",
    )?;
    let rows = stmt.query_map([], |r| Ok(DayStat { date: r.get(0)?, rounds: r.get(1)? }))?
        .collect();
    rows
}

/// Completed work rounds per local calendar day, all time (no date limit).
/// The frontend slices this into per-year views for navigation.
pub fn get_heatmap_data(conn: &Connection) -> Result<Vec<HeatmapEntry>> {
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day,
                COUNT(*) as cnt
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         GROUP BY day
         ORDER BY day",
    )?;
    let rows = stmt.query_map([], |r| Ok(HeatmapEntry { date: r.get(0)?, count: r.get(1)? }))?
        .collect();
    rows
}

/// Current and longest work-session streaks (consecutive local calendar days).
/// A streak stays active until midnight: if yesterday had sessions but today does not,
/// the streak is still counted as current.
pub fn get_streak(conn: &Connection) -> Result<StreakInfo> {
    let today: String = conn.query_row(
        "SELECT date('now', 'localtime')",
        [],
        |r| r.get(0),
    )?;

    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         GROUP BY day
         ORDER BY day",
    )?;
    let days: Vec<String> = stmt
        .query_map([], |r| r.get(0))?
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
// Achievement queries
// ---------------------------------------------------------------------------

/// Returns the set of achievement IDs that have been unlocked.
pub fn get_earned_achievement_ids(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare("SELECT id FROM achievements")?;
    let ids = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .flatten()
        .collect();
    Ok(ids)
}

/// Inserts an achievement unlock record. No-ops if the ID already exists.
pub fn insert_achievement(conn: &Connection, id: &str, unlocked_at: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO achievements (id, unlocked_at) VALUES (?1, ?2)",
        params![id, unlocked_at],
    )?;
    Ok(())
}

/// Returns the unlock timestamp for a specific achievement, if earned.
pub fn get_achievement_unlocked_at(conn: &Connection, id: &str) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT unlocked_at FROM achievements WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )
    .optional()
}

/// Max completed work sessions in any single local calendar day.
pub fn get_hat_trick_max_day(conn: &Connection) -> Result<u32> {
    let max: u32 = conn.query_row(
        "SELECT COALESCE(MAX(cnt), 0) FROM (
            SELECT COUNT(*) as cnt
            FROM sessions
            WHERE round_type = 'work' AND completed = 1
            GROUP BY date(started_at, 'unixepoch', 'localtime')
         )",
        [],
        |r| r.get(0),
    )?;
    Ok(max)
}

/// True if any completed work session was started before 07:00 local time.
pub fn get_early_sessions(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) < 7",
        [],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

/// True if any completed work session was started at or after 23:00 local time.
pub fn get_midnight_sessions(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1
         AND CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) >= 23",
        [],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

/// True if any local calendar day had ≥4 started work sessions with 100% completion.
pub fn get_perfect_day_exists(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
            SELECT
                date(started_at, 'unixepoch', 'localtime') as day,
                COUNT(*) as total,
                SUM(completed) as done
            FROM sessions
            WHERE round_type = 'work'
            GROUP BY day
            HAVING total >= 4 AND total = done
         )",
        [],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

/// True if ≥5 distinct local calendar days share the same clock-hour of a completed work session.
pub fn get_creature_of_habit(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
            SELECT
                CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) as hour,
                COUNT(DISTINCT date(started_at, 'unixepoch', 'localtime')) as day_count
            FROM sessions
            WHERE round_type = 'work' AND completed = 1
            GROUP BY hour
            HAVING day_count >= 5
         )",
        [],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

/// True if the user has prior session history, a current streak of 1, and a gap
/// of ≥2 days between the second-most-recent and most-recent active session days.
pub fn get_comeback_kid(conn: &Connection) -> Result<bool> {
    // Get all distinct days with completed work sessions, ordered ascending.
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') as day
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         GROUP BY day
         ORDER BY day",
    )?;
    let days: Vec<String> = stmt
        .query_map([], |r| r.get(0))?
        .flatten()
        .collect();

    if days.len() < 2 {
        return Ok(false);
    }

    let today: String = conn.query_row(
        "SELECT date('now', 'localtime')", [], |r| r.get(0),
    )?;

    let streak = crate::db::queries::compute_streak(&days, &today);
    if streak.current != 1 {
        return Ok(false);
    }

    // Check gap between second-to-last day and last day.
    let last = days.last().unwrap();
    let prev = &days[days.len() - 2];
    let last_n = date_to_day_num(last).unwrap_or(0);
    let prev_n = date_to_day_num(prev).unwrap_or(0);
    Ok(last_n - prev_n >= 2)
}

/// True if any long-break session is marked completed.
pub fn get_long_break_completed(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions WHERE round_type = 'long-break' AND completed = 1",
        [],
        |r| r.get(0),
    )?;
    Ok(count > 0)
}

// ---------------------------------------------------------------------------
// Additional achievement queries (new achievements)
// ---------------------------------------------------------------------------

/// Longest streak of consecutive days where ≥3 completed work sessions were
/// started before noon (local time).  Used for Morning Ritual (target: 5).
pub fn get_morning_ritual_streak(conn: &Connection) -> u32 {
    let days: Vec<String> = conn
        .prepare(
            "SELECT date(started_at, 'unixepoch', 'localtime') as day
             FROM sessions
             WHERE round_type = 'work' AND completed = 1
               AND CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) < 12
             GROUP BY day HAVING COUNT(*) >= 3
             ORDER BY day",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();
    max_consecutive_day_streak(&days)
}

/// True if any calendar week (Mon–Sun) had at least one completed work session
/// and zero skipped break sessions.  Used for Balanced.
pub fn get_balanced_week(conn: &Connection) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
            SELECT strftime('%Y-%W', started_at, 'unixepoch', 'localtime') as wk
            FROM sessions WHERE round_type = 'work' AND completed = 1
            GROUP BY wk
         ) work_weeks
         WHERE wk NOT IN (
             SELECT DISTINCT strftime('%Y-%W', started_at, 'unixepoch', 'localtime')
             FROM sessions WHERE round_type != 'work' AND completed = 0
         )",
        [],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// True if no long-break session has been skipped in the last 14 days AND
/// the user has had at least one long-break (complete or otherwise) in that
/// window.  Used for Stretch Break.
pub fn get_stretch_break(conn: &Connection) -> bool {
    let cutoff = unix_now() - 14 * 86_400;
    // Any skipped long break in the last 14 days?
    let skipped: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'long-break' AND completed = 0 AND started_at >= ?1",
        params![cutoff],
        |r| r.get(0),
    ).unwrap_or(1);
    if skipped > 0 { return false; }
    // At least one long break event in the window (so the user actually had some).
    let any: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'long-break' AND started_at >= ?1",
        params![cutoff],
        |r| r.get(0),
    ).unwrap_or(0);
    any > 0
}

/// Longest streak of consecutive calendar weeks where stats were opened at
/// least once.  Used for Weekly Review (target: 4).
pub fn get_stats_weekly_streak(conn: &Connection) -> u32 {
    let weeks: Vec<String> = conn
        .prepare(
            "SELECT DISTINCT strftime('%Y-%W', ts, 'unixepoch') as wk
             FROM events WHERE name = 'stats_opened'
             ORDER BY wk ASC",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();

    let mut max_streak = 0u32;
    let mut run = 0u32;
    for i in 0..weeks.len() {
        if i == 0 {
            run = 1;
        } else if is_next_week(&weeks[i - 1], &weeks[i]) {
            run += 1;
        } else {
            run = 1;
        }
        if run > max_streak { max_streak = run; }
    }
    max_streak
}

fn is_next_week(w1: &str, w2: &str) -> bool {
    let parse = |w: &str| -> Option<(i32, i32)> {
        let mut it = w.splitn(2, '-');
        Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
    };
    let (Some((y1, wk1)), Some((y2, wk2))) = (parse(w1), parse(w2)) else { return false };
    (y1 == y2 && wk2 == wk1 + 1) || (y2 == y1 + 1 && wk1 >= 52 && wk2 <= 1)
}

/// Maximum consecutive completed work sessions where no break was skipped
/// between any two.  Used for Flow State (target: 4).
pub fn get_flow_state(conn: &Connection) -> u32 {
    conn.query_row(
        "WITH with_resets AS (
             SELECT round_type, completed,
                 SUM(CASE WHEN round_type != 'work' AND completed = 0 THEN 1 ELSE 0 END)
                     OVER (ORDER BY id ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS grp
             FROM sessions
         )
         SELECT COALESCE(MAX(cnt), 0) FROM (
             SELECT COUNT(*) AS cnt FROM with_resets
             WHERE round_type = 'work' AND completed = 1
             GROUP BY grp
         )",
        [],
        |r| r.get(0),
    ).unwrap_or(0)
}

/// Longest streak of consecutive days where ≥5 completed work sessions occurred.
/// Used for Heatmap Inferno (target: 7).
pub fn get_heatmap_inferno_streak(conn: &Connection) -> u32 {
    let days: Vec<String> = conn
        .prepare(
            "SELECT date(started_at, 'unixepoch', 'localtime') as day
             FROM sessions
             WHERE round_type = 'work' AND completed = 1
             GROUP BY day HAVING COUNT(*) >= 5
             ORDER BY day",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();
    max_consecutive_day_streak(&days)
}

/// Count of distinct payload values for a named event.
/// Used for The Full Palette (count distinct themes applied).
pub fn count_distinct_event_payloads(conn: &Connection, name: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(DISTINCT payload) FROM events WHERE name = ?1 AND payload IS NOT NULL",
        params![name],
        |r| r.get(0),
    ).unwrap_or(0)
}

/// True if any day had ≥4 completed work sessions and zero skipped breaks.
/// Used for Rest is Productive.
pub fn get_rest_is_productive(conn: &Connection) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
             SELECT date(started_at, 'unixepoch', 'localtime') as day
             FROM sessions WHERE round_type = 'work' AND completed = 1
             GROUP BY day HAVING COUNT(*) >= 4
         ) work_days
         WHERE day NOT IN (
             SELECT DISTINCT date(started_at, 'unixepoch', 'localtime')
             FROM sessions WHERE round_type != 'work' AND completed = 0
         )",
        [],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// Count of work sessions that started within 3 seconds of any preceding
/// session ending.  Used for Too Eager (target: 3).
pub fn get_too_eager_count(conn: &Connection) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM sessions s2
         WHERE s2.round_type = 'work' AND s2.completed = 1
           AND s2.ended_at IS NOT NULL
           AND EXISTS (
               SELECT 1 FROM sessions s1
               WHERE s1.id < s2.id
                 AND s1.ended_at IS NOT NULL
                 AND s2.started_at - s1.ended_at BETWEEN 0 AND 3
           )",
        [],
        |r| r.get(0),
    ).unwrap_or(0)
}

/// Longest streak of consecutive days where the first completed work session
/// started more than 5 minutes after the first app_launched event that day.
/// Used for Slow and Steady (target: 7).
pub fn get_slow_and_steady_streak(conn: &Connection) -> u32 {
    let days: Vec<String> = conn
        .prepare(
            "SELECT date(s.started_at, 'unixepoch', 'localtime') as day
             FROM sessions s
             WHERE s.round_type = 'work' AND s.completed = 1
             GROUP BY day
             HAVING (
                 SELECT MIN(e.ts) FROM events e
                 WHERE e.name = 'app_launched'
                   AND date(e.ts, 'unixepoch', 'localtime') = day
             ) IS NOT NULL
             AND MIN(s.started_at) - (
                 SELECT MIN(e.ts) FROM events e
                 WHERE e.name = 'app_launched'
                   AND date(e.ts, 'unixepoch', 'localtime') = day
             ) > 300
             ORDER BY day",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();
    max_consecutive_day_streak(&days)
}

/// True if there are 10 or more consecutive settings_opened events with no
/// settings_saved event between them.  Used for Obsessive Saver.
pub fn get_obsessive_saver(conn: &Connection) -> bool {
    let events: Vec<String> = conn
        .prepare(
            "SELECT name FROM events
             WHERE name IN ('settings_opened', 'settings_saved')
             ORDER BY ts ASC",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();

    let mut max_run = 0u32;
    let mut run = 0u32;
    for ev in &events {
        if ev == "settings_opened" {
            run += 1;
            if run > max_run { max_run = run; }
        } else {
            run = 0;
        }
    }
    max_run >= 10
}

/// True if any completed work session was started on the given month and day
/// (local time).  Used for holiday achievements.
pub fn get_session_on_month_day(conn: &Connection, month: u32, day: u32) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1
           AND CAST(strftime('%m', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) = ?1
           AND CAST(strftime('%d', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) = ?2",
        params![month, day],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// True if any local calendar day had exactly `exact` completed work sessions.
/// Used for Lucky Streak (7) and Perfect Ten (10).
pub fn get_exact_daily_work_count(conn: &Connection, exact: u32) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
             SELECT date(started_at, 'unixepoch', 'localtime') as day
             FROM sessions WHERE round_type = 'work' AND completed = 1
             GROUP BY day HAVING COUNT(*) = ?1
         )",
        params![exact],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// True if the user has had 5 or more consecutive days with app_launched events
/// and zero settings_saved events on any of those days.  Used for Ghost Mode.
pub fn get_ghost_mode_streak(conn: &Connection) -> bool {
    let days: Vec<String> = conn
        .prepare(
            "SELECT DISTINCT date(ts, 'unixepoch') as day
             FROM events WHERE name = 'app_launched'
             AND date(ts, 'unixepoch') NOT IN (
                 SELECT DISTINCT date(ts, 'unixepoch') FROM events WHERE name = 'settings_saved'
             )
             ORDER BY day ASC",
        )
        .and_then(|mut s| Ok(s.query_map([], |r| r.get(0))?.flatten().collect()))
        .unwrap_or_default();
    max_consecutive_day_streak(&days) >= 5
}

/// True if any completed work session used a duration of 3600+ seconds (60 min).
/// Used for Marathon.
pub fn get_marathon(conn: &Connection) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions
         WHERE round_type = 'work' AND completed = 1 AND duration_secs >= 3600",
        [],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// True if there are 3 or more consecutive completed work sessions with
/// duration_secs ≤ 300 (5 minutes).  Used for Baby Steps.
pub fn get_baby_steps(conn: &Connection) -> bool {
    let max: u32 = conn.query_row(
        "WITH with_resets AS (
             SELECT completed, duration_secs,
                 SUM(CASE WHEN round_type = 'work' AND (completed = 0 OR duration_secs > 300)
                          THEN 1 ELSE 0 END)
                     OVER (ORDER BY id ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS grp
             FROM sessions
             WHERE round_type = 'work'
         )
         SELECT COALESCE(MAX(cnt), 0) FROM (
             SELECT COUNT(*) AS cnt FROM with_resets
             WHERE completed = 1 AND duration_secs <= 300
             GROUP BY grp
         )",
        [],
        |r| r.get(0),
    ).unwrap_or(0);
    max >= 3
}

/// True if any completed work session followed a gap of 14+ days from the
/// previous session.  Used for Cold Spell.
pub fn get_cold_spell(conn: &Connection) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions s2
         WHERE s2.round_type = 'work' AND s2.completed = 1
           AND EXISTS (
               SELECT 1 FROM sessions s1
               WHERE s1.id < s2.id
                 AND s1.ended_at IS NOT NULL
                 AND s2.started_at - s1.ended_at >= 1209600
           )",
        [],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// True if a named event occurred on December 31st (any year, local time).
/// Used for History Buff.
pub fn get_event_on_dec31(conn: &Connection, event_name: &str) -> bool {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM events
         WHERE name = ?1
           AND CAST(strftime('%m', datetime(ts, 'unixepoch', 'localtime')) AS INTEGER) = 12
           AND CAST(strftime('%d', datetime(ts, 'unixepoch', 'localtime')) AS INTEGER) = 31",
        params![event_name],
        |r| r.get(0),
    ).unwrap_or(0);
    count > 0
}

/// Maximum streak of consecutive days in a sorted list of date strings.
fn max_consecutive_day_streak(days: &[String]) -> u32 {
    if days.is_empty() { return 0; }
    let nums: Vec<i32> = days.iter().filter_map(|s| date_to_day_num(s)).collect();
    let mut max_run = 1u32;
    let mut run = 1u32;
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] + 1 {
            run += 1;
            if run > max_run { max_run = run; }
        } else {
            run = 1;
        }
    }
    max_run
}

// ---------------------------------------------------------------------------
// Event log queries
// ---------------------------------------------------------------------------

/// Delete events older than `days` days.  Called on startup to keep the table lean.
pub fn prune_events(conn: &Connection, days: i64) {
    let cutoff = unix_now() - days * 86_400;
    if let Err(e) = conn.execute("DELETE FROM events WHERE ts < ?1", params![cutoff]) {
        log::warn!("[db] prune_events failed: {e}");
    }
}

/// Insert a named event into the log.  `payload` is optional JSON context.
pub fn insert_event(conn: &Connection, name: &str, payload: Option<&str>) -> Result<i64> {
    let ts = unix_now();
    conn.execute(
        "INSERT INTO events (name, ts, payload) VALUES (?1, ?2, ?3)",
        params![name, ts, payload],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Count how many times a named event has been recorded.
pub fn count_events(conn: &Connection, name: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM events WHERE name = ?1",
        params![name],
        |r| r.get(0),
    )
    .unwrap_or(0)
}

/// Count events matching `name` whose payload equals `payload`.
pub fn count_events_with_payload(conn: &Connection, name: &str, payload: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM events WHERE name = ?1 AND payload = ?2",
        params![name, payload],
        |r| r.get(0),
    )
    .unwrap_or(0)
}

/// Count how many distinct calendar days (UTC) a named event has occurred on.
pub fn count_event_days(conn: &Connection, name: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(DISTINCT date(ts, 'unixepoch')) FROM events WHERE name = ?1",
        params![name],
        |r| r.get(0),
    )
    .unwrap_or(0)
}

/// Count distinct consecutive days up to today on which a named event occurred.
/// Returns the current streak length (0 if the event didn't occur today).
pub fn event_current_streak(conn: &Connection, name: &str) -> i64 {
    // Pull all distinct days descending; walk until a gap is found.
    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT date(ts, 'unixepoch') AS d
             FROM events WHERE name = ?1
             ORDER BY d DESC",
        )
        .unwrap();
    let days: Vec<String> = stmt
        .query_map(params![name], |r| r.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let today = {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Format as YYYY-MM-DD (UTC)
        chrono_day_from_secs(secs)
    };

    let mut streak = 0i64;
    let mut expected = today;
    for day in &days {
        if day.as_str() == expected.as_str() {
            streak += 1;
            expected = prev_day(&expected);
        } else {
            break;
        }
    }
    streak
}

fn chrono_day_from_secs(secs: u64) -> String {
    // Simple UTC date without pulling in chrono: days since epoch.
    let days = secs / 86400;
    let (y, m, d) = days_to_ymd(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

fn prev_day(date: &str) -> String {
    // Parse YYYY-MM-DD, subtract one day.
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 { return date.to_string(); }
    let (y, m, d): (i64, i64, i64) = (
        parts[0].parse().unwrap_or(2000),
        parts[1].parse().unwrap_or(1),
        parts[2].parse().unwrap_or(1),
    );
    let days = ymd_to_days(y, m, d) - 1;
    let (ny, nm, nd) = days_to_ymd(days);
    format!("{ny:04}-{nm:02}-{nd:02}")
}

fn ymd_to_days(y: i64, m: i64, d: i64) -> i64 {
    // Days since Unix epoch (1970-01-01) using proleptic Gregorian calendar.
    let m = if m <= 2 { m + 12 } else { m };
    let y = if m <= 14 { y - 1 } else { y };  // Adjust for Jan/Feb
    let a = y / 100;
    let b = 2 - a + a / 4;
    ((365.25 * (y + 4716) as f64) as i64)
        + ((30.6001 * (m + 1) as f64) as i64)
        + d + b - 1524 - 2440588
}

fn days_to_ymd(days: i64) -> (i64, i64, i64) {
    let jd = days + 2440588; // Unix epoch → Julian day
    let a = jd + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + m / 10;
    (year, month, day)
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
        let stats = get_daily_stats(&conn).unwrap();
        assert_eq!(stats.rounds, 0);
        assert_eq!(stats.focus_mins, 0);
        assert!(stats.completion_rate.is_none());
        assert_eq!(stats.by_hour.len(), 24);
    }

    #[test]
    fn get_weekly_stats_empty() {
        let conn = setup();
        let stats = get_weekly_stats(&conn).unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn get_heatmap_data_empty() {
        let conn = setup();
        let entries = get_heatmap_data(&conn).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn focus_mins_rounds_to_nearest_minute() {
        let conn = setup();

        // 339 s = 5:39 → rounds up to 6 min (remainder 39 ≥ 30).
        let id1 = insert_session(&conn, "work", 339).unwrap();
        complete_session(&conn, id1, true).unwrap();
        let stats = get_daily_stats(&conn).unwrap();
        assert_eq!(stats.focus_mins, 6, "339 s should round to 6 min");

        // Reset and test round-down: 324 s = 5:24 → rounds down to 5 min (remainder 24 < 30).
        let conn2 = setup();
        let id2 = insert_session(&conn2, "work", 324).unwrap();
        complete_session(&conn2, id2, true).unwrap();
        let stats2 = get_daily_stats(&conn2).unwrap();
        assert_eq!(stats2.focus_mins, 5, "324 s should round to 5 min");

        // Exact minute boundary: 1500 s = 25:00 → stays 25 min.
        let conn3 = setup();
        let id3 = insert_session(&conn3, "work", 1500).unwrap();
        complete_session(&conn3, id3, true).unwrap();
        let stats3 = get_daily_stats(&conn3).unwrap();
        assert_eq!(stats3.focus_mins, 25, "1500 s should be exactly 25 min");
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
