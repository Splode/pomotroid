/// System tray management with dynamic arc icon via tiny-skia.
///
/// The tray icon is a 64×64 RGBA image displaying remaining time as "MM:SS"
/// text on a colored background matching the round type (work/short-break/long-break).
/// While paused: two semi-transparent bars drawn over the time display.
///
/// Colors come from the active theme, updated when theme changes.
/// The tray is created/destroyed when `min_to_tray` setting changes.
///
/// ## Linux / Wayland note
/// Tauri's tray support on Linux is backed by `libappindicator-sys`, which
/// `dlopen`s `libayatana-appindicator3` or `libappindicator3` at runtime and
/// panics (aborting the process) if neither is found.  KDE Plasma 6 on Wayland
/// typically does not ship these libraries.  `create_tray` probes for them
/// before calling `TrayIconBuilder::build()` and returns early with a warning
/// when they are absent, preventing the abort.
use std::sync::{Arc, Mutex};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::timer::TimerController;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontRef, PxScale};

// ---------------------------------------------------------------------------
// Theme colors for tray rendering
// ---------------------------------------------------------------------------

/// Color tokens needed for tray icon rendering.
#[derive(Clone)]
pub struct TrayColors {
    pub background: [u8; 4],
    pub focus_round: [u8; 4],
    pub short_round: [u8; 4],
    pub long_round: [u8; 4],
    pub foreground: [u8; 4],
}

impl Default for TrayColors {
    fn default() -> Self {
        // Pomotroid theme defaults (matches pomotroid.json bundled theme).
        Self {
            background: [47, 56, 75, 255],    // #2F384B
            focus_round: [226, 93, 96, 255],  // #E25D60
            short_round: [53, 188, 174, 255], // #35BCAE
            long_round: [89, 174, 209, 255],  // #59AED1
            foreground: [255, 255, 255, 255], // #FFFFFF
        }
    }
}

impl TrayColors {
    /// Build a `TrayColors` from a theme's CSS-variable color map.
    /// Falls back to `TrayColors::default()` values for any key that is
    /// missing or unparseable.
    pub fn from_colors_map(colors: &std::collections::HashMap<String, String>) -> Self {
        let d = Self::default();
        let get = |key: &str, fallback: [u8; 4]| {
            colors.get(key)
                .and_then(|hex| parse_hex_color(hex))
                .unwrap_or(fallback)
        };
        Self {
            background: get("--color-background", d.background),
            focus_round: get("--color-focus-round", d.focus_round),
            short_round: get("--color-short-round", d.short_round),
            long_round:  get("--color-long-round",  d.long_round),
            foreground:  get("--color-foreground",   d.foreground),
        }
    }
}

