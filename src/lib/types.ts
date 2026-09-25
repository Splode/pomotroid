// Shared TypeScript types mirroring Rust structs (must stay in sync with Rust serde output).

export type RoundType = 'work' | 'short-break' | 'long-break';

/** Mirrors Rust `TimerSnapshot` — emitted via timer:tick / timer:round-change events
 *  and returned by the `timer_get_state` IPC command. */
export interface TimerState {
  round_type: RoundType;
  previous_round_type: string; // round type before this one; "" on first round
  elapsed_secs: number;
  total_secs: number;
  is_running: boolean;
  is_paused: boolean;
  work_round_number: number; // current work round (1-based)
  work_rounds_total: number; // total work rounds before long break
  session_work_count: number; // monotonic focus round count since last reset
  // --- Incremental focus mode ---
  incremental_work_enabled: boolean; // escalating work-duration ladder is active
  base_work_secs: number; // un-escalated work duration from settings
  work_increment_secs: number; // seconds added per completed work round
  work_max_secs: number; // escalation ceiling
  increment_steps: number; // increments currently applied
  at_increment_cap: boolean; // work duration has reached the ceiling
}

/** Mirrors Rust `Settings` struct returned by `settings_get`. */
export interface Settings {
  time_work_secs: number;
  time_short_break_secs: number;
  time_long_break_secs: number;
  /** Incremental focus mode: each work round gets longer than the last. */
  incremental_work_enabled: boolean;
  /** Seconds added to the work duration after each completed work round. */
  time_work_increment_secs: number;
  /** Ceiling for the escalated work duration, in seconds. */
  time_work_max_secs: number;
  /** Restart the incremental ladder when a long break begins. */
  incremental_reset_on_long_break: boolean;
  /** Restart the incremental ladder when the local calendar day changes. */
  incremental_reset_daily: boolean;
  long_break_interval: number;
  short_breaks_enabled: boolean;
  long_breaks_enabled: boolean;
  auto_start_work: boolean;
  auto_start_break: boolean;
  tray_icon_enabled: boolean;
  min_to_tray: boolean;
  min_to_tray_on_close: boolean;
  notifications_enabled: boolean;
  always_on_top: boolean;
  break_always_on_top: boolean;
  volume: number; // 0.0–1.0
  tick_sounds_during_work: boolean;
  tick_sounds_during_break: boolean;
  shortcut_toggle: string;
  shortcut_reset: string;
  shortcut_skip: string;
  shortcut_restart: string;
  websocket_enabled: boolean;
  websocket_port: number;
  theme_mode: string; // 'auto' | 'light' | 'dark'
  theme_light: string;
  theme_dark: string;
  dial_countdown: boolean;
  language: string; // 'auto' | 'en' | 'es' | 'fr' | 'de' | 'ja'
  verbose_logging: boolean;
  check_for_updates: boolean;
  global_shortcuts_enabled: boolean;
  local_shortcut_toggle: string;
  local_shortcut_reset: string;
  local_shortcut_skip: string;
  local_shortcut_volume_down: string;
  local_shortcut_volume_up: string;
  local_shortcut_mute: string;
  local_shortcut_fullscreen: string;
}

/** Returned by `check_update` — describes an available update. */
export interface UpdateInfo {
  version: string;
  body: string | null;
  date: string | null;
}

/** Mirrors Rust `CustomAudioInfo` — null means the built-in sound is active. */
export interface CustomAudioInfo {
  work_alert: string | null;
  short_break_alert: string | null;
  long_break_alert: string | null;
}

/** Mirrors Rust `Theme` struct. Color keys include the `--` CSS var prefix. */
export interface Theme {
  name: string;
  colors: Record<string, string>; // keys like "--color-background", "--color-focus-round"
  is_custom: boolean;
}

// ---------------------------------------------------------------------------
// Stats types — mirror Rust structs in commands.rs / queries.rs
// ---------------------------------------------------------------------------

export interface DailyStats {
  rounds: number;
  focus_mins: number;
  completion_rate: number | null; // null when no sessions started today
  by_hour: number[]; // 24 entries, index = hour of day
}

export interface DayStat {
  date: string; // "YYYY-MM-DD"
  rounds: number;
}

export interface HeatmapEntry {
  date: string; // "YYYY-MM-DD"
  count: number;
}

export interface StreakInfo {
  current: number;
  longest: number;
}

