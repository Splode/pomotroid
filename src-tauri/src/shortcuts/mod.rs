/// Global shortcut registration via tauri-plugin-global-shortcut.
///
/// Default shortcuts:
///   - toggle: Ctrl+F1 (start/pause/resume)
///   - reset:  Ctrl+F2
///   - skip:   Ctrl+F3
///
/// All shortcuts are unregistered before re-registering, so calling
/// `register_all` is idempotent.
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

use crate::settings::Settings;
use crate::timer::TimerController;

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parse a shortcut string like "Control+F1" into a `Shortcut`.
///
/// Supported modifiers (case-insensitive): Control, Shift, Alt, Super/Meta.
/// Supported keys: F1–F12, letter keys, digit keys, named keys.
/// Returns `None` if parsing fails — the caller falls back to the default.
pub fn parse_shortcut(s: &str) -> Option<Shortcut> {
    let parts: Vec<&str> = s.split('+').map(str::trim).collect();
    if parts.is_empty() {
        return None;
    }

    let key_str = parts.last()?;
    let modifier_strs = &parts[..parts.len() - 1];

    let mut mods = Modifiers::empty();
    for m in modifier_strs {
        match m.to_ascii_lowercase().as_str() {
            "control" | "ctrl" => mods |= Modifiers::CONTROL,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" | "option" => mods |= Modifiers::ALT,
            "super" | "meta" | "cmd" | "command" => mods |= Modifiers::SUPER,
            _ => return None, // unknown modifier
        }
    }

    let code = parse_code(key_str)?;
    Some(Shortcut::new(Some(mods), code))
}

fn parse_code(key: &str) -> Option<Code> {
    Some(match key.to_ascii_lowercase().as_str() {
        "f1"  => Code::F1,  "f2"  => Code::F2,  "f3"  => Code::F3,
        "f4"  => Code::F4,  "f5"  => Code::F5,  "f6"  => Code::F6,
        "f7"  => Code::F7,  "f8"  => Code::F8,  "f9"  => Code::F9,
        "f10" => Code::F10, "f11" => Code::F11, "f12" => Code::F12,
        "a" => Code::KeyA, "b" => Code::KeyB, "c" => Code::KeyC,
        "d" => Code::KeyD, "e" => Code::KeyE, "f" => Code::KeyF,
        "g" => Code::KeyG, "h" => Code::KeyH, "i" => Code::KeyI,
        "j" => Code::KeyJ, "k" => Code::KeyK, "l" => Code::KeyL,
        "m" => Code::KeyM, "n" => Code::KeyN, "o" => Code::KeyO,
        "p" => Code::KeyP, "q" => Code::KeyQ, "r" => Code::KeyR,
        "s" => Code::KeyS, "t" => Code::KeyT, "u" => Code::KeyU,
        "v" => Code::KeyV, "w" => Code::KeyW, "x" => Code::KeyX,
        "y" => Code::KeyY, "z" => Code::KeyZ,
        "0" => Code::Digit0, "1" => Code::Digit1, "2" => Code::Digit2,
        "3" => Code::Digit3, "4" => Code::Digit4, "5" => Code::Digit5,
        "6" => Code::Digit6, "7" => Code::Digit7, "8" => Code::Digit8,
        "9" => Code::Digit9,
        "space" => Code::Space,
        "enter" | "return" => Code::Enter,
        "escape" | "esc" => Code::Escape,
        "tab" => Code::Tab,
        "backspace" => Code::Backspace,
        "delete" => Code::Delete,
        "arrowup" | "up" => Code::ArrowUp,
        "arrowdown" | "down" => Code::ArrowDown,
        "arrowleft" | "left" => Code::ArrowLeft,
        "arrowright" | "right" => Code::ArrowRight,
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

/// Unregister all current shortcuts and register the three actions defined
/// in `settings`. Silent on individual parse/register failures.
pub fn register_all(app: &AppHandle, settings: &Settings) {
    let gsm = app.global_shortcut();

    // Unregister everything first to avoid stale bindings.
    let _ = gsm.unregister_all();

    let shortcuts = [
        (settings.shortcut_toggle.as_str(), ShortcutAction::Toggle),
        (settings.shortcut_reset.as_str(),  ShortcutAction::Reset),
        (settings.shortcut_skip.as_str(),   ShortcutAction::Skip),
    ];

    for (key_str, action) in shortcuts {
        let Some(shortcut) = parse_shortcut(key_str) else {
            eprintln!("[shortcuts] could not parse shortcut '{key_str}'");
            continue;
        };

        let app_clone = app.clone();
        if let Err(e) = gsm.on_shortcut(shortcut, move |_app, _shortcut, _event| {
            fire_action(&app_clone, action);
        }) {
            eprintln!("[shortcuts] failed to register '{key_str}': {e}");
        }
    }
}

/// Unregister all global shortcuts (called on app exit or when disabled).
pub fn unregister_all(app: &AppHandle) {
    let _ = app.global_shortcut().unregister_all();
}

// ---------------------------------------------------------------------------
// Action dispatch
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum ShortcutAction {
    Toggle,
    Reset,
    Skip,
}

fn fire_action(app: &AppHandle, action: ShortcutAction) {
    let Some(timer) = app.try_state::<TimerController>() else { return };
    match action {
        ShortcutAction::Toggle => timer.toggle(),
        ShortcutAction::Reset  => timer.reset(),
        ShortcutAction::Skip   => timer.skip(),
    }
}

// ---------------------------------------------------------------------------
// IPC command wiring
// ---------------------------------------------------------------------------

/// Re-register shortcuts from a new configuration.
/// Called by the frontend via `settings_set` when shortcut keys change.
pub fn apply_new_settings(app: &AppHandle, settings: &Settings) {
    register_all(app, settings);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ctrl_f1() {
        let s = parse_shortcut("Control+F1");
        assert!(s.is_some(), "Control+F1 must parse");
    }

    #[test]
    fn parse_ctrl_f2() {
        assert!(parse_shortcut("Control+F2").is_some());
    }

    #[test]
    fn parse_ctrl_f3() {
        assert!(parse_shortcut("Control+F3").is_some());
    }

    #[test]
    fn parse_super_modifier() {
        assert!(parse_shortcut("Super+F1").is_some(), "Super modifier must parse");
    }

    #[test]
    fn parse_multi_modifier() {
        assert!(parse_shortcut("Control+Shift+F5").is_some());
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert!(parse_shortcut("").is_none());
        assert!(parse_shortcut("NotAModifier+F1").is_none());
        assert!(parse_shortcut("Control+NotAKey").is_none());
    }
}
