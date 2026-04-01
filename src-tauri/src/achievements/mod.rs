pub mod eval;
pub mod toast;

use serde::Serialize;

// ---------------------------------------------------------------------------
// Event name constants
// Every named event that the achievement system knows about is listed here.
// Call `achievements_eval::record_event(event::FOO, ...)` wherever that
// event occurs; add new constants freely as new trackable moments are found.
// ---------------------------------------------------------------------------

pub mod event {
    pub const SESSION_COMPLETED:         &str = "session_completed";
    pub const THEME_CREATED:             &str = "theme_created";
    pub const APP_LAUNCHED:              &str = "app_launched";
    pub const SHORTCUT_USED:             &str = "shortcut_used";
    pub const SETTINGS_OPENED:           &str = "settings_opened";
    pub const SETTINGS_SAVED:            &str = "settings_saved";
    pub const STATS_OPENED:              &str = "stats_opened";
    pub const THEME_APPLIED:             &str = "theme_applied";
    pub const AUDIO_CUSTOM_LOADED:       &str = "audio_custom_loaded";
    pub const LANGUAGE_CHANGED:          &str = "language_changed";
    pub const WEBSOCKET_ENABLED:         &str = "websocket_enabled";
    pub const WEBSOCKET_MESSAGE:         &str = "websocket_message";
    pub const STATS_LONG_VIEW:           &str = "stats_long_view";
    pub const SESSION_TRAY:              &str = "session_tray";
    pub const SESSION_COMPACT:           &str = "session_compact";
    pub const SESSION_ALWAYS_ON_TOP:     &str = "session_always_on_top";
    pub const SESSION_SKIPPED_LATE:      &str = "session_skipped_late";
    pub const SESSION_WEBSOCKET_ACTIVE:  &str = "session_websocket_active";
    pub const SESSION_SILENT:            &str = "session_silent";
}

// ---------------------------------------------------------------------------
// Progress kind — how completion is measured
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum ProgressKind {
    /// Achievement is binary (done or not done).
    Binary,
    /// Achievement tracks a count toward a target.
    Count { target: u32 },
}

// ---------------------------------------------------------------------------
// Achievement category
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Category {
    Milestone,
    Habit,
    Discovery,
}

// ---------------------------------------------------------------------------
// Static achievement definition
// ---------------------------------------------------------------------------

pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub secret: bool,
    pub color: &'static str,
    pub emoji: &'static str,
    pub progress: ProgressKind,
    /// Event names (from `event::*`) that trigger re-evaluation of this achievement.
    pub triggers: &'static [&'static str],
}

// ---------------------------------------------------------------------------
// Serializable view sent to the frontend
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct AchievementView {
    pub id: String,
    /// null for unearned secret achievements
    pub name: Option<String>,
    /// null for unearned secret achievements
    pub description: Option<String>,
    pub category: Category,
    pub secret: bool,
    /// null for unearned secret achievements
    pub color: Option<String>,
    pub emoji: String,
    pub earned: bool,
    /// Unix timestamp (seconds) when first unlocked; null if not earned
    pub unlocked_at: Option<i64>,
    /// Current progress for milestone achievements; null for binary ones
    pub progress_current: Option<u32>,
    /// Target progress for milestone achievements; null for binary ones
    pub progress_total: Option<u32>,
}

// ---------------------------------------------------------------------------
// Static achievement list — all 16 achievements
// ---------------------------------------------------------------------------

