/// All default key-value pairs seeded into the `settings` table on first launch.
/// Values are stored as TEXT in SQLite; the loader in `mod.rs` parses them
/// into their proper Rust types.
///
/// Time values are stored in **seconds** (since MIGRATION_2).
/// Volume is stored on a **0–100** integer scale; converted to 0.0–1.0 on load.
pub const DEFAULTS: &[(&str, &str)] = &[
    ("always_on_top", "false"),
    ("break_always_on_top", "false"),
    ("auto_start_work", "true"),
    ("auto_start_break", "true"),
    ("tray_icon_enabled", "false"),
    ("min_to_tray", "false"),
    ("min_to_tray_on_close", "false"),
    ("notifications", "false"),
    ("work_rounds", "4"),
    ("dial_countdown", "true"),
    ("theme_mode", "auto"),
    ("theme_light", "Pomotroid Light"),
    ("theme_dark", "Pomotroid"),
    ("tick_sounds_work", "false"),
    ("tick_sounds_break", "false"),
    ("time_work_secs", "1500"),
    ("time_short_break_secs", "300"),
    ("time_long_break_secs", "900"),
    ("volume", "100"),
    ("websocket_enabled", "false"),
    ("websocket_port", "1314"),
    ("language", "auto"),
    ("verbose_logging", "false"),
    ("check_for_updates", "true"),
];
