pub mod defaults;

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// All user-configurable settings, fully typed.
///
/// Time fields are in **seconds** (converted from stored minutes).
/// `volume` is in the **0.0–1.0** range (converted from stored 0–100).
///
/// This struct is serialized to JSON and sent to the Svelte frontend via Tauri IPC.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub always_on_top: bool,
    pub break_always_on_top: bool,
    pub auto_start_work: bool,
    pub auto_start_break: bool,
    pub min_to_tray: bool,
    pub min_to_tray_on_close: bool,
    pub notifications_enabled: bool,
    /// Number of work rounds before a long break.
    pub long_break_interval: u32,
    /// When true the dial arc starts full and subtracts; when false it fills from empty.
    pub dial_countdown: bool,
    pub theme_mode: String,
    pub theme_light: String,
    pub theme_dark: String,
    pub tick_sounds_during_work: bool,
    pub tick_sounds_during_break: bool,
    /// Work round duration in seconds.
    pub time_work_secs: u32,
    /// Short break duration in seconds.
    pub time_short_break_secs: u32,
    /// Long break duration in seconds.
    pub time_long_break_secs: u32,
    /// Audio volume in the 0.0–1.0 range.
    pub volume: f32,
    pub shortcut_toggle: String,
    pub shortcut_reset: String,
    pub shortcut_skip: String,
    pub shortcut_restart: String,
    pub websocket_enabled: bool,
    pub websocket_port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            always_on_top: false,
            break_always_on_top: false,
            auto_start_work: true,
            auto_start_break: true,
            min_to_tray: false,
            min_to_tray_on_close: false,
            notifications_enabled: false,
            long_break_interval: 4,
            dial_countdown: true,
            theme_mode: "auto".to_string(),
            theme_light: "Pomotroid".to_string(),
            theme_dark: "Pomotroid".to_string(),
            tick_sounds_during_work: false,
            tick_sounds_during_break: false,
            time_work_secs: 25 * 60,
            time_short_break_secs: 5 * 60,
            time_long_break_secs: 15 * 60,
            volume: 1.0,
            shortcut_toggle: "Control+F1".to_string(),
            shortcut_reset: "Control+F2".to_string(),
            shortcut_skip: "Control+F3".to_string(),
            shortcut_restart: "Control+F4".to_string(),
            websocket_enabled: false,
            websocket_port: 1314,
        }
    }
}

/// Seed the `settings` table with default values for any missing keys.
/// Uses `INSERT OR IGNORE` so existing customizations are preserved.
pub fn seed_defaults(conn: &Connection) -> Result<()> {
    for (key, value) in defaults::DEFAULTS {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
    }
    Ok(())
}

/// Load all settings from the database. Falls back to `Settings::default()`
/// values for any key that is missing or cannot be parsed.
pub fn load(conn: &Connection) -> Result<Settings> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let map: HashMap<String, String> = stmt
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
        .filter_map(|r| r.ok())
        .collect();

    let d = Settings::default();
    Ok(Settings {
        always_on_top: parse_bool(&map, "always_on_top", d.always_on_top),
        break_always_on_top: parse_bool(&map, "break_always_on_top", d.break_always_on_top),
        auto_start_work: parse_bool(&map, "auto_start_work", d.auto_start_work),
        auto_start_break: parse_bool(&map, "auto_start_break", d.auto_start_break),
        min_to_tray: parse_bool(&map, "min_to_tray", d.min_to_tray),
        min_to_tray_on_close: parse_bool(&map, "min_to_tray_on_close", d.min_to_tray_on_close),
        notifications_enabled: parse_bool(&map, "notifications", d.notifications_enabled),
        long_break_interval: parse_u32(&map, "work_rounds", d.long_break_interval),
        dial_countdown: parse_bool(&map, "dial_countdown", d.dial_countdown),
        theme_mode: map
            .get("theme_mode")
            .cloned()
            .unwrap_or(d.theme_mode),
        theme_light: map
            .get("theme_light")
            .cloned()
            .unwrap_or(d.theme_light),
        theme_dark: map
            .get("theme_dark")
            .cloned()
            .unwrap_or(d.theme_dark),
        tick_sounds_during_work: parse_bool(&map, "tick_sounds_work", d.tick_sounds_during_work),
        tick_sounds_during_break: parse_bool(
            &map,
            "tick_sounds_break",
            d.tick_sounds_during_break,
        ),
        // DB stores minutes; expose seconds to the frontend and timer engine.
        time_work_secs: parse_u32(&map, "time_work_mins", d.time_work_secs / 60) * 60,
        time_short_break_secs: parse_u32(&map, "time_short_break_mins", d.time_short_break_secs / 60) * 60,
        time_long_break_secs: parse_u32(&map, "time_long_break_mins", d.time_long_break_secs / 60) * 60,
        // DB stores 0–100; convert to 0.0–1.0.
        volume: (parse_u32(&map, "volume", (d.volume * 100.0) as u32) as f32 / 100.0)
            .clamp(0.0, 1.0),
        shortcut_toggle: map
            .get("shortcut_toggle")
            .cloned()
            .unwrap_or(d.shortcut_toggle),
        shortcut_reset: map
            .get("shortcut_reset")
            .cloned()
            .unwrap_or(d.shortcut_reset),
        shortcut_skip: map
            .get("shortcut_skip")
            .cloned()
            .unwrap_or(d.shortcut_skip),
        shortcut_restart: map
            .get("shortcut_restart")
            .cloned()
            .unwrap_or(d.shortcut_restart),
        websocket_enabled: parse_bool(&map, "websocket_enabled", d.websocket_enabled),
        websocket_port: parse_u32(&map, "websocket_port", d.websocket_port as u32) as u16,
    })
}

