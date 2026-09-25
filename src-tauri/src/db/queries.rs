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
// Insights queries — richer analytics powering the "Better Stats" window
// ---------------------------------------------------------------------------

/// A completed work round reduced to the local calendar day it started on.
struct WorkDay {
    date: String,
    /// Start-of-round hour (0–23) in local time.
    hour: u32,
    duration_secs: i64,
}

/// Per-day rollup used for the trend chart.
#[derive(Debug, Clone, Serialize)]
pub struct TrendPoint {
    /// Local calendar date in "YYYY-MM-DD" format.
    pub date: String,
    pub rounds: u32,
    pub focus_mins: u32,
}

/// Activity snapshot for one rolling window (e.g. the last 7 or 28 days).
#[derive(Debug, Serialize)]
pub struct PeriodSummary {
    pub label: String,
    pub days: u32,
    pub rounds: u32,
    pub focus_mins: u32,
    pub active_days: u32,
    /// Mean rounds per *active* day; `None` when nothing was recorded.
    pub avg_rounds_per_active_day: Option<f32>,
    /// Mean focus minutes per *active* day; `None` when nothing was recorded.
    pub avg_focus_mins_per_active_day: Option<f32>,
    /// Completed / started work rounds in the window; `None` if nothing started.
    pub completion_rate: Option<f32>,
    /// The single best day in the window (by rounds, focus time as tie-break).
    pub best_day: Option<TrendPoint>,
}

/// Aggregate weekday profile across all history.
#[derive(Debug, Serialize)]
pub struct WeekdayStat {
    /// 0 = Monday … 6 = Sunday.
    pub weekday: u32,
    pub rounds: u32,
    pub focus_mins: u32,
}

/// Focus distribution by hour of day across all history.
#[derive(Debug, Serialize)]
pub struct HourStat {
    pub hour: u32,
    pub rounds: u32,
    pub focus_mins: u32,
}

/// One cell of the day-of-week × hour-of-day activity grid.
#[derive(Debug, Serialize)]
pub struct RhythmCell {
    /// 0 = Monday … 6 = Sunday.
    pub weekday: u32,
    pub hour: u32,
    pub rounds: u32,
    pub focus_mins: u32,
}

/// How a set of rounds splits across session lengths.
#[derive(Debug, Serialize)]
pub struct SessionLengthBucket {
    /// Inclusive lower bound in minutes.
    pub min_mins: u32,
    /// Exclusive upper bound in minutes; `None` for the open-ended top bucket.
    pub max_mins: Option<u32>,
    pub rounds: u32,
    pub focus_mins: u32,
}

/// Turns a two-point series into a readable story.
#[derive(Debug, Serialize)]
pub struct Comparison {
    pub label: String,
    pub current: f32,
    pub previous: f32,
    /// `(current - previous) / previous`; `None` when the baseline is zero.
    pub delta_pct: Option<f32>,
}

/// Consecutive-day activity, both current and the all-time best.
#[derive(Debug, Serialize)]
pub struct StreakSummary {
    pub current: u32,
    pub longest: u32,
    /// The most recent day with any completed work round ("YYYY-MM-DD").
    pub last_active_date: Option<String>,
    /// True when a live streak has nothing logged yet today.
    pub at_risk: bool,
}

/// Rolling-average and consistency metrics.
#[derive(Debug, Serialize)]
pub struct ConsistencyStats {
    /// Days considered (trailing window, ending today).
    pub window_days: u32,
    /// Days in the window with at least one completed work round.
    pub active_days: u32,
    /// `active_days / window_days`.
    pub active_ratio: f32,
    /// Mean rounds logged per calendar day in the window (active or not).
    pub avg_rounds_per_day: f32,
    /// Mean focus minutes per calendar day in the window (active or not).
    pub avg_focus_mins_per_day: f32,
    /// Sample standard deviation of daily rounds — lower means steadier habits.
    pub stddev_rounds: f32,
    /// Days at or above 75% of the window's best day.
    pub strong_days: u32,
    /// Days with at least one round but at or below 25% of the best day.
    pub light_days: u32,
    /// Days in the window with nothing logged.
    pub rest_days: u32,
    /// Longest run of consecutive days with a session, within the window.
    pub best_streak_in_window: u32,
}

/// Personal bests and totals.
#[derive(Debug, Serialize)]
pub struct RecordStats {
    pub total_rounds: u32,
    pub total_focus_mins: u32,
    /// Distinct calendar days with at least one completed work round.
    pub tracked_days: u32,
    pub avg_session_mins: Option<f32>,
    /// Longest single completed work round, in minutes.
    pub longest_session_mins: Option<u32>,
    pub avg_rounds_per_active_day: Option<f32>,
    pub best_day_rounds: Option<TrendPoint>,
    pub best_day_focus: Option<TrendPoint>,
    pub best_week: Option<TrendPoint>,
}

/// Everything the "Better Stats" window needs, in a single IPC round-trip.
#[derive(Debug, Serialize)]
pub struct Insights {
    /// Local calendar date the payload was computed for ("YYYY-MM-DD").
    pub today: String,

    // --- Today -------------------------------------------------------------
    pub today_rounds: u32,
    pub today_focus_mins: u32,
    /// 24 entries: completed work rounds started in each local hour today.
    pub today_by_hour: Vec<u32>,
    /// Rounds left to match the trailing daily average; negative when ahead.
    pub rounds_to_beat_average: Option<i64>,
    pub ahead_of_average: bool,

    // --- Momentum ----------------------------------------------------------
    /// Rolling 7 / 28-day windows anchored on today.
    pub week: PeriodSummary,
    pub month: PeriodSummary,
    /// The last 7 and 90 calendar days (zero-filled windows).
    pub last_7_days: PeriodSummary,
    pub last_90_days: PeriodSummary,
    /// The 7 calendar days immediately before `last_7_days`.
    pub previous_7_days: PeriodSummary,
    /// Head-to-head series for the momentum cards.
    pub comparisons: Vec<Comparison>,

