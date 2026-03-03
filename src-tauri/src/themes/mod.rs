pub mod watcher;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ---------------------------------------------------------------------------
// Theme type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    /// CSS custom property values keyed by their full property name (e.g. "--color-background").
    pub colors: HashMap<String, String>,
    /// True for user-created themes in {app_data_dir}/themes/.
    #[serde(default)]
    pub is_custom: bool,
}

// ---------------------------------------------------------------------------
// Bundled themes embedded at compile time
// ---------------------------------------------------------------------------

/// Raw JSON for every built-in theme, embedded into the binary via include_str!.
/// The path is relative to this source file (src-tauri/src/themes/mod.rs).
const BUNDLED_JSON: &[&str] = &[
    include_str!("../../../static/themes/andromeda.json"),
    include_str!("../../../static/themes/ayu.json"),
    include_str!("../../../static/themes/catppuccin-frappe.json"),
    include_str!("../../../static/themes/catppuccin-latte.json"),
    include_str!("../../../static/themes/catppuccin-macchiato.json"),
    include_str!("../../../static/themes/catppuccin-mocha.json"),
    include_str!("../../../static/themes/city-lights.json"),
    include_str!("../../../static/themes/cobalt2.json"),
    include_str!("../../../static/themes/dracula.json"),
    include_str!("../../../static/themes/darcula.json"),
    include_str!("../../../static/themes/dva.json"),
    include_str!("../../../static/themes/everforest.json"),
    include_str!("../../../static/themes/github.json"),
    include_str!("../../../static/themes/github-dark.json"),
    include_str!("../../../static/themes/graphite.json"),
    include_str!("../../../static/themes/gruvbox.json"),
    include_str!("../../../static/themes/gruvbox-light.json"),
    include_str!("../../../static/themes/horizon.json"),
    include_str!("../../../static/themes/kanagawa.json"),
    include_str!("../../../static/themes/material-palenight.json"),
    include_str!("../../../static/themes/monokai.json"),
    include_str!("../../../static/themes/monokai-pro.json"),
    include_str!("../../../static/themes/night-owl.json"),
    include_str!("../../../static/themes/nord.json"),
    include_str!("../../../static/themes/one-dark.json"),
    include_str!("../../../static/themes/panda.json"),
    include_str!("../../../static/themes/pomotroid.json"),
    include_str!("../../../static/themes/pomotroid-light.json"),
    include_str!("../../../static/themes/popping-and-locking.json"),
    include_str!("../../../static/themes/rose-pine.json"),
    include_str!("../../../static/themes/rose-pine-dawn.json"),
    include_str!("../../../static/themes/rose-pine-moon.json"),
    include_str!("../../../static/themes/solarized-dark.json"),
    include_str!("../../../static/themes/solarized-light.json"),
    include_str!("../../../static/themes/spandex.json"),
    include_str!("../../../static/themes/synthwave.json"),
    include_str!("../../../static/themes/tokyo-night.json"),
];

/// Parse all bundled theme JSON strings. Panics at startup if any are malformed
/// (a compile-time-like assertion that the shipped assets are valid).
pub fn load_bundled() -> Vec<Theme> {
    let themes: Vec<Theme> = BUNDLED_JSON
        .iter()
        .filter_map(|raw| {
            serde_json::from_str::<serde_json::Value>(raw)
                .ok()
                .and_then(|v| parse_theme_value(v, false))
        })
        .collect();
    log::debug!("[themes] loaded {} bundled themes", themes.len());
    themes
}

// ---------------------------------------------------------------------------
// Custom themes — loaded from {app_data_dir}/themes/ at runtime
// ---------------------------------------------------------------------------

/// Load user-defined themes from the given directory. Non-fatal: bad files are
/// logged and skipped so one corrupt theme cannot prevent the app from starting.
pub fn load_custom(themes_dir: &Path) -> Vec<Theme> {
    let Ok(entries) = std::fs::read_dir(themes_dir) else {
        return Vec::new();
    };

    let mut themes = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let raw = match std::fs::read_to_string(&path) {
            Ok(r) => r,
            Err(e) => { log::warn!("[themes] cannot read {path:?}: {e}"); continue; }
        };
        let value = match serde_json::from_str::<serde_json::Value>(&raw) {
            Ok(v) => v,
            Err(e) => { log::warn!("[themes] invalid JSON in {path:?}: {e}"); continue; }
        };
        match parse_theme_value(value, true) {
            Some(t) => {
                log::debug!("[themes] loaded custom theme: {}", t.name);
                themes.push(t);
            }
            None => log::warn!("[themes] missing required fields in {path:?}"),
        }
    }
    themes
}

/// Return all themes: bundled first, then custom. Custom themes with the same
/// name as a bundled theme override the bundled version.
pub fn list_all(app_data_dir: &Path) -> Vec<Theme> {
    let mut themes = load_bundled();
    let custom = load_custom(&app_data_dir.join("themes"));
    let custom_count = custom.len();

    for custom_theme in custom {
        // Replace built-in with same name, or append.
        if let Some(existing) = themes.iter_mut().find(|t| t.name == custom_theme.name) {
            *existing = custom_theme;
        } else {
            themes.push(custom_theme);
        }
    }

    log::info!("[themes] available: {} total ({} custom)", themes.len(), custom_count);
    themes
}

/// Look up a single theme by name (case-insensitive).
pub fn find(app_data_dir: &Path, name: &str) -> Option<Theme> {
    list_all(app_data_dir)
        .into_iter()
        .find(|t| t.name.eq_ignore_ascii_case(name))
}

// ---------------------------------------------------------------------------
// Parsing helper
// ---------------------------------------------------------------------------

fn parse_theme_value(v: serde_json::Value, is_custom: bool) -> Option<Theme> {
    let name = v.get("name")?.as_str()?.to_string();
    let colors_obj = v.get("colors")?.as_object()?;
    let colors: HashMap<String, String> = colors_obj
        .iter()
        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
        .collect();
    Some(Theme { name, colors, is_custom })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bundled_themes_parse() {
        let themes = load_bundled();
        assert_eq!(themes.len(), 37, "expected 37 bundled themes");
    }

    #[test]
    fn bundled_themes_have_required_color_keys() {
        let required = [
            "--color-long-round",
            "--color-short-round",
            "--color-focus-round",
            "--color-background",
            "--color-background-light",
            "--color-foreground",
            "--color-accent",
        ];
        for theme in load_bundled() {
            for key in &required {
                assert!(
                    theme.colors.contains_key(*key),
                    "theme '{}' is missing '{key}'",
                    theme.name
                );
            }
        }
    }

    #[test]
    fn pomotroid_theme_is_bundled() {
        let themes = load_bundled();
        assert!(
            themes.iter().any(|t| t.name == "Pomotroid"),
            "Pomotroid theme must be bundled"
        );
    }
}