/// Upsert a single setting by its DB key. The caller is responsible for
/// converting typed values back to their stored string representation.
pub fn save_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn parse_bool(map: &HashMap<String, String>, key: &str, default: bool) -> bool {
    map.get(key).map(|v| v == "true").unwrap_or(default)
}

fn parse_u32(map: &HashMap<String, String>, key: &str, default: u32) -> u32 {
    map.get(key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

// ---------------------------------------------------------------------------
// Tests (DATA-02 acceptance criteria)
// ---------------------------------------------------------------------------

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
    fn defaults_round_trip() {
        let conn = setup();
        seed_defaults(&conn).unwrap();
        let s = load(&conn).unwrap();

        assert_eq!(s.time_work_secs, 25 * 60);
        assert_eq!(s.time_short_break_secs, 5 * 60);
        assert_eq!(s.time_long_break_secs, 15 * 60);
        assert_eq!(s.long_break_interval, 4);
        assert!((s.volume - 1.0).abs() < f32::EPSILON);
        assert_eq!(s.shortcut_toggle, "Control+F1");
        assert_eq!(s.shortcut_reset, "Control+F2");
        assert_eq!(s.shortcut_skip, "Control+F3");
        assert_eq!(s.shortcut_restart, "Control+F4");
        assert!(!s.always_on_top);
        assert!(!s.websocket_enabled);
        assert_eq!(s.websocket_port, 1314);
        assert_eq!(s.theme_mode, "auto");
        assert_eq!(s.theme_light, "Pomotroid");
        assert_eq!(s.theme_dark, "Pomotroid");
    }

    #[test]
    fn seed_is_idempotent() {
        let conn = setup();
        seed_defaults(&conn).unwrap();
        // Second seed must not overwrite existing values.
        save_setting(&conn, "always_on_top", "true").unwrap();
        seed_defaults(&conn).unwrap();
        let s = load(&conn).unwrap();
        assert!(s.always_on_top, "seed_defaults must not overwrite saved value");
    }

    #[test]
    fn save_and_reload_bool() {
        let conn = setup();
        seed_defaults(&conn).unwrap();
        save_setting(&conn, "always_on_top", "true").unwrap();
        let s = load(&conn).unwrap();
        assert!(s.always_on_top);
    }

    #[test]
    fn save_and_reload_volume() {
        let conn = setup();
        seed_defaults(&conn).unwrap();
        save_setting(&conn, "volume", "50").unwrap();
        let s = load(&conn).unwrap();
        assert!((s.volume - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn save_and_reload_time() {
        let conn = setup();
        seed_defaults(&conn).unwrap();
        save_setting(&conn, "time_work_mins", "30").unwrap();
        let s = load(&conn).unwrap();
        assert_eq!(s.time_work_secs, 30 * 60);
    }

    #[test]
    fn missing_keys_fall_back_to_defaults() {
        let conn = setup();
        // No seed — table is empty.
        let s = load(&conn).unwrap();
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn reset_defaults_restores_all_settings() {
        // Mutate several settings (timer-related and others).
        let conn = setup();
        seed_defaults(&conn).unwrap();
        save_setting(&conn, "time_work_mins", "45").unwrap();
        save_setting(&conn, "time_short_break_mins", "10").unwrap();
        save_setting(&conn, "work_rounds", "8").unwrap();
        save_setting(&conn, "always_on_top", "true").unwrap();

        // Simulate the reset_defaults command: wipe all rows then re-seed.
        conn.execute("DELETE FROM settings", []).unwrap();
        seed_defaults(&conn).unwrap();

        let s = load(&conn).unwrap();
        // Timer settings must be restored to defaults.
        assert_eq!(s.time_work_secs, 25 * 60, "work duration must reset to 25 min");
        assert_eq!(s.time_short_break_secs, 5 * 60, "short break must reset to 5 min");
        assert_eq!(s.long_break_interval, 4, "work rounds must reset to 4");
        // Non-timer settings are also wiped and reseeded to their defaults.
        assert!(!s.always_on_top, "always_on_top must reset to default false");
    }

    #[test]
    fn boolean_settings_survive_multiple_writes() {
        // Writing the same boolean key repeatedly must not corrupt the value.
        let conn = setup();
        seed_defaults(&conn).unwrap();

        for _ in 0..5 {
            save_setting(&conn, "auto_start_work", "true").unwrap();
        }
        let s = load(&conn).unwrap();
        assert!(s.auto_start_work, "auto_start_work must remain true after repeated writes");

        for _ in 0..5 {
            save_setting(&conn, "auto_start_work", "false").unwrap();
        }
        let s = load(&conn).unwrap();
        assert!(!s.auto_start_work, "auto_start_work must be false after repeated false writes");
    }
}
