/// All default key-value pairs seeded into the `settings` table on first launch.
/// Values are stored as TEXT in SQLite; the loader in `mod.rs` parses them
/// into their proper Rust types.
///
/// Time values are stored in **minutes**; they are converted to seconds on load.
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
    ("theme_light", "Pomotroid"),
    ("theme_dark", "Pomotroid"),
    ("tick_sounds_work", "false"),
    ("tick_sounds_break", "false"),
    ("time_work_mins", "25"),
    ("time_short_break_mins", "5"),
    ("time_long_break_mins", "15"),
    ("volume", "100"),
    ("shortcut_toggle", "Control+F1"),
    ("shortcut_reset", "Control+F2"),
    ("shortcut_skip", "Control+F3"),
    ("shortcut_restart", "Control+F4"),
    ("websocket_enabled", "false"),
    ("websocket_port", "1314"),
    ("language", "auto"),
    ("verbose_logging", "false"),
];