pub static ACHIEVEMENTS: &[AchievementDef] = &[
    // --- Milestone ---
    AchievementDef {
        id: "the_seed",
        name: "The Seed",
        description: "Complete your first session.",
        category: Category::Milestone,
        secret: false,
        color: "#22c55e",
        emoji: "🌱",
        progress: ProgressKind::Count { target: 1 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "hat_trick",
        name: "Hat-Trick",
        description: "Complete 3 sessions in a single day.",
        category: Category::Milestone,
        secret: false,
        color: "#f97316",
        emoji: "⚽",
        progress: ProgressKind::Count { target: 3 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "the_centurion",
        name: "The Centurion",
        description: "Complete 100 sessions.",
        category: Category::Milestone,
        secret: false,
        color: "#eab308",
        emoji: "⚔️",
        progress: ProgressKind::Count { target: 100 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "tomato_baron",
        name: "Tomato Baron",
        description: "Complete 500 sessions.",
        category: Category::Milestone,
        secret: false,
        color: "#ef4444",
        emoji: "🍅",
        progress: ProgressKind::Count { target: 500 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "tomato_tycoon",
        name: "Tomato Tycoon",
        description: "Complete 1,000 sessions.",
        category: Category::Milestone,
        secret: false,
        color: "#a855f7",
        emoji: "👑",
        progress: ProgressKind::Count { target: 1000 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "time_lord",
        name: "Time Lord",
        description: "Accumulate 100 hours of focus time.",
        category: Category::Milestone,
        secret: false,
        color: "#3b82f6",
        emoji: "⏳",
        progress: ProgressKind::Count { target: 100 },
        triggers: &[event::SESSION_COMPLETED],
    },
    // --- Habit ---
    AchievementDef {
        id: "on_a_roll",
        name: "On a Roll",
        description: "Maintain a 3-day work session streak.",
        category: Category::Habit,
        secret: false,
        color: "#fb923c",
        emoji: "🔥",
        progress: ProgressKind::Count { target: 3 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "week_warrior",
        name: "Week Warrior",
        description: "Maintain a 7-day work session streak.",
        category: Category::Habit,
        secret: false,
        color: "#10b981",
        emoji: "🛡️",
        progress: ProgressKind::Count { target: 7 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "month_of_zen",
        name: "Month of Zen",
        description: "Maintain a 30-day work session streak.",
        category: Category::Habit,
        secret: false,
        color: "#14b8a6",
        emoji: "🪷",
        progress: ProgressKind::Count { target: 30 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "comeback_kid",
        name: "Comeback Kid",
        description: "Return after a broken streak.",
        category: Category::Habit,
        secret: true,
        color: "#ec4899",
        emoji: "💪",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    // --- Discovery ---
    AchievementDef {
        id: "early_bird",
        name: "Early Bird",
        description: "Complete a session before 7am.",
        category: Category::Discovery,
        secret: true,
        color: "#fbbf24",
        emoji: "🌅",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "midnight_oil",
        name: "Midnight Oil",
        description: "Complete a session after 11pm.",
        category: Category::Discovery,
        secret: true,
        color: "#6366f1",
        emoji: "🦉",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "perfect_day",
        name: "Perfect Day",
        description: "100% completion rate with at least 4 sessions in a day.",
        category: Category::Discovery,
        secret: true,
        color: "#34d399",
        emoji: "✨",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "the_long_haul",
        name: "The Long Haul",
        description: "Complete a full Pomodoro cycle.",
        category: Category::Discovery,
        secret: false,
        color: "#d97706",
        emoji: "🏔️",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "creature_of_habit",
        name: "Creature of Habit",
        description: "Work in the same hour on 5 different days.",
        category: Category::Discovery,
        secret: true,
        color: "#8b5cf6",
        emoji: "🔄",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "theme_artist",
        name: "Theme Artist",
        description: "Create your first custom theme.",
        category: Category::Discovery,
        secret: false,
        color: "#f472b6",
        emoji: "🎨",
        progress: ProgressKind::Binary,
        triggers: &[event::THEME_CREATED],
    },

    // -----------------------------------------------------------------------
    // Normal — Habit (streaks / consistency)
    // -----------------------------------------------------------------------
    AchievementDef {
        id: "showing_up",
        name: "Showing Up",
        description: "Use the app 7 days in a row.",
        category: Category::Habit,
        secret: false,
        color: "#4ade80",
        emoji: "📅",
        progress: ProgressKind::Count { target: 7 },
        triggers: &[event::APP_LAUNCHED],
    },
    AchievementDef {
        id: "daily_devotee",
        name: "Daily Devotee",
        description: "Use the app 30 days in a row.",
        category: Category::Habit,
        secret: false,
        color: "#16a34a",
        emoji: "🗓️",
        progress: ProgressKind::Count { target: 30 },
        triggers: &[event::APP_LAUNCHED],
    },
    AchievementDef {
        id: "the_long_game",
        name: "The Long Game",
        description: "Achieve a 100-day work session streak.",
        category: Category::Habit,
        secret: false,
        color: "#0f766e",
        emoji: "📆",
        progress: ProgressKind::Count { target: 100 },
        triggers: &[event::APP_LAUNCHED],
    },
    AchievementDef {
        id: "morning_ritual",
        name: "Morning Ritual",
        description: "Complete 3 sessions before noon, 5 days in a row.",
        category: Category::Habit,
        secret: false,
        color: "#f97316",
        emoji: "☕",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "balanced",
        name: "Balanced",
        description: "Take every scheduled break for an entire week.",
        category: Category::Habit,
        secret: false,
        color: "#0d9488",
        emoji: "🧘",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "stretch_break",
        name: "Stretch Break",
        description: "Don't skip a long break for 2 weeks.",
        category: Category::Habit,
        secret: false,
        color: "#84cc16",
        emoji: "🚶",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "weekly_review",
        name: "Weekly Review",
        description: "Open Statistics at least once every week for a month.",
        category: Category::Habit,
        secret: false,
        color: "#0ea5e9",
        emoji: "📋",
        progress: ProgressKind::Binary,
        triggers: &[event::STATS_OPENED],
    },

    // -----------------------------------------------------------------------
    // Normal — Milestone (counts / big goals)
    // -----------------------------------------------------------------------
    AchievementDef {
        id: "flow_state",
        name: "Flow State",
        description: "Complete 4 sessions in a row without skipping a break.",
        category: Category::Milestone,
        secret: false,
        color: "#eab308",
        emoji: "⚡",
        progress: ProgressKind::Count { target: 4 },
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "heatmap_inferno",
        name: "Heatmap Inferno",
        description: "Complete 5+ sessions every day for an entire week.",
        category: Category::Milestone,
        secret: false,
        color: "#ef4444",
        emoji: "🌋",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "full_palette",
        name: "The Full Palette",
        description: "Try 10 different themes.",
        category: Category::Milestone,
        secret: false,
        color: "#8b5cf6",
        emoji: "🖌️",
        progress: ProgressKind::Count { target: 10 },
        triggers: &[event::THEME_APPLIED],
    },
    AchievementDef {
        id: "front_and_center",
        name: "Front and Center",
        description: "Complete 4 sessions with Always on Top enabled.",
        category: Category::Milestone,
        secret: false,
        color: "#ea580c",
        emoji: "📌",
        progress: ProgressKind::Count { target: 4 },
        triggers: &[event::SESSION_ALWAYS_ON_TOP],
    },
    AchievementDef {
        id: "compact_champion",
        name: "Compact Champion",
        description: "Complete 5 sessions in compact mode.",
        category: Category::Milestone,
        secret: false,
        color: "#7c3aed",
        emoji: "📦",
        progress: ProgressKind::Count { target: 5 },
        triggers: &[event::SESSION_COMPACT],
    },
    AchievementDef {
        id: "power_user",
        name: "Power User",
        description: "Use a global shortcut 50 times.",
        category: Category::Milestone,
        secret: false,
        color: "#6366f1",
        emoji: "🎮",
        progress: ProgressKind::Count { target: 50 },
        triggers: &[event::SHORTCUT_USED],
    },

    // -----------------------------------------------------------------------
    // Normal — Discovery (first-time or exploratory)
    // -----------------------------------------------------------------------
    AchievementDef {
        id: "rest_is_productive",
        name: "Rest is Productive",
        description: "Complete a day with 4+ sessions and zero skipped breaks.",
        category: Category::Discovery,
        secret: false,
        color: "#22c55e",
        emoji: "🌿",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "first_impression",
        name: "First Impression",
        description: "Apply your first non-default theme.",
        category: Category::Discovery,
        secret: false,
        color: "#ec4899",
        emoji: "🖼️",
        progress: ProgressKind::Binary,
        triggers: &[event::THEME_APPLIED],
    },
    AchievementDef {
        id: "by_the_numbers",
        name: "By the Numbers",
        description: "Open the Statistics view for the first time.",
        category: Category::Discovery,
        secret: false,
        color: "#3b82f6",
        emoji: "📊",
        progress: ProgressKind::Binary,
        triggers: &[event::STATS_OPENED],
    },
    AchievementDef {
        id: "your_rules",
        name: "Your Rules",
        description: "Change the default work duration for the first time.",
        category: Category::Discovery,
        secret: false,
        color: "#f59e0b",
        emoji: "⏱️",
        progress: ProgressKind::Binary,
        triggers: &[event::SETTINGS_SAVED],
    },
    AchievementDef {
        id: "sound_check",
        name: "Sound Check",
        description: "Load a custom audio file for a round alert.",
        category: Category::Discovery,
        secret: false,
        color: "#a3e635",
        emoji: "🔔",
        progress: ProgressKind::Binary,
        triggers: &[event::AUDIO_CUSTOM_LOADED],
    },
    AchievementDef {
        id: "lost_in_translation",
        name: "Lost in Translation",
        description: "Switch the app language from the default.",
        category: Category::Discovery,
        secret: false,
        color: "#34d399",
        emoji: "🌍",
        progress: ProgressKind::Binary,
        triggers: &[event::LANGUAGE_CHANGED],
    },
    AchievementDef {
        id: "background_worker",
        name: "Background Worker",
        description: "Complete a session while minimised to the tray.",
        category: Category::Discovery,
        secret: false,
        color: "#64748b",
        emoji: "👻",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_TRAY],
    },

    // -----------------------------------------------------------------------
    // Secret
    // -----------------------------------------------------------------------
    AchievementDef {
        id: "too_eager",
        name: "Too Eager",
        description: "Start a new session within 3 seconds of finishing one, 3 times.",
        category: Category::Discovery,
        secret: true,
        color: "#f97316",
        emoji: "🏃",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "slow_and_steady",
        name: "Slow and Steady",
        description: "Wait 5+ minutes before your first session of the day, 7 days running.",
        category: Category::Habit,
        secret: true,
        color: "#78716c",
        emoji: "🐌",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "obsessive_saver",
        name: "Obsessive Saver",
        description: "Open settings 10 times without changing anything.",
        category: Category::Discovery,
        secret: true,
        color: "#6b7280",
        emoji: "💾",
        progress: ProgressKind::Binary,
        triggers: &[event::SETTINGS_OPENED],
    },
    AchievementDef {
        id: "rebel",
        name: "Rebel",
        description: "Skip a session in the last 60 seconds, 3 times.",
        category: Category::Discovery,
        secret: true,
        color: "#f43f5e",
        emoji: "🙃",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_SKIPPED_LATE],
    },
    AchievementDef {
        id: "holiday_focus",
        name: "Holiday Focus",
        description: "Complete a session on December 25th.",
        category: Category::Discovery,
        secret: true,
        color: "#dc2626",
        emoji: "🎄",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "new_year_focus",
        name: "New Year, New Focus",
        description: "Complete a session on January 1st.",
        category: Category::Discovery,
        secret: true,
        color: "#1d4ed8",
        emoji: "🎆",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "self_love",
        name: "Self-Love",
        description: "Complete a session on February 14th.",
        category: Category::Discovery,
        secret: true,
        color: "#db2777",
        emoji: "💝",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "lucky_streak",
        name: "Lucky Streak",
        description: "Complete exactly 7 sessions in one day.",
        category: Category::Discovery,
        secret: true,
        color: "#d97706",
        emoji: "🎰",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "perfect_ten",
        name: "Perfect Ten",
        description: "Complete exactly 10 sessions in a single day.",
        category: Category::Discovery,
        secret: true,
        color: "#ca8a04",
        emoji: "🔢",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "ghost_mode",
        name: "Ghost Mode",
        description: "Use the app 5 days in a row without changing any settings.",
        category: Category::Habit,
        secret: true,
        color: "#475569",
        emoji: "🫥",
        progress: ProgressKind::Binary,
        triggers: &[event::APP_LAUNCHED],
    },
    AchievementDef {
        id: "the_completionist",
        name: "The Completionist",
        description: "Unlock every non-secret achievement.",
        category: Category::Discovery,
        secret: true,
        color: "#fbbf24",
        emoji: "📖",
        progress: ProgressKind::Binary,
        // Empty triggers — evaluated explicitly in on_event after any unlock.
        triggers: &[],
    },
    AchievementDef {
        id: "wired_in",
        name: "Wired In",
        description: "Enable the WebSocket server for the first time.",
        category: Category::Discovery,
        secret: true,
        color: "#06b6d4",
        emoji: "🔌",
        progress: ProgressKind::Binary,
        triggers: &[event::WEBSOCKET_ENABLED],
    },
    AchievementDef {
        id: "streaming_live",
        name: "Streaming Live",
        description: "Complete 5 sessions with the WebSocket server active.",
        category: Category::Milestone,
        secret: true,
        color: "#0891b2",
        emoji: "📡",
        progress: ProgressKind::Count { target: 5 },
        triggers: &[event::SESSION_WEBSOCKET_ACTIVE],
    },
    AchievementDef {
        id: "automated",
        name: "Automated",
        description: "Receive a remote getState request via WebSocket.",
        category: Category::Discovery,
        secret: true,
        color: "#334155",
        emoji: "🤖",
        progress: ProgressKind::Binary,
        triggers: &[event::WEBSOCKET_MESSAGE],
    },
    AchievementDef {
        id: "deep_dive",
        name: "Deep Dive",
        description: "Spend more than 5 minutes on the Statistics page.",
        category: Category::Discovery,
        secret: true,
        color: "#2563eb",
        emoji: "🔬",
        progress: ProgressKind::Binary,
        triggers: &[event::STATS_LONG_VIEW],
    },
    AchievementDef {
        id: "history_buff",
        name: "History Buff",
        description: "Open Statistics on the last day of the year.",
        category: Category::Discovery,
        secret: true,
        color: "#b45309",
        emoji: "📜",
        progress: ProgressKind::Binary,
        triggers: &[event::STATS_OPENED],
    },
    AchievementDef {
        id: "tres_bien",
        name: "Très Bien",
        description: "Use the app in French for an entire day.",
        category: Category::Discovery,
        secret: true,
        color: "#1e40af",
        emoji: "🥐",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "chromesthete",
        name: "Chromesthete",
        description: "Have 3 or more custom themes loaded at once.",
        category: Category::Discovery,
        secret: true,
        color: "#c026d3",
        emoji: "👁️‍🗨️",
        progress: ProgressKind::Binary,
        triggers: &[event::THEME_CREATED],
    },
    AchievementDef {
        id: "marathon",
        name: "Marathon",
        description: "Complete a session with a work duration of 60+ minutes.",
        category: Category::Discovery,
        secret: true,
        color: "#9a3412",
        emoji: "🏋️",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "baby_steps",
        name: "Baby Steps",
        description: "Complete 3 sessions in a row with a duration of 5 minutes or less.",
        category: Category::Discovery,
        secret: true,
        color: "#a16207",
        emoji: "🐣",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "no_rest",
        name: "No Rest for the Wicked",
        description: "Set long break interval to 6+ rounds and reach the long break.",
        category: Category::Discovery,
        secret: true,
        color: "#991b1b",
        emoji: "♾️",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "cold_spell",
        name: "Cold Spell",
        description: "Return after a gap of 2+ weeks and complete a session.",
        category: Category::Discovery,
        secret: true,
        color: "#0284c7",
        emoji: "❄️",
        progress: ProgressKind::Binary,
        triggers: &[event::SESSION_COMPLETED],
    },
    AchievementDef {
        id: "library_mode",
        name: "Library Mode",
        description: "Complete 4 sessions with all sounds disabled.",
        category: Category::Discovery,
        secret: true,
        color: "#374151",
        emoji: "🤫",
        progress: ProgressKind::Count { target: 4 },
        triggers: &[event::SESSION_SILENT],
    },
];

// ---------------------------------------------------------------------------
// Helper: build an AchievementView from a definition + earned state
// ---------------------------------------------------------------------------

pub fn build_view(
    def: &AchievementDef,
    unlocked_at: Option<i64>,
    progress_current: Option<u32>,
) -> AchievementView {
    let earned = unlocked_at.is_some();
    let reveal = earned || !def.secret;

    let progress_total = match def.progress {
        ProgressKind::Count { target } => Some(target),
        ProgressKind::Binary => None,
    };

    AchievementView {
        id: def.id.to_string(),
        name: if reveal { Some(def.name.to_string()) } else { None },
        description: if reveal { Some(def.description.to_string()) } else { None },
        category: def.category,
        secret: def.secret,
        color: if reveal { Some(def.color.to_string()) } else { None },
        emoji: def.emoji.to_string(),
        earned,
        unlocked_at,
        progress_current: if earned { None } else { progress_current },
        progress_total: if earned { None } else { progress_total },
    }
}