    // --- Trends ------------------------------------------------------------
    /// Trailing 28 days, zero-filled, oldest → newest.
    pub trend: Vec<TrendPoint>,
    /// Trailing 120 days, zero-filled, oldest → newest (default heatmap view).
    pub heatmap: Vec<TrendPoint>,
    /// Every day with recorded focus, oldest → newest. Lets the heatmap render
    /// any month or year without shipping a separate query per range.
    pub daily_all: Vec<TrendPoint>,
    /// Trailing 12 Monday-start weeks.
    pub weeks: Vec<TrendPoint>,
    /// 7-day moving average aligned index-for-index with `trend`.
    pub moving_avg_7: Vec<f32>,
    /// Minutes of focus per day for the trailing 28 days.
    pub focus_trend: Vec<TrendPoint>,
    /// The 7 calendar days ending today (alias of `trend` tail, zero-filled).
    pub daily_28: Vec<TrendPoint>,

    // --- Rhythm ------------------------------------------------------------
    /// All-time hourly focus distribution (24 entries).
    pub hour_profile: Vec<HourStat>,
    /// All-time weekday profile (7 entries, Monday first).
    pub weekday_profile: Vec<WeekdayStat>,
    /// Weekday × hour activity grid (7 × 24 = 168 cells, row-major from Monday).
    pub rhythm_grid: Vec<RhythmCell>,
    /// Start hour of the earliest session ever logged.
    pub active_from_hour: Option<u32>,
    /// Start hour of the latest session ever logged.
    pub active_to_hour: Option<u32>,

    // --- Shape of the work -------------------------------------------------
    /// Histogram of round lengths.
    pub session_buckets: Vec<SessionLengthBucket>,
    /// Share of focus time contributed by the busiest day of the week.
    pub top_weekday_share: Option<f32>,
    /// Share of focus time contributed by the busiest hour of the day.
    pub top_hour_share: Option<f32>,

    // --- Consistency / records --------------------------------------------
    pub consistency: ConsistencyStats,
    pub records: RecordStats,
    pub streak: StreakSummary,
}


/// Convenience accessors for pulling typed columns out of a `rusqlite` row.
trait RowExt {
    fn get_string(&self, idx: usize) -> String;
    fn get_i64(&self, idx: usize) -> i64;
}

impl RowExt for rusqlite::Row<'_> {
    fn get_string(&self, idx: usize) -> String {
        self.get::<_, String>(idx).unwrap_or_default()
    }

    fn get_i64(&self, idx: usize) -> i64 {
        self.get::<_, i64>(idx).unwrap_or(0)
    }
}