/** Returned by stats_get_detailed — Today + This Week + streak in one call. */
export interface DetailedStats {
  today: DailyStats;
  week: DayStat[];
  streak: StreakInfo;
}

/** Returned by stats_get_heatmap — heatmap entries + lifetime totals. */
export interface HeatmapStats {
  entries: HeatmapEntry[];
  total_rounds: number;
  total_hours: number;
  longest_streak: number;
}

// ---------------------------------------------------------------------------
// Insights types — powering the "Better Stats" window
// ---------------------------------------------------------------------------

/** One calendar day of completed work activity. */
export interface TrendPoint {
  date: string; // "YYYY-MM-DD"
  rounds: number;
  focus_mins: number;
}

/** Aggregate activity over a rolling window of days. */
export interface PeriodSummary {
  label: string;
  days: number;
  rounds: number;
  focus_mins: number;
  active_days: number;
  avg_rounds_per_active_day: number | null;
  avg_focus_mins_per_active_day: number | null;
  completion_rate: number | null; // completed / started; null when nothing started
  best_day: TrendPoint | null;
}

export interface WeekdayStat {
  weekday: number; // 0 = Monday … 6 = Sunday
  rounds: number;
  focus_mins: number;
}

export interface HourStat {
  hour: number; // 0–23
  rounds: number;
  focus_mins: number;
}

/** One cell of the day-of-week × hour-of-day activity grid. */
export interface RhythmCell {
  weekday: number; // 0 = Monday … 6 = Sunday
  hour: number; // 0–23
  rounds: number;
  focus_mins: number;
}

/** A bucket in the session-length histogram. */
export interface SessionLengthBucket {
  min_mins: number;
  max_mins: number | null; // null for the open-ended top bucket
  rounds: number;
  focus_mins: number;
}

/** A two-point series used for momentum comparisons. */
export interface Comparison {
  label: string; // 'rounds_week' | 'focus_week' | 'active_days_week' | 'avg_per_active_day'
  current: number;
  previous: number;
  delta_pct: number | null;
}

export interface StreakSummary {
  current: number;
  longest: number;
  last_active_date: string | null;
  at_risk: boolean;
}

export interface ConsistencyStats {
  window_days: number;
  active_days: number;
  active_ratio: number; // 0–1
  avg_rounds_per_day: number;
  avg_focus_mins_per_day: number;
  stddev_rounds: number;
  strong_days: number;
  light_days: number;
  rest_days: number;
  best_streak_in_window: number;
}

export interface RecordStats {
  total_rounds: number;
  total_focus_mins: number;
  tracked_days: number;
  avg_session_mins: number | null;
  longest_session_mins: number | null;
  avg_rounds_per_active_day: number | null;
  best_day_rounds: TrendPoint | null;
  best_day_focus: TrendPoint | null;
  best_week: TrendPoint | null;
}

/** Returned by stats_get_insights — the full "Better Stats" payload. */
export interface Insights {
  today: string; // "YYYY-MM-DD"

  // Today
  today_rounds: number;
  today_focus_mins: number;
  today_by_hour: number[]; // 24 entries
  rounds_to_beat_average: number | null;
  ahead_of_average: boolean;

  // Momentum
  week: PeriodSummary;
  month: PeriodSummary;
  last_7_days: PeriodSummary;
  last_90_days: PeriodSummary;
  previous_7_days: PeriodSummary;
  comparisons: Comparison[];

  // Trends
  trend: TrendPoint[]; // trailing 28 days, zero-filled, oldest → newest
  heatmap: TrendPoint[]; // trailing 120 days, zero-filled
  daily_all: TrendPoint[]; // every recorded day, oldest → newest
  weeks: TrendPoint[]; // trailing 12 Monday-start weeks
  moving_avg_7: number[]; // 7-day moving average, aligned with `trend`
  focus_trend: TrendPoint[]; // minutes per day for the trailing 28 days
  daily_28: TrendPoint[];

  // Rhythm
  hour_profile: HourStat[]; // 24 entries, all time
  weekday_profile: WeekdayStat[]; // 7 entries, Monday first
  rhythm_grid: RhythmCell[]; // 7 × 24 = 168 cells, row-major from Monday
  active_from_hour: number | null;
  active_to_hour: number | null;

  // Shape of the work
  session_buckets: SessionLengthBucket[];
  top_weekday_share: number | null; // 0–1
  top_hour_share: number | null; // 0–1

  // Consistency + records
  consistency: ConsistencyStats;
  records: RecordStats;
  streak: StreakSummary;
}
