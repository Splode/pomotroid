// Shared TypeScript types mirroring Rust structs (must stay in sync with Rust serde output).

export type RoundType = 'work' | 'short-break' | 'long-break';


/** Mirrors Rust `TimerSnapshot` — emitted via timer:tick / timer:round-change events
 *  and returned by the `timer_get_state` IPC command. */
export interface TimerState {
  round_type: RoundType;
  previous_round_type: string;  // round type before this one; "" on first round
  elapsed_secs: number;
  total_secs: number;
  is_running: boolean;
  is_paused: boolean;
  work_round_number: number;   // current work round (1-based)
  work_rounds_total: number;   // total work rounds before long break
  session_work_count: number;  // monotonic focus round count since last reset
}

/** Mirrors Rust `Settings` struct returned by `settings_get`. */
export interface Settings {
  time_work_secs: number;
  time_short_break_secs: number;
  time_long_break_secs: number;
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
  volume: number;             // 0.0–1.0
  tick_sounds_during_work: boolean;
  tick_sounds_during_break: boolean;
  shortcut_toggle: string;
  shortcut_reset: string;
  shortcut_skip: string;
  shortcut_restart: string;
  websocket_enabled: boolean;
  websocket_port: number;
  theme_mode: string;   // 'auto' | 'light' | 'dark'
  theme_light: string;
  theme_dark: string;
  dial_countdown: boolean;
  language: string;     // 'auto' | 'en' | 'es' | 'fr' | 'de' | 'ja'
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
  achievement_notifications: boolean;
}

// ---------------------------------------------------------------------------
// Achievement types — mirror Rust structs in achievements/mod.rs
// ---------------------------------------------------------------------------

export type AchievementCategory = 'Milestone' | 'Habit' | 'Discovery';

/** Mirrors Rust `AchievementView` — returned by `achievements_get_all`. */
export interface AchievementView {
  id: string;
  /** null for unearned secret achievements */
  name: string | null;
  /** null for unearned secret achievements */
  description: string | null;
  category: AchievementCategory;
  secret: boolean;
  /** null for unearned secret achievements */
  color: string | null;
  emoji: string;
  earned: boolean;
  /** Unix timestamp (seconds) when unlocked; null if not earned */
  unlocked_at: number | null;
  /** Current progress value for milestone achievements; null for binary ones */
  progress_current: number | null;
  /** Target progress value for milestone achievements; null for binary ones */
  progress_total: number | null;
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
  colors: Record<string, string>;  // keys like "--color-background", "--color-focus-round"
  is_custom: boolean;
}

// ---------------------------------------------------------------------------
// Stats types — mirror Rust structs in commands.rs / queries.rs
// ---------------------------------------------------------------------------

export interface DailyStats {
  rounds: number;
  focus_mins: number;
  completion_rate: number | null;  // null when no sessions started today
  by_hour: number[];               // 24 entries, index = hour of day
}

export interface DayStat {
  date: string;   // "YYYY-MM-DD"
  rounds: number;
}

export interface HeatmapEntry {
  date: string;   // "YYYY-MM-DD"
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
