/// System tray management with dynamic arc icon via tiny-skia.
///
/// The tray icon is a 32×32 RGBA image:
///   - Solid filled background circle in the theme's background color.
///   - Progress arc from 12 o'clock sweeping clockwise, colored by round type.
///   - While paused: two vertical bars drawn over the background (no arc).
///
/// Colors come from the active theme, updated when theme changes.
/// The tray is created/destroyed when `min_to_tray` setting changes.
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::sync::{Arc, Mutex};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::timer::TimerController;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

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

// ---------------------------------------------------------------------------
// Tray lifecycle
// ---------------------------------------------------------------------------

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
        let bytes = render_tray_icon_rgba(&colors, false, 0.0, "work", countdown);
        Image::new_owned(bytes, SIZE, SIZE)
    };

    let tray = TrayIconBuilder::new()
        .icon(image)
        .tooltip("Pomotroid")
        .menu(&menu)
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
                    match window.is_visible() {
                        Ok(true) => {
                            log::debug!("[tray] left-click → hide");
                            let _ = window.hide();
                        }
                        _ => {
                            log::debug!("[tray] left-click → show");
                            let _ = window.show();
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
/// - `paused`: show pause bars instead of an arc
/// - `progress`: 0.0 (empty) to 1.0 (full, i.e. elapsed/total)
pub fn update_icon(state: &Arc<TrayState>, round_type: &str, paused: bool, progress: f32) {
    let guard = state.icon.lock().unwrap();
    let Some(tray) = guard.as_ref() else { return };

    let colors = state.colors.lock().unwrap().clone();
    let countdown = *state.countdown_mode.lock().unwrap();
    let bytes = render_tray_icon_rgba(&colors, paused, progress, round_type, countdown);

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
// Icon rendering (tiny-skia)
// ---------------------------------------------------------------------------

// Render at 64×64 so the icon looks sharp on HiDPI displays (Ubuntu often
// runs at 1.5× or 2× scale).  The tray host scales it down on standard
// density displays; the larger source means the circle stays clean either way.
const SIZE: u32 = 64;
const CENTER: f32 = SIZE as f32 / 2.0;
const RADIUS: f32 = CENTER - 5.0; // 5 px margin keeps the stroke inside the canvas
const STROKE_WIDTH: f32 = 6.0;
// Track opacity: the "empty" part of the ring at this brightness on a dark panel.
// 22% white on #1a1a1a ≈ #383838 — invisible. 65% ≈ #a6a6a6 — clearly visible.
const TRACK_ALPHA: u8 = 165; // ≈ 65 %

fn rgba_color(c: [u8; 4]) -> Color {
    Color::from_rgba8(c[0], c[1], c[2], c[3])
}

/// Render a 64×64 RGBA tray icon as a **ring** with a progress arc.
///
/// Using a ring (stroke-only circle) on a transparent background means the
/// icon reads as a clear circle regardless of panel colour or scale factor,
/// unlike a solid filled disc which looks like a dark blob at small sizes.
pub fn render_tray_icon_rgba(
    colors: &TrayColors,
    paused: bool,
    progress: f32,
    round_type: &str,
    countdown: bool,
) -> Vec<u8> {
    let mut pixmap = Pixmap::new(SIZE, SIZE).expect("pixmap alloc");

    let mut paint = Paint { anti_alias: true, ..Default::default() };

    let stroke = Stroke {
        width: STROKE_WIDTH,
        line_cap: tiny_skia::LineCap::Round,
        ..Default::default()
    };

    // Track ring: full circle at low opacity — defines the circular shape.
    {
        let [r, g, b, _] = colors.foreground;
        paint.set_color(Color::from_rgba8(r, g, b, TRACK_ALPHA));
    }
    let ring = {
        let mut pb = PathBuilder::new();
        pb.push_circle(CENTER, CENTER, RADIUS);
        pb.finish().expect("ring path")
    };
    pixmap.stroke_path(&ring, &paint, &stroke, Transform::identity(), None);

    // Round-type color: used for both the progress arc and the pause bars.
    let round_color = match round_type {
        "short-break" => rgba_color(colors.short_round),
        "long-break"  => rgba_color(colors.long_round),
        _             => rgba_color(colors.focus_round),
    };

    if paused {
        // Two vertical bars centred in the ring, in the round-type colour so
        // they read clearly on any panel colour regardless of theme foreground.
        paint.set_color(round_color);
        let bar_h = RADIUS * 0.75;
        let bar_w = STROKE_WIDTH * 1.2;
        let bar_gap = STROKE_WIDTH * 1.1;
        let bar_y = CENTER - bar_h / 2.0;
        for x in [CENTER - bar_gap / 2.0 - bar_w, CENTER + bar_gap / 2.0] {
            if let Some(rect) = tiny_skia::Rect::from_xywh(x, bar_y, bar_w, bar_h) {
                let p = PathBuilder::from_rect(rect);
                pixmap.fill_path(
                    &p, &paint, tiny_skia::FillRule::Winding,
                    Transform::identity(), None,
                );
            }
        }
    } else {
        // Progress arc from 12 o'clock, clockwise, in the round-type colour.
        paint.set_color(round_color);

        // In elapsed mode the arc grows as time passes; in countdown mode it shrinks.
        let effective = if countdown { 1.0 - progress } else { progress };
        let sweep = effective.clamp(0.0, 1.0) * TAU;
        if sweep > 0.001 {
            let start = -FRAC_PI_2;
            let end   = start + sweep;
            let path  = build_arc_path(CENTER, CENTER, RADIUS, start, end);
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    pixmap.take()
}

/// Approximate a circular arc with cubic Bézier segments (max π/2 per segment).
fn build_arc_path(cx: f32, cy: f32, r: f32, start: f32, end: f32) -> tiny_skia::Path {
    let total = end - start;
    let n = ((total / (PI / 2.0)).ceil() as usize).max(1);
    let step = total / n as f32;
    let mut pb = PathBuilder::new();

    for i in 0..n {
        let a0 = start + step * i as f32;
        let a1 = a0 + step;
        arc_segment(&mut pb, cx, cy, r, a0, a1, i == 0);
    }

    pb.finish().unwrap_or_else(|| {
        PathBuilder::from_rect(tiny_skia::Rect::from_xywh(cx, cy, 1.0, 1.0).unwrap())
    })
}

/// Append one arc segment (≤ π/2) as a cubic Bézier.
fn arc_segment(pb: &mut PathBuilder, cx: f32, cy: f32, r: f32, a0: f32, a1: f32, first: bool) {
    let alpha = ((a1 - a0) / 4.0).tan() * 4.0 / 3.0;
    let (s0, c0) = a0.sin_cos();
    let (s1, c1) = a1.sin_cos();
    let x0 = cx + r * c0; let y0 = cy + r * s0;
    let x3 = cx + r * c1; let y3 = cy + r * s1;
    let x1 = x0 - alpha * r * s0; let y1 = y0 + alpha * r * c0;
    let x2 = x3 + alpha * r * s1; let y2 = y3 - alpha * r * c1;
    if first { pb.move_to(x0, y0); }
    pb.cubic_to(x1, y1, x2, y2, x3, y3);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), false, 0.5, "work", false);
        assert_eq!(bytes.len(), (SIZE * SIZE * 4) as usize);
    }

    #[test]
    fn render_paused_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), true, 0.0, "work", false);
        assert_eq!(bytes.len(), (SIZE * SIZE * 4) as usize);
    }

    #[test]
    fn render_zero_progress_returns_correct_byte_count() {
        let bytes = render_tray_icon_rgba(&TrayColors::default(), false, 0.0, "work", false);
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
}