/// Parse a CSS hex color (#RRGGBB or #RRGGBBAA) into [r, g, b, a].
pub fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let h = hex.strip_prefix('#')?;
    match h.len() {
        6 => {
            let r = u8::from_str_radix(&h[0..2], 16).ok()?;
            let g = u8::from_str_radix(&h[2..4], 16).ok()?;
            let b = u8::from_str_radix(&h[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&h[0..2], 16).ok()?;
            let g = u8::from_str_radix(&h[2..4], 16).ok()?;
            let b = u8::from_str_radix(&h[4..6], 16).ok()?;
            let a = u8::from_str_radix(&h[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Shared tray state
// ---------------------------------------------------------------------------

/// Handles to the dynamic timer-control menu items.
/// Stored in `TrayState` so the timer event thread can update labels/enabled states.
pub struct TrayMenuItems {
    pub toggle: MenuItem<tauri::Wry>,
    pub skip: MenuItem<tauri::Wry>,
    pub reset_round: MenuItem<tauri::Wry>,
}

/// Tauri-managed state for the tray icon (uses the default Wry runtime).
pub struct TrayState {
    pub icon: Mutex<Option<TrayIcon<tauri::Wry>>>,
    pub colors: Mutex<TrayColors>,
    pub countdown_mode: Mutex<bool>,
    pub menu_items: Mutex<Option<TrayMenuItems>>,
}

impl TrayState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            icon: Mutex::new(None),
            colors: Mutex::new(TrayColors::default()),
            countdown_mode: Mutex::new(false),
            menu_items: Mutex::new(None),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrayWindowAction {
    Hide,
    Restore,
}

fn tray_window_action(is_visible: bool, is_minimized: bool) -> TrayWindowAction {
    if is_visible && !is_minimized {
        TrayWindowAction::Hide
    } else {
        TrayWindowAction::Restore
    }
}

// ---------------------------------------------------------------------------
// Tray lifecycle
// ---------------------------------------------------------------------------

/// Check whether the appindicator shared library is loadable on this system.
///
/// `libappindicator-sys` panics with an unrecoverable abort when none of the
/// four candidate `.so` names can be opened.  We probe via `dlopen` first so
/// `create_tray` can return gracefully instead of crashing the process.
///
/// KDE Plasma 6 on Wayland typically does not ship these libraries.
#[cfg(target_os = "linux")]
pub(crate) fn appindicator_available() -> bool {
    use std::ffi::c_char;
    extern "C" {
        fn dlopen(filename: *const c_char, flags: i32) -> *mut std::ffi::c_void;
        fn dlclose(handle: *mut std::ffi::c_void) -> i32;
    }
    const RTLD_LAZY: i32 = 0x0001;
    // Mirror the four names tried by libappindicator-sys 0.9.
    let candidates: &[&[u8]] = &[
        b"libayatana-appindicator3.so.1\0",
        b"libappindicator3.so.1\0",
        b"libayatana-appindicator3.so\0",
        b"libappindicator3.so\0",
    ];
    log::debug!("[tray] probing for appindicator shared library");
    let result = candidates.iter().find_map(|name| unsafe {
        let name_str = std::str::from_utf8(name)
            .unwrap_or("?")
            .trim_end_matches('\0');
        log::debug!("[tray] trying {name_str}");
        let handle = dlopen(name.as_ptr() as *const c_char, RTLD_LAZY);
        if !handle.is_null() {
            dlclose(handle);
            Some(name_str)
        } else {
            None
        }
    });
    match result {
        Some(found) => {
            log::info!("[tray] appindicator found: {found}");
            true
        }
        None => {
            log::warn!(
                "[tray] appindicator not found — checked: {}",
                candidates
                    .iter()
                    .map(|n| std::str::from_utf8(n).unwrap_or("?").trim_end_matches('\0'))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            false
        }
    }
}

/// Show the system tray icon.
///
/// If the icon has already been created (e.g. it was previously hidden), it is
/// made visible again without allocating a second OS icon.  A new icon is only
/// built on the very first call.
pub fn create_tray(app: &AppHandle, state: &Arc<TrayState>) {
    // Re-show existing icon if present — avoids duplicate OS tray entries.
    {
        let guard = state.icon.lock().unwrap();
        if let Some(existing) = guard.as_ref() {
            let _ = existing.set_visible(true);
            log::info!("[tray] shown (reused existing icon)");
            return;
        }
    }

    // On Linux, libappindicator-sys aborts the process if neither
    // libayatana-appindicator3 nor libappindicator3 is installed (common on
    // KDE Plasma 6 / Wayland).  Bail out gracefully if the probe fails.
    #[cfg(target_os = "linux")]
    if !appindicator_available() {
        log::warn!(
            "[tray] libayatana-appindicator3 / libappindicator3 not found — \
             system tray is unavailable on this system. \
             Install libayatana-appindicator3 to enable it."
        );
        return;
    }

    let toggle_item = match MenuItem::with_id(app, "toggle", "Start", true, None::<&str>) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let skip_item = match MenuItem::with_id(app, "skip", "Skip", false, None::<&str>) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let reset_item = match MenuItem::with_id(app, "reset-round", "Reset Round", false, None::<&str>) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let sep = match PredefinedMenuItem::separator(app) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let show_item = match MenuItem::with_id(app, "show", "Show", true, None::<&str>) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let exit_item = match MenuItem::with_id(app, "exit", "Exit", true, None::<&str>) {
        Ok(i) => i,
        Err(e) => { log::warn!("[tray] menu item error: {e}"); return; }
    };
    let menu = match Menu::with_items(app, &[&toggle_item, &skip_item, &reset_item, &sep, &show_item, &exit_item]) {
        Ok(m) => m,
        Err(e) => { log::warn!("[tray] menu error: {e}"); return; }
    };

    // Render the initial idle icon using the current state (respects countdown mode
    // and theme colors already set before create_tray is called).
    let image = {
        let colors = state.colors.lock().unwrap().clone();
        let countdown = *state.countdown_mode.lock().unwrap();
        let bytes = render_tray_icon_rgba(&colors, false, 0.0, "work", countdown, 0);
        Image::new_owned(bytes, SIZE, SIZE)
    };

    let tray = TrayIconBuilder::new()
        .icon(image)
        .tooltip("Pomotroid")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray_icon, event| {
            // Left-click: toggle window visibility.
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray_icon.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let visible = window.is_visible().unwrap_or(false);
                    let minimized = window.is_minimized().unwrap_or(false);
                    match tray_window_action(visible, minimized) {
                        TrayWindowAction::Hide => {
                            log::debug!("[tray] left-click → hide");
                            let _ = window.hide();
                        }
                        TrayWindowAction::Restore => {
                            log::debug!("[tray] left-click → show");
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "toggle" => {
                    if let Some(timer) = app.try_state::<TimerController>() {
                        timer.toggle();
                    }
                }
                "skip" => {
                    if let Some(timer) = app.try_state::<TimerController>() {
                        timer.skip();
                    }
                }
                "reset-round" => {
                    if let Some(timer) = app.try_state::<TimerController>() {
                        timer.restart_round();
                    }
                }
                "show" => {
                    log::info!("[tray] show");
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                }
                "exit" => {
                    log::info!("[tray] exit");
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app);

    match tray {
        Ok(t) => {
            *state.icon.lock().unwrap() = Some(t);
            *state.menu_items.lock().unwrap() = Some(TrayMenuItems {
                toggle: toggle_item,
                skip: skip_item,
                reset_round: reset_item,
            });
            log::info!("[tray] created");
        }
        Err(e) => log::warn!("[tray] failed to build tray icon: {e}"),
    }
}

/// Hide the system tray icon.
///
/// The underlying `TrayIcon` is kept alive so it can be shown again without
/// allocating a second OS icon.  Dropping the handle is not sufficient to
/// remove the icon on all platforms; `set_visible(false)` is the reliable path.
pub fn destroy_tray(state: &Arc<TrayState>) {
    let guard = state.icon.lock().unwrap();
    if let Some(existing) = guard.as_ref() {
        let _ = existing.set_visible(false);
        log::info!("[tray] hidden");
    }
}

// ---------------------------------------------------------------------------
// Icon update (called from the timer event listener)
// ---------------------------------------------------------------------------

/// Re-render and push a new RGBA icon to the tray.
///
/// - `round_type`: "work" | "short-break" | "long-break"
/// - `paused`: show pause bars over the progress arc
/// - `progress`: 0.0 (empty) to 1.0 (full, i.e. elapsed/total)
/// - `remaining_secs`: remaining seconds to display as text
pub fn update_icon(state: &Arc<TrayState>, round_type: &str, paused: bool, progress: f32, remaining_secs: u32) {
    let guard = state.icon.lock().unwrap();
    let Some(tray) = guard.as_ref() else { return };

    let colors = state.colors.lock().unwrap().clone();
    let countdown = *state.countdown_mode.lock().unwrap();
    let bytes = render_tray_icon_rgba(&colors, paused, progress, round_type, countdown, remaining_secs);

    let image = Image::new_owned(bytes, SIZE, SIZE);
    let _ = tray.set_icon(Some(image));
}

// ---------------------------------------------------------------------------
// Menu item update (called from the timer event listener)
// ---------------------------------------------------------------------------

/// Update the tray menu items to reflect the current timer state.
///
/// - `is_running`: timer is actively counting down.
/// - `is_paused`: timer has been started and then paused (elapsed > 0, not running).
///
/// No-op when the tray menu has not been created yet.
pub fn update_menu_items(state: &Arc<TrayState>, is_running: bool, is_paused: bool) {
    let guard = state.menu_items.lock().unwrap();
    let Some(items) = guard.as_ref() else { return };

    let toggle_label = if is_running { "Pause" } else if is_paused { "Resume" } else { "Start" };
    let controls_enabled = is_running || is_paused;

    let _ = items.toggle.set_text(toggle_label);
    let _ = items.skip.set_enabled(controls_enabled);
    let _ = items.reset_round.set_enabled(controls_enabled);
}

// ---------------------------------------------------------------------------
// Icon rendering
// ---------------------------------------------------------------------------

// Render at 64×64 so the icon looks sharp on HiDPI displays (Ubuntu often
// runs at 1.5× or 2× scale).  The tray host scales it down on standard
// density displays; the larger source means the text stays clean either way.
const SIZE: u32 = 64;

/// Render a 64×64 RGBA tray icon with remaining time text.
///
/// Instead of the ring progress, this displays the remaining time as "MM:SS"
/// in the center of the icon with a colored background based on round type.
pub fn render_tray_icon_rgba(
    colors: &TrayColors,
    paused: bool,
    _progress: f32,
    round_type: &str,
    _countdown: bool,
    remaining_secs: u32,
) -> Vec<u8> {
    // Create a transparent background
    let mut img: RgbaImage = ImageBuffer::new(SIZE, SIZE);

    // Round-type color for background circle
    let bg_color = match round_type {
        "short-break" => colors.short_round,
        "long-break"  => colors.long_round,
        _             => colors.focus_round,
    };

    // Draw filled circle background
    let center_x = (SIZE / 2) as i32;
    let center_y = (SIZE / 2) as i32;
    let radius = (SIZE / 2 - 2) as i32; // Leave 2px margin

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as i32 - center_x;
            let dy = y as i32 - center_y;
            let distance_squared = dx * dx + dy * dy;

            if distance_squared <= radius * radius {
                img.put_pixel(x, y, Rgba([bg_color[0], bg_color[1], bg_color[2], bg_color[3]]));
            } else {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0])); // Transparent outside circle
            }
        }
    }

    // Format time - only show minutes
    let minutes = remaining_secs / 60;
    let time_text = format!("{:02}", minutes);

    // Use embedded font data
    let font_data = include_bytes!("../../fonts/DejaVuSansMono-Bold.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Error loading font");

    // Use larger font scale for better visibility (48.0 instead of 24.0)
    let scale = PxScale::from(48.0);

    // Text color (white for visibility)
    let text_color = Rgba([colors.foreground[0], colors.foreground[1], colors.foreground[2], 255]);

    // Draw text centered (adjusted position for larger font and 2-digit display)
    draw_text_mut(&mut img, text_color, 8, 8, scale, &font, &time_text);

    // If paused, draw pause bars overlay
    if paused {
        let bar_color = Rgba([colors.foreground[0], colors.foreground[1], colors.foreground[2], 180]);
        let bar_width = 4;
        let bar_height = 16;
        let bar_gap = 6;
        let bar_y = (SIZE / 2 - bar_height / 2) as i32;

        // Left bar
        let bar_x1 = (SIZE / 2 - bar_gap / 2 - bar_width) as i32;
        for dy in 0..bar_height {
            for dx in 0..bar_width {
                if let Some(pixel) = img.get_pixel_mut_checked((bar_x1 + dx as i32) as u32, (bar_y + dy as i32) as u32) {
                    *pixel = bar_color;
                }
            }
        }

        // Right bar
        let bar_x2 = (SIZE / 2 + bar_gap / 2) as i32;
        for dy in 0..bar_height {
            for dx in 0..bar_width {
                if let Some(pixel) = img.get_pixel_mut_checked((bar_x2 + dx as i32) as u32, (bar_y + dy as i32) as u32) {
                    *pixel = bar_color;
                }
            }
        }
    }

    // Convert ImageBuffer to Vec<u8>
    img.into_raw()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), false, 0.5, "work", false, 1500);
        assert_eq!(bytes.len(), (SIZE * SIZE * 4) as usize);
    }

    #[test]
    fn render_paused_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), true, 0.0, "work", false, 1500);
        assert_eq!(bytes.len(), (SIZE * SIZE * 4) as usize);
    }

    #[test]
    fn render_paused_preserves_progress_arc() {
        let empty = render_tray_icon_rgba(&TrayColors::default(), true, 0.0, "work", false, 0);
        let half = render_tray_icon_rgba(&TrayColors::default(), true, 0.5, "work", false, 750);
        assert!(empty != half);
    }

    #[test]
    fn render_paused_still_shows_pause_indicator() {
        let running = render_tray_icon_rgba(&TrayColors::default(), false, 0.5, "work", false, 750);
        let paused = render_tray_icon_rgba(&TrayColors::default(), true, 0.5, "work", false, 750);
        assert!(running != paused);
    }

    #[test]
    fn render_zero_progress_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), false, 0.0, "work", false, 0);
        assert_eq!(bytes.len(), (SIZE * SIZE * 4) as usize);
    }

    #[test]
    fn parse_hex_6_digit() {
        assert_eq!(parse_hex_color("#FF8800"), Some([255, 136, 0, 255]));
    }

    #[test]
    fn parse_hex_8_digit() {
        assert_eq!(parse_hex_color("#FF880080"), Some([255, 136, 0, 128]));
    }

    #[test]
    fn parse_hex_invalid() {
        assert_eq!(parse_hex_color("not-a-color"), None);
        assert_eq!(parse_hex_color("#ZZZ"), None);
        assert_eq!(parse_hex_color(""), None);
    }

    #[test]
    fn tray_click_hides_visible_unminimized_window() {
        assert_eq!(tray_window_action(true, false), TrayWindowAction::Hide);
    }

    #[test]
    fn tray_click_restores_minimized_window() {
        assert_eq!(tray_window_action(true, true), TrayWindowAction::Restore);
    }

    #[test]
    fn tray_click_restores_hidden_window() {
        assert_eq!(tray_window_action(false, false), TrayWindowAction::Restore);
    }
}