/// Load every completed work round as (local date, local hour, duration).
///
/// Volumes are small for a personal timer app, so the whole history is pulled
/// once and every derived metric is computed in Rust. That keeps the SQL
/// simple and makes `get_insights` a single consistent snapshot.
fn load_work_days(conn: &Connection) -> Result<Vec<WorkDay>> {
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') AS day,
                CAST(strftime('%H', datetime(started_at, 'unixepoch', 'localtime')) AS INTEGER) AS hour,
                duration_secs
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         ORDER BY started_at",
    )?;

    let rows = stmt
        .query_map([], |r| {
            Ok(WorkDay {
                date: r.get_string(0),
                hour: r.get::<_, i64>(1).unwrap_or(0).clamp(0, 23) as u32,
                duration_secs: r.get_i64(2),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(rows)
}

/// Seconds → whole minutes, rounding half up (matches `get_daily_stats`).
fn secs_to_mins(secs: i64) -> u32 {
    ((secs + 30) / 60).max(0) as u32
}

/// Collapse work rounds into per-day totals, ordered oldest → newest.
fn roll_up_by_day(days: &[WorkDay]) -> Vec<TrendPoint> {
    let mut out: Vec<TrendPoint> = Vec::new();
    for d in days {
        match out.last_mut() {
            Some(last) if last.date == d.date => {
                last.rounds += 1;
                last.focus_mins += secs_to_mins(d.duration_secs);
            }
            _ => out.push(TrendPoint {
                date: d.date.clone(),
                rounds: 1,
                focus_mins: secs_to_mins(d.duration_secs),
            }),
        }
    }
    out
}

/// Build a summary for the window `[start_day, today]` (both inclusive).
///
/// `started` supplies the Started-work-round counts for the same window so the
/// completion rate can be derived without a second query.
fn summarise(
    label: &str,
    per_day: &[TrendPoint],
    started_by_day: &[(String, u32)],
    today: &str,
    start_day: &str,
    window_days: u32,
) -> PeriodSummary {
    let in_window = |date: &str| date >= start_day && date <= today;

    let rows: Vec<&TrendPoint> = per_day.iter().filter(|p| in_window(&p.date)).collect();
    let rounds: u32 = rows.iter().map(|p| p.rounds).sum();
    let focus_mins: u32 = rows.iter().map(|p| p.focus_mins).sum();
    let active_days = rows.len() as u32;

    let started: u32 = started_by_day
        .iter()
        .filter(|(date, _)| in_window(date))
        .map(|(_, n)| *n)
        .sum();

    let best_day = rows
        .iter()
        .max_by_key(|p| (p.rounds, p.focus_mins))
        .map(|p| TrendPoint {
            date: p.date.clone(),
            rounds: p.rounds,
            focus_mins: p.focus_mins,
        });

    PeriodSummary {
        label: label.to_string(),
        days: window_days,
        rounds,
        focus_mins,
        active_days,
        avg_rounds_per_active_day: (active_days > 0)
            .then(|| rounds as f32 / active_days as f32),
        avg_focus_mins_per_active_day: (active_days > 0)
            .then(|| focus_mins as f32 / active_days as f32),
        completion_rate: (started > 0).then(|| rounds as f32 / started as f32),
        best_day,
    }
}

/// Count *started* (not necessarily completed) work rounds per local day.
fn load_started_by_day(conn: &Connection) -> Result<Vec<(String, u32)>> {
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') AS day, COUNT(*)
         FROM sessions
         WHERE round_type = 'work'
         GROUP BY day",
    )?;
    let rows = stmt
        .query_map([], |r| {
            Ok((r.get_string(0), r.get::<_, i64>(1).unwrap_or(0).max(0) as u32))
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Zero-fill the `count` calendar days ending at `today` (oldest → newest).
fn fill_recent_days(per_day: &[TrendPoint], today: &str, count: u32) -> Vec<TrendPoint> {
    let today_num = match date_to_day_num(today) {
        Some(n) => n,
        None => return Vec::new(),
    };

    let mut out = Vec::with_capacity(count as usize);
    for offset in (0..count).rev() {
        let day_num = today_num - offset as i32;
        let date = day_num_to_date(day_num);
        let found = per_day.iter().find(|p| p.date == date);
        out.push(TrendPoint {
            date: date.clone(),
            rounds: found.map(|p| p.rounds).unwrap_or(0),
            focus_mins: found.map(|p| p.focus_mins).unwrap_or(0),
        });
    }
    out
}

/// Aggregate per-day totals into Monday-start weeks, most recent `count` weeks.
fn roll_up_weeks(per_day: &[TrendPoint], today: &str, count: u32) -> Vec<TrendPoint> {
    let today_num = match date_to_day_num(today) {
        Some(n) => n,
        None => return Vec::new(),
    };

    // Weekday of the Monday that starts the current week.
    let days_since_monday = weekday_index(today_num);
    let current_monday = today_num - days_since_monday as i32;

    let mut out = Vec::with_capacity(count as usize);
    for offset in (0..count).rev() {
        let start = current_monday - (offset as i32 * 7);
        let end = start + 6;

        let mut rounds = 0u32;
        let mut focus_mins = 0u32;
        for p in per_day {
            if let Some(n) = date_to_day_num(&p.date) {
                if n >= start && n <= end {
                    rounds += p.rounds;
                    focus_mins += p.focus_mins;
                }
            }
        }

        out.push(TrendPoint {
            date: day_num_to_date(start),
            rounds,
            focus_mins,
        });
    }
    out
}

/// Inverse of `date_to_day_num`: convert its day number to "YYYY-MM-DD".
///
/// `date_to_day_num` runs exactly one day ahead of the standard epoch-day
/// numbering used by the civil-from-days algorithm below, so the input is
/// shifted by one first. Only differences between day numbers are ever
/// meaningful for interval maths, but the round-trip must be exact — see the
/// `day_num_round_trips_are_exact` test.
fn day_num_to_date(z: i32) -> String {
    // Civil-from-days algorithm (Howard Hinnant), valid for the proleptic
    // Gregorian calendar.
    let z = z - 1;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

/// Monday-based weekday index (0 = Monday … 6 = Sunday) for a day number.
///
/// The `+ 1` aligns `date_to_day_num`'s numbering with Monday-first weeks;
/// verified against known weekdays in `weekday_index_matches_known_weekdays`.
fn weekday_index(day_num: i32) -> u32 {
    (((day_num + 1) % 7 + 7) % 7) as u32
}

/// Full analytics payload for the "Better Stats" window.
pub fn get_insights(conn: &Connection) -> Result<Insights> {
    let today: String = conn.query_row("SELECT date('now', 'localtime')", [], |r| r.get(0))?;
    let today_num = date_to_day_num(&today).unwrap_or(0);

    let work_days = load_work_days(conn)?;
    let per_day = roll_up_by_day(&work_days);
    let started_by_day = load_started_by_day(conn)?;

    // --- Today -------------------------------------------------------------
    let today_row = per_day.iter().find(|p| p.date == today);
    let today_rounds = today_row.map(|p| p.rounds).unwrap_or(0);
    let today_focus_mins = today_row.map(|p| p.focus_mins).unwrap_or(0);

    let mut today_by_hour = vec![0u32; 24];
    for d in work_days.iter().filter(|d| d.date == today) {
        today_by_hour[d.hour as usize] += 1;
    }

    // --- Windows -----------------------------------------------------------
    let day_str = |offset: i32| day_num_to_date(today_num - offset);

    let week_start = day_str(6);
    let month_start = day_str(27);
    let last7_start = day_str(6);
    let prev7_start = day_str(13);
    let prev7_end = day_str(7);

    let week = summarise(
        "week",
        &per_day,
        &started_by_day,
        &today,
        &week_start,
        7,
    );
    let month = summarise(
        "month",
        &per_day,
        &started_by_day,
        &today,
        &month_start,
        28,
    );
    let last_7_days = summarise(
        "last7",
        &per_day,
        &started_by_day,
        &today,
        &last7_start,
        7,
    );

    // Previous 7 days: [prev7_start, prev7_end].
    let prev7 = {
        let started: u32 = started_by_day
            .iter()
            .filter(|(date, _)| date.as_str() >= prev7_start.as_str() && date.as_str() <= prev7_end.as_str())
            .map(|(_, n)| *n)
            .sum();
        let rows: Vec<&TrendPoint> = per_day
            .iter()
            .filter(|p| p.date.as_str() >= prev7_start.as_str() && p.date.as_str() <= prev7_end.as_str())
            .collect();
        let rounds: u32 = rows.iter().map(|p| p.rounds).sum();
        let focus_mins: u32 = rows.iter().map(|p| p.focus_mins).sum();
        let active_days = rows.len() as u32;
        PeriodSummary {
            label: "prev7".to_string(),
            days: 7,
            rounds,
            focus_mins,
            active_days,
            avg_rounds_per_active_day: (active_days > 0).then(|| rounds as f32 / active_days as f32),
            avg_focus_mins_per_active_day: (active_days > 0)
                .then(|| focus_mins as f32 / active_days as f32),
            completion_rate: (started > 0).then(|| rounds as f32 / started as f32),
            best_day: rows
                .iter()
                .max_by_key(|p| (p.rounds, p.focus_mins))
                .map(|p| TrendPoint {
                    date: p.date.clone(),
                    rounds: p.rounds,
                    focus_mins: p.focus_mins,
                }),
        }
    };

    // --- Momentum ----------------------------------------------------------
    let rounds_to_beat_average = week
        .avg_rounds_per_active_day
        .map(|avg| avg.round() as i64 - today_rounds as i64);
    let ahead_of_average = today_rounds as f32 > week.avg_rounds_per_active_day.unwrap_or(0.0);

    // --- Trend + weekly rollup --------------------------------------------
    let trend = fill_recent_days(&per_day, &today, 28);
    let weeks = roll_up_weeks(&per_day, &today, 12);

    // --- Streaks -----------------------------------------------------------
    let mut stmt = conn.prepare(
        "SELECT date(started_at, 'unixepoch', 'localtime') AS day
         FROM sessions
         WHERE round_type = 'work' AND completed = 1
         GROUP BY day
         ORDER BY day",
    )?;
    let days: Vec<String> = stmt
        .query_map([], |r| r.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    let streak = compute_streak(&days, &today);
    let streak_at_risk = streak.current > 0
        && today_rounds == 0
        && days.last().map(|d| d.as_str() != today).unwrap_or(false);

    // --- Profiles ----------------------------------------------------------
    let mut hour_totals = vec![(0u32, 0i64); 24];
    for d in &work_days {
        hour_totals[d.hour as usize].0 += 1;
        hour_totals[d.hour as usize].1 += d.duration_secs;
    }
    let hour_profile: Vec<HourStat> = hour_totals
        .iter()
        .enumerate()
        .map(|(hour, (rounds, secs))| HourStat {
            hour: hour as u32,
            rounds: *rounds,
            focus_mins: secs_to_mins(*secs),
        })
        .collect();

    let mut weekday_totals = vec![(0u32, 0i64); 7];
    for d in &work_days {
        if let Some(num) = date_to_day_num(&d.date) {
            let idx = weekday_index(num) as usize;
            weekday_totals[idx].0 += 1;
            weekday_totals[idx].1 += d.duration_secs;
        }
    }
    let weekday_profile: Vec<WeekdayStat> = weekday_totals
        .iter()
        .enumerate()
        .map(|(weekday, (rounds, secs))| WeekdayStat {
            weekday: weekday as u32,
            rounds: *rounds,
            focus_mins: secs_to_mins(*secs),
        })
        .collect();

    // --- Long-range windows ------------------------------------------------
    let last_90_days = summarise(
        "last90",
        &per_day,
        &started_by_day,
        &today,
        &day_str(89),
        90,
    );

    // --- Series ------------------------------------------------------------
    let heatmap = fill_recent_days(&per_day, &today, 120);
    let moving_avg_7 = moving_average(&trend, 7);
    let focus_trend = trend.clone();
    // --- Momentum comparisons ---------------------------------------------
    let comparisons = vec![
        Comparison {
            label: "rounds_week".into(),
            current: last_7_days.rounds as f32,
            previous: prev7.rounds as f32,
            delta_pct: pct_change(last_7_days.rounds as f32, prev7.rounds as f32),
        },
        Comparison {
            label: "focus_week".into(),
            current: last_7_days.focus_mins as f32,
            previous: prev7.focus_mins as f32,
            delta_pct: pct_change(last_7_days.focus_mins as f32, prev7.focus_mins as f32),
        },
        Comparison {
            label: "active_days_week".into(),
            current: last_7_days.active_days as f32,
            previous: prev7.active_days as f32,
            delta_pct: pct_change(last_7_days.active_days as f32, prev7.active_days as f32),
        },
        Comparison {
            label: "avg_per_active_day".into(),
            current: last_7_days.avg_rounds_per_active_day.unwrap_or(0.0),
            previous: prev7.avg_rounds_per_active_day.unwrap_or(0.0),
            delta_pct: pct_change(
                last_7_days.avg_rounds_per_active_day.unwrap_or(0.0),
                prev7.avg_rounds_per_active_day.unwrap_or(0.0),
            ),
        },
    ];

    // --- Rhythm grid (weekday × hour) --------------------------------------
    let mut grid = vec![(0u32, 0i64); 7 * 24];
    for d in &work_days {
        if let Some(num) = date_to_day_num(&d.date) {
            let wd = weekday_index(num) as usize;
            let idx = wd * 24 + d.hour as usize;
            grid[idx].0 += 1;
            grid[idx].1 += d.duration_secs;
        }
    }
    let rhythm_grid: Vec<RhythmCell> = grid
        .iter()
        .enumerate()
        .map(|(i, (rounds, secs))| RhythmCell {
            weekday: (i / 24) as u32,
            hour: (i % 24) as u32,
            rounds: *rounds,
            focus_mins: secs_to_mins(*secs),
        })
        .collect();

    // --- Active span -------------------------------------------------------
    let active_from_hour = work_days.iter().map(|d| d.hour).min();
    let active_to_hour = work_days.iter().map(|d| d.hour).max();

    // --- Session-length histogram -----------------------------------------
    let session_buckets = build_session_buckets(&work_days);

    // --- Concentration -----------------------------------------------------
    let top_weekday_share = {
        let total: u32 = weekday_profile.iter().map(|w| w.focus_mins).sum();
        let best = weekday_profile.iter().map(|w| w.focus_mins).max().unwrap_or(0);
        (total > 0).then(|| best as f32 / total as f32)
    };
    let top_hour_share = {
        let total: u32 = hour_profile.iter().map(|h| h.focus_mins).sum();
        let best = hour_profile.iter().map(|h| h.focus_mins).max().unwrap_or(0);
        (total > 0).then(|| best as f32 / total as f32)
    };

    // --- Consistency -------------------------------------------------------
    let consistency = build_consistency(&per_day, &today, 30);

    // --- Records -----------------------------------------------------------
    let total_rounds: u32 = per_day.iter().map(|p| p.rounds).sum();
    let total_focus_mins: u32 = per_day.iter().map(|p| p.focus_mins).sum();
    let total_tracked_days = per_day.len() as u32;
    let avg_session_mins = (total_rounds > 0).then(|| total_focus_mins as f32 / total_rounds as f32);
    let best_day_rounds = per_day
        .iter()
        .max_by_key(|p| (p.rounds, p.focus_mins))
        .map(|p| p.clone());
    let best_day_focus = per_day
        .iter()
        .max_by_key(|p| (p.focus_mins, p.rounds))
        .map(|p| p.clone());
    let best_week = weeks.iter().max_by_key(|p| p.focus_mins).map(|p| p.clone());
    let longest_session_mins = work_days
        .iter()
        .map(|d| secs_to_mins(d.duration_secs))
        .max();

    let records = RecordStats {
        total_rounds,
        total_focus_mins,
        tracked_days: total_tracked_days,
        avg_session_mins,
        longest_session_mins,
        avg_rounds_per_active_day: (total_tracked_days > 0)
            .then(|| total_rounds as f32 / total_tracked_days as f32),
        best_day_rounds,
        best_day_focus,
        best_week,
    };

    let streak = StreakSummary {
        current: streak.current,
        longest: streak.longest,
        last_active_date: per_day.last().map(|p| p.date.clone()),
        at_risk: streak_at_risk,
    };

    Ok(Insights {
        today,
        today_rounds,
        today_focus_mins,
        today_by_hour,
        rounds_to_beat_average,
        ahead_of_average,
        week,
        month,
        last_7_days,
        last_90_days,
        previous_7_days: prev7,
        comparisons,
        trend: trend.clone(),
        heatmap,
        daily_all: per_day,
        weeks,
        moving_avg_7,
        focus_trend,
        daily_28: trend,
        hour_profile,
        weekday_profile,
        rhythm_grid,
        active_from_hour,
        active_to_hour,
        session_buckets,
        top_weekday_share,
        top_hour_share,
        consistency,
        records,
        streak,
    })
}

// ---------------------------------------------------------------------------
// Insights helpers
// ---------------------------------------------------------------------------

/// `(current - previous) / previous`, or `None` when the baseline is zero.
fn pct_change(current: f32, previous: f32) -> Option<f32> {
    (previous > 0.0).then(|| (current - previous) / previous)
}

/// Mean of the `window` values ending at each index, aligned index-for-index
/// with the input. Values before a full window has accumulated average over
/// whatever is available, so the series has no leading gap.
fn moving_average(points: &[TrendPoint], window: usize) -> Vec<f32> {
    if points.is_empty() {
        return Vec::new();
    }
    let window = window.max(1);
    points
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let start = (i + 1).saturating_sub(window);
            let slice = &points[start..=i];
            let sum: u32 = slice.iter().map(|p| p.rounds).sum();
            sum as f32 / slice.len() as f32
        })
        .collect()
}

/// Histogram of completed round lengths in minute buckets.
fn build_session_buckets(days: &[WorkDay]) -> Vec<SessionLengthBucket> {
    // (lower, upper) in minutes; the last bucket is open-ended.
    const BOUNDS: [(u32, Option<u32>); 5] = [
        (0, Some(15)),
        (15, Some(25)),
        (25, Some(40)),
        (40, Some(60)),
        (60, None),
    ];

    let mut buckets: Vec<SessionLengthBucket> = BOUNDS
        .iter()
        .map(|(min, max)| SessionLengthBucket {
            min_mins: *min,
            max_mins: *max,
            rounds: 0,
            focus_mins: 0,
        })
        .collect();

    for d in days {
        let mins = secs_to_mins(d.duration_secs);
        let bucket = buckets.iter_mut().find(|b| match b.max_mins {
            Some(max) => mins >= b.min_mins && mins < max,
            None => mins >= b.min_mins,
        });
        if let Some(b) = bucket {
            b.rounds += 1;
            b.focus_mins += mins;
        }
    }

    buckets
}

/// Habit metrics over the trailing `window` calendar days.
fn build_consistency(per_day: &[TrendPoint], today: &str, window: u32) -> ConsistencyStats {
    let filled = fill_recent_days(per_day, today, window);
    if filled.is_empty() {
        return ConsistencyStats {
            window_days: 0,
            active_days: 0,
            active_ratio: 0.0,
            avg_rounds_per_day: 0.0,
            avg_focus_mins_per_day: 0.0,
            stddev_rounds: 0.0,
            strong_days: 0,
            light_days: 0,
            rest_days: 0,
            best_streak_in_window: 0,
        };
    }

    let days = filled.len() as u32;
    let active_days = filled.iter().filter(|p| p.rounds > 0).count() as u32;
    let total_rounds: u32 = filled.iter().map(|p| p.rounds).sum();
    let total_focus: u32 = filled.iter().map(|p| p.focus_mins).sum();
    let mean = total_rounds as f32 / days as f32;

    // Sample standard deviation of daily rounds.
    let variance = filled
        .iter()
        .map(|p| {
            let d = p.rounds as f32 - mean;
            d * d
        })
        .sum::<f32>()
        / days as f32;
    let stddev_rounds = variance.sqrt();

    let best = filled.iter().map(|p| p.rounds).max().unwrap_or(0);
    let strong_cut = (best as f32 * 0.75).ceil() as u32;
    let light_cut = (best as f32 * 0.25).floor() as u32;

    let strong_days = filled.iter().filter(|p| p.rounds >= strong_cut && best > 0).count() as u32;
    let light_days = filled
        .iter()
        .filter(|p| p.rounds > 0 && p.rounds <= light_cut)
        .count() as u32;
    let rest_days = filled.iter().filter(|p| p.rounds == 0).count() as u32;

    let mut best_streak_in_window = 0u32;
    let mut run = 0u32;
    for p in &filled {
        if p.rounds > 0 {
            run += 1;
            best_streak_in_window = best_streak_in_window.max(run);
        } else {
            run = 0;
        }
    }

    ConsistencyStats {
        window_days: days,
        active_days,
        active_ratio: active_days as f32 / days as f32,
        avg_rounds_per_day: mean,
        avg_focus_mins_per_day: total_focus as f32 / days as f32,
        stddev_rounds,
        strong_days,
        light_days,
        rest_days,
        best_streak_in_window,
    }
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

    // -----------------------------------------------------------------------
    // Insights
    // -----------------------------------------------------------------------

    #[test]
    fn day_num_and_date_round_trip() {
        for date in [
            "1970-01-01",
            "2000-02-29",
            "2024-03-15",
            "2026-12-31",
            "2027-01-01",
        ] {
            let num = date_to_day_num(date).expect("parse");
            assert_eq!(day_num_to_date(num), date, "round trip failed for {date}");
        }
    }

    #[test]
    fn day_num_advances_by_one_per_day() {
        let a = date_to_day_num("2024-02-28").unwrap();
        let b = date_to_day_num("2024-02-29").unwrap(); // leap day
        let c = date_to_day_num("2024-03-01").unwrap();
        assert_eq!(b - a, 1);
        assert_eq!(c - b, 1);
    }

    #[test]
    fn weekday_index_matches_known_weekdays() {
        // (date, Monday-based index) — 0 = Monday … 6 = Sunday.
        let cases = [
            ("2024-03-11", 0), // Monday
            ("2024-03-12", 1),
            ("2024-03-13", 2), // Wednesday
            ("2024-03-04", 0), // Monday
            ("2024-03-10", 6), // Sunday
            ("2000-02-29", 1), // Tuesday
            ("1970-01-01", 3), // Thursday
            ("2026-12-31", 3), // Thursday
        ];
        for (date, want) in cases {
            let n = date_to_day_num(date).expect("parse");
            assert_eq!(
                weekday_index(n),
                want,
                "wrong weekday index for {date} (expected {want})"
            );
        }
    }

    #[test]
    fn day_num_round_trips_are_exact() {
        // Guards the one-day shift in day_num_to_date. These exact pairs were
        // verified against an independent calendar implementation.
        for date in [
            "1970-01-01",
            "1970-01-02",
            "1969-12-31",
            "2000-02-29",
            "2024-02-28",
            "2024-02-29",
            "2024-03-01",
            "2024-03-15",
            "2026-03-09",
            "2026-09-25",
            "2026-12-31",
            "2027-01-01",
            "2100-03-01",
            "2038-01-19",
        ] {
            let n = date_to_day_num(date).expect("parse");
            assert_eq!(
                day_num_to_date(n),
                date,
                "round trip failed for {date}"
            );
        }
    }

    #[test]
    fn date_helpers_match_known_calendar_dates() {
        // Absolute day numbers are an implementation detail, but these values
        // pin the numbering so an accidental change is caught immediately.
        assert_eq!(date_to_day_num("1970-01-01"), Some(719_469));
        assert_eq!(date_to_day_num("2026-09-25"), Some(740_190));
        // Consecutive days differ by exactly one.
        let a = date_to_day_num("2026-09-24").unwrap();
        let b = date_to_day_num("2026-09-25").unwrap();
        assert_eq!(b - a, 1);
        // And the inverse lands on the right date.
        assert_eq!(day_num_to_date(740_190), "2026-09-25");
        assert_eq!(day_num_to_date(740_189), "2026-09-24");
    }

    #[test]
    fn roll_up_weeks_starts_each_bucket_on_a_monday() {
        // Regression guard for the weekday constant: every bucket start must be
        // a Monday, and the last bucket must be the week containing `today`.
        let weeks = roll_up_weeks(&[], "2024-03-13", 4); // Wednesday
        assert_eq!(weeks.len(), 4);
        assert_eq!(weeks.last().unwrap().date, "2024-03-11");
        for w in &weeks {
            let n = date_to_day_num(&w.date).expect("parse");
            assert_eq!(weekday_index(n), 0, "{} is not a Monday", w.date);
        }
    }

    #[test]
    fn fill_recent_days_zero_fills_and_orders_oldest_first() {
        let per_day = vec![
            TrendPoint { date: "2024-03-10".into(), rounds: 2, focus_mins: 50 },
            TrendPoint { date: "2024-03-13".into(), rounds: 4, focus_mins: 100 },
        ];
        let filled = fill_recent_days(&per_day, "2024-03-13", 5);

        assert_eq!(filled.len(), 5);
        assert_eq!(filled.first().unwrap().date, "2024-03-09");
        assert_eq!(filled.last().unwrap().date, "2024-03-13");

        assert_eq!(filled[1].date, "2024-03-10");
        assert_eq!(filled[1].rounds, 2, "existing day must be preserved");
        assert_eq!(filled[3].date, "2024-03-12");
        assert_eq!(filled[3].rounds, 0, "missing days are zero-filled");
        assert_eq!(filled[4].rounds, 4);
    }

    #[test]
    fn roll_up_weeks_buckets_by_monday_start() {
        // 2024-03-13 is a Wednesday → its week starts Monday 2024-03-11.
        let per_day = vec![
            TrendPoint { date: "2024-03-11".into(), rounds: 1, focus_mins: 25 },
            TrendPoint { date: "2024-03-13".into(), rounds: 2, focus_mins: 50 },
            TrendPoint { date: "2024-03-04".into(), rounds: 5, focus_mins: 125 },
        ];
        let weeks = roll_up_weeks(&per_day, "2024-03-13", 2);

        assert_eq!(weeks.len(), 2);
        assert_eq!(weeks[0].date, "2024-03-04", "previous week starts on a Monday");
        assert_eq!(weeks[0].rounds, 5);
        assert_eq!(weeks[1].date, "2024-03-11", "current week starts on a Monday");
        assert_eq!(weeks[1].rounds, 3, "Monday + Wednesday roll into the same week");
        assert_eq!(weeks[1].focus_mins, 75);
    }

    #[test]
    fn roll_up_by_day_groups_consecutive_days() {
        let days = vec![
            WorkDay { date: "2024-03-01".into(), hour: 9, duration_secs: 1500 },
            WorkDay { date: "2024-03-01".into(), hour: 10, duration_secs: 1500 },
            WorkDay { date: "2024-03-02".into(), hour: 11, duration_secs: 300 },
        ];
        let rolled = roll_up_by_day(&days);

        assert_eq!(rolled.len(), 2);
        assert_eq!(rolled[0].date, "2024-03-01");
        assert_eq!(rolled[0].rounds, 2);
        assert_eq!(rolled[0].focus_mins, 50);
        assert_eq!(rolled[1].date, "2024-03-02");
        assert_eq!(rolled[1].rounds, 1);
        assert_eq!(rolled[1].focus_mins, 5);
    }

    #[test]
    fn summarise_computes_averages_and_best_day() {
        let per_day = vec![
            TrendPoint { date: "2024-03-11".into(), rounds: 2, focus_mins: 50 },
            TrendPoint { date: "2024-03-13".into(), rounds: 6, focus_mins: 150 },
        ];
        // Two work rounds started on the 11th, eight on the 13th.
        let started = vec![("2024-03-11".to_string(), 2u32), ("2024-03-13".to_string(), 8)];

        let s = summarise("week", &per_day, &started, "2024-03-13", "2024-03-07", 7);

        assert_eq!(s.rounds, 8);
        assert_eq!(s.focus_mins, 200);
        assert_eq!(s.active_days, 2);
        assert_eq!(s.avg_rounds_per_active_day, Some(4.0));
        assert_eq!(s.avg_focus_mins_per_active_day, Some(100.0));
        assert_eq!(s.completion_rate, Some(8.0 / 10.0));
        assert_eq!(s.best_day.as_ref().unwrap().date, "2024-03-13");
        assert_eq!(s.best_day.as_ref().unwrap().rounds, 6);
    }

    #[test]
    fn summarise_handles_empty_window() {
        let s = summarise("week", &[], &[], "2024-03-13", "2024-03-07", 7);
        assert_eq!(s.rounds, 0);
        assert!(s.avg_rounds_per_active_day.is_none());
        assert!(s.completion_rate.is_none());
        assert!(s.best_day.is_none());
    }

    #[test]
    fn get_insights_empty_db_is_well_formed() {
        let conn = setup();
        let i = get_insights(&conn).unwrap();

        assert_eq!(i.today_rounds, 0);
        assert_eq!(i.today_focus_mins, 0);
        assert_eq!(i.today_by_hour.len(), 24);
        assert_eq!(i.hour_profile.len(), 24);
        assert_eq!(i.weekday_profile.len(), 7);
        assert_eq!(i.trend.len(), 28);
        assert_eq!(i.weeks.len(), 12);
        assert_eq!(i.heatmap.len(), 120);
        assert_eq!(i.moving_avg_7.len(), 28);
        assert_eq!(i.rhythm_grid.len(), 7 * 24);
        assert_eq!(i.session_buckets.len(), 5);
        assert_eq!(i.comparisons.len(), 4);
        assert_eq!(i.records.total_rounds, 0);
        assert_eq!(i.streak.current, 0);
        assert!(!i.streak.at_risk, "no streak means nothing to lose");
        assert!(i.records.avg_session_mins.is_none());
        assert!(i.rounds_to_beat_average.is_none());
        assert!(!i.ahead_of_average);
        assert!(i.active_from_hour.is_none());
        assert!(i.top_hour_share.is_none());
        assert_eq!(i.consistency.window_days, 30);
        assert_eq!(i.consistency.active_days, 0);
        assert_eq!(i.consistency.rest_days, 30, "an empty month is all rest days");
        // Every series must be zero-filled and ordered oldest → newest.
        assert!(i.trend.iter().all(|p| p.rounds == 0));
        assert!(i.heatmap.iter().all(|p| p.rounds == 0));
        assert_eq!(i.trend.last().unwrap().date, i.today);
        assert_eq!(i.heatmap.last().unwrap().date, i.today);
        assert!(i.moving_avg_7.iter().all(|v| *v == 0.0));
    }

    #[test]
    fn get_insights_rolls_up_backdated_sessions() {
        let conn = setup();
        let today: String = conn
            .query_row("SELECT date('now', 'localtime')", [], |r| r.get(0))
            .unwrap();

        // Two completed work rounds today, one skipped, one break.
        let a = insert_session(&conn, "work", 1500).unwrap();
        complete_session(&conn, a, true).unwrap();
        let b = insert_session(&conn, "work", 1500).unwrap();
        complete_session(&conn, b, true).unwrap();
        let c = insert_session(&conn, "work", 1500).unwrap();
        complete_session(&conn, c, false).unwrap();
        let d = insert_session(&conn, "short-break", 300).unwrap();
        complete_session(&conn, d, true).unwrap();

        let i = get_insights(&conn).unwrap();

        assert_eq!(i.today_rounds, 2, "only completed work rounds count");
        assert_eq!(i.today_focus_mins, 50);
        assert_eq!(i.records.total_rounds, 2);
        assert_eq!(i.records.avg_session_mins, Some(25.0));
        assert_eq!(i.records.longest_session_mins, Some(25));
        assert_eq!(i.records.tracked_days, 1);
        assert_eq!(i.streak.current, 1, "today counts toward the streak");
        assert!(!i.streak.at_risk, "today already has a session");
        assert_eq!(i.streak.last_active_date.as_deref(), Some(i.today.as_str()));
        assert_eq!(i.week.rounds, 2);
        assert_eq!(i.week.completion_rate, Some(2.0 / 3.0));
        assert_eq!(i.trend.last().unwrap().date, today);
        assert_eq!(i.trend.last().unwrap().rounds, 2);
        assert_eq!(i.weekday_profile.iter().map(|w| w.rounds).sum::<u32>(), 2);
        assert_eq!(i.hour_profile.iter().map(|h| h.rounds).sum::<u32>(), 2);
        assert_eq!(i.rhythm_grid.iter().map(|c| c.rounds).sum::<u32>(), 2);
        assert_eq!(
            i.session_buckets.iter().map(|b| b.rounds).sum::<u32>(),
            2,
            "every round lands in exactly one bucket"
        );
        // Both rounds are 25 min → the 25–40 bucket.
        assert_eq!(i.session_buckets[2].rounds, 2);
        // Best day is today for both metrics.
        assert_eq!(i.records.best_day_rounds.as_ref().unwrap().date, today);
        assert_eq!(i.records.best_day_focus.as_ref().unwrap().date, today);
        // Consistency over the trailing 30 days: one active day.
        assert_eq!(i.consistency.active_days, 1);
        assert_eq!(i.consistency.rest_days, 29);
        // 2 rounds on a single day makes a 1-day best streak inside the window.
        assert_eq!(i.consistency.best_streak_in_window, 1);
        assert!(i.active_from_hour.is_some());
        assert!(i.active_to_hour.is_some());
        assert!(i.moving_avg_7.last().unwrap() > &0.0);
        assert_eq!(
            i.moving_avg_7.last().copied(),
            Some(2.0),
            "a single day of 2 rounds averages to 2.0"
        );
        // Today beats the trailing average once there is history.
        assert_eq!(i.comparisons.len(), 4);
    }

    #[test]
    fn get_insights_marks_a_streak_at_risk() {
        let conn = setup();
        // One completed work round yesterday, none today.
        conn.execute(
            "INSERT INTO sessions (started_at, ended_at, round_type, duration_secs, completed)
             VALUES (strftime('%s', 'now', 'localtime', 'start of day', '-1 day') + 36000,
                     strftime('%s', 'now', 'localtime', 'start of day', '-1 day') + 37500,
                     'work', 1500, 1)",
            [],
        )
        .unwrap();

        let i = get_insights(&conn).unwrap();
        assert_eq!(i.today_rounds, 0);
        assert_eq!(i.streak.current, 1, "yesterday keeps the streak alive");
        assert!(i.streak.at_risk, "a live streak with nothing logged today is at risk");
    }

    // -----------------------------------------------------------------------
    // Insights helpers
    // -----------------------------------------------------------------------

    #[test]
    fn pct_change_handles_zero_baseline() {
        assert_eq!(pct_change(10.0, 0.0), None, "dividing by a zero baseline is undefined");
        assert_eq!(pct_change(10.0, 10.0), Some(0.0));
        assert_eq!(pct_change(15.0, 10.0), Some(0.5));
        assert_eq!(pct_change(5.0, 10.0), Some(-0.5));
    }

    #[test]
    fn moving_average_ramps_then_tracks_the_window() {
        let points: Vec<TrendPoint> = [4u32, 0, 2, 0, 8, 2, 0, 10]
            .iter()
            .enumerate()
            .map(|(i, r)| TrendPoint {
                date: format!("2024-03-{:02}", i + 1),
                rounds: *r,
                focus_mins: r * 25,
            })
            .collect();

        let avg = moving_average(&points, 7);
        assert_eq!(avg.len(), points.len(), "aligned index-for-index with the input");
        // Partial window: the first value is just the first day.
        assert_eq!(avg[0], 4.0);
        // Three days in: (4 + 0 + 2) / 3 = 2.0
        assert_eq!(avg[2], 2.0);
        // Fully saturated 7-day window at index 6: (4+0+2+0+8+2+0)/7 = 2.2857…
        assert!((avg[6] - 16.0 / 7.0).abs() < 1e-5);
        // Index 7 rolls the window forward: (0+2+0+8+2+0+10)/7 = 22/7
        assert!((avg[7] - 22.0 / 7.0).abs() < 1e-5);
    }

    #[test]
    fn moving_average_of_empty_series_is_empty() {
        assert!(moving_average(&[], 7).is_empty());
    }

    #[test]
    fn session_buckets_partition_every_round() {
        let days = vec![
            WorkDay { date: "2024-03-01".into(), hour: 9, duration_secs: 5 * 60 },   // 5m
            WorkDay { date: "2024-03-01".into(), hour: 10, duration_secs: 15 * 60 }, // boundary → 15-25
            WorkDay { date: "2024-03-01".into(), hour: 11, duration_secs: 25 * 60 }, // boundary → 25-40
            WorkDay { date: "2024-03-01".into(), hour: 12, duration_secs: 60 * 60 }, // open top bucket
            WorkDay { date: "2024-03-01".into(), hour: 13, duration_secs: 45 * 60 }, // 40-60
        ];
        let buckets = build_session_buckets(&days);

        assert_eq!(buckets.iter().map(|b| b.rounds).sum::<u32>(), 5);
        assert_eq!(buckets[0].rounds, 1, "<15m holds the 5m round");
        assert_eq!(buckets[1].rounds, 1, "15m falls into the 15-25 bucket");
        assert_eq!(buckets[2].rounds, 1, "25m falls into the 25-40 bucket");
        assert_eq!(buckets[3].rounds, 1, "45m falls into the 40-60 bucket");
        assert_eq!(buckets[4].rounds, 1, "60m falls into the open bucket");
        assert_eq!(buckets[4].max_mins, None, "the top bucket is open-ended");
        // Buckets are contiguous and ordered.
        for pair in buckets.windows(2) {
            assert_eq!(pair[0].max_mins, Some(pair[1].min_mins));
        }
    }

    #[test]
    fn consistency_measures_habit_strength() {
        // A deliberate pattern inside a 10-day window:
        // 2 days at 8 rounds (strong), 1 day at 1 round (light), 4 rest days.
        let mut per_day = Vec::new();
        for (date, rounds) in [
            ("2024-03-01", 8u32),
            ("2024-03-02", 8),
            ("2024-03-05", 1),
            ("2024-03-08", 4),
            ("2024-03-09", 6),
            ("2024-03-10", 2),
        ] {
            per_day.push(TrendPoint { date: date.into(), rounds, focus_mins: rounds * 25 });
        }

        let c = build_consistency(&per_day, "2024-03-10", 10);

        assert_eq!(c.window_days, 10);
        assert_eq!(c.active_days, 6);
        assert!((c.active_ratio - 0.6).abs() < 1e-6);
        assert_eq!(c.rest_days, 4);
        // Best day is 8 → strong cut is ceil(6) = 6, light cut is floor(2) = 2.
        assert_eq!(c.strong_days, 3, "8, 8 and 6 are strong days");
        assert_eq!(c.light_days, 1, "only the 1-round day counts as light");
        // The window has no consecutive active days.
        assert_eq!(c.best_streak_in_window, 1);
        // Mean over all 10 calendar days, not just active ones.
        assert!((c.avg_rounds_per_day - 2.9).abs() < 1e-5);
        assert!(c.stddev_rounds > 0.0, "an uneven pattern has spread");
    }

    #[test]
    fn consistency_of_empty_history_is_zeroed() {
        let c = build_consistency(&[], "2024-03-10", 30);
        assert_eq!(c.window_days, 30);
        assert_eq!(c.active_days, 0);
        assert_eq!(c.rest_days, 30);
        assert_eq!(c.stddev_rounds, 0.0);
        assert_eq!(c.best_streak_in_window, 0);
    }

    #[test]
    fn consistency_detects_consecutive_runs() {
        let per_day: Vec<TrendPoint> = ["2024-03-01", "2024-03-02", "2024-03-03", "2024-03-07"]
            .iter()
            .map(|d| TrendPoint { date: (*d).into(), rounds: 3, focus_mins: 75 })
            .collect();
        let c = build_consistency(&per_day, "2024-03-07", 30);
        assert_eq!(c.best_streak_in_window, 3, "the 1st–3rd form a 3-day run");
        assert_eq!(c.active_days, 4);
    }
}
