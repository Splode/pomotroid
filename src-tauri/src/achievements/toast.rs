use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::achievements::ACHIEVEMENTS;

/// Spawn a short-lived decoration-free achievement toast window at the
/// bottom-right corner of the primary monitor.
///
/// All display data (name, emoji, color, theme background) is embedded in the
/// URL query string so the frontend needs no IPC calls — preventing HMR
/// interference with other windows in dev mode.
pub fn spawn_toast_window(app: &AppHandle, ids: &[String], count: u32) {
    let (pos_x, pos_y) = get_corner_position(app);

    // Resolve achievement display data from the static definitions.
    let first = ids.first().and_then(|id| ACHIEVEMENTS.iter().find(|a| a.id == id.as_str()));
    let name = if count > 1 {
        format!("{count} achievements!")
    } else {
        first.map(|a| a.name.to_string()).unwrap_or_default()
    };
    let emoji  = first.map(|a| a.emoji).unwrap_or("");
    let color  = first.map(|a| a.color).unwrap_or("#888888");

    // Resolve theme background + foreground so the toast matches the app.
    let (bg, fg) = resolve_theme_colors(app);

    let url = format!(
        "/achievement-toast?count={count}&name={name}&emoji={emoji}&color={color}&bg={bg}&fg={fg}",
        name  = urlencoding::encode(&name),
        emoji = urlencoding::encode(emoji),
        color = urlencoding::encode(color),
        bg    = urlencoding::encode(&bg),
        fg    = urlencoding::encode(&fg),
    );

    let label = format!(
        "achievement-toast-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );

    match WebviewWindowBuilder::new(app, &label, WebviewUrl::App(url.into()))
        .title("Achievement")
        .inner_size(320.0, 66.0)
        .min_inner_size(320.0, 66.0)
        .max_inner_size(320.0, 66.0)
        .position(pos_x, pos_y)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(false)
        .build()
    {
        Ok(_) => log::debug!("[achievements/toast] window created: {label}"),
        Err(e) => log::warn!("[achievements/toast] failed to create window: {e}"),
    }
}

/// Read the active theme's --color-background and --color-foreground.
/// Falls back to safe dark-mode defaults if anything fails.
fn resolve_theme_colors(app: &AppHandle) -> (String, String) {
    let default = ("#0e0e12".to_string(), "#ddd0bc".to_string());

    let Some(db) = app.try_state::<crate::db::DbState>() else { return default; };
    let Ok(conn) = db.lock() else { return default; };
    let Ok(settings) = crate::settings::load(&conn) else { return default; };
    let Ok(data_dir) = app.path().app_data_dir() else { return default; };

    // Resolve which theme name is active.
    let theme_name = match settings.theme_mode.as_str() {
        "dark"  => &settings.theme_dark,
        "light" => &settings.theme_light,
        _ => {
            // "auto" — follow the OS dark/light preference via the main window.
            let is_dark = app
                .get_webview_window("main")
                .and_then(|w| w.theme().ok())
                .map(|t| matches!(t, tauri::Theme::Dark))
                .unwrap_or(true);
            if is_dark { &settings.theme_dark } else { &settings.theme_light }
        }
    };

    let Some(theme) = crate::themes::find(&data_dir, theme_name) else { return default; };

    let bg = theme.colors.get("--color-background").cloned().unwrap_or(default.0.clone());
    let fg = theme.colors.get("--color-foreground").cloned().unwrap_or(default.1.clone());
    (bg, fg)
}

fn get_corner_position(app: &AppHandle) -> (f64, f64) {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(Some(monitor)) = window.primary_monitor() {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            let w = size.width as f64 / scale;
            let h = size.height as f64 / scale;
            // 12px margin from the right and bottom edges.
            return (w - 320.0 - 12.0, h - 66.0 - 12.0);
        }
    }
    (1280.0 - 332.0, 720.0 - 78.0)
}
