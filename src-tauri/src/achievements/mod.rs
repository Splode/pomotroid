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
    pub const SESSION_COMPLETED:  &str = "session_completed";
    pub const THEME_CREATED:      &str = "theme_created";
    pub const APP_LAUNCHED:       &str = "app_launched";
    pub const SHORTCUT_USED:      &str = "shortcut_used";
    pub const SETTINGS_OPENED:    &str = "settings_opened";
    pub const SETTINGS_SAVED:     &str = "settings_saved";
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
        description: "Maintain a 3-day streak.",
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
        description: "Maintain a 7-day streak.",
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
        description: "Maintain a 30-day streak.",
        category: Category::Habit,
        secret: false,
        color: "#14b8a6",
        emoji: "🧘",
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
