/// Desktop notification dispatch.
///
/// Fires at the end of each Pomodoro round when `notifications_enabled` is true.
/// Called from the timer event listener on `timer:round-change`.
///
/// On Linux, `tauri-plugin-notification` uses `notify-rust` / `zbus` which
/// fails to connect to the D-Bus session bus from within the Tauri process
/// context.  We spawn `notify-send` (libnotify-bin, installed by default on
/// Ubuntu/GNOME) as a subprocess instead — it inherits the correct session
/// environment and works reliably.
///
/// On macOS and Windows the Tauri plugin is used as normal.
use tauri::AppHandle;

use crate::timer::sequence::RoundType;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Send a round-transition notification if notifications are enabled.
///
/// `next_round` is the round type that is *starting* (after the completed one).
pub fn notify_round_change(app: &AppHandle, next_round: RoundType, enabled: bool) {
    if !enabled {
        return;
    }

    let (title, body) = round_notification_text(next_round);
    dispatch(app, title, body);
}

// ---------------------------------------------------------------------------
// Platform dispatch
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
fn dispatch(_app: &AppHandle, title: &str, body: &str) {
    let _ = std::process::Command::new("notify-send")
        .args(["--app-name=Pomotroid", "--urgency=normal", "--expire-time=5000", title, body])
        .spawn();
}

#[cfg(not(target_os = "linux"))]
fn dispatch(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app.notification().builder().title(title).body(body).show() {
        eprintln!("[notifications] failed to send notification: {e}");
    }
}

// ---------------------------------------------------------------------------
// Notification content
// ---------------------------------------------------------------------------

fn round_notification_text(round: RoundType) -> (&'static str, &'static str) {
    match round {
        RoundType::Work => (
            "Break over — focus time!",
            "Time to get back to work. You've got this!",
        ),
        RoundType::ShortBreak => (
            "Work round complete!",
            "Take a short break. Stretch, breathe, relax.",
        ),
        RoundType::LongBreak => (
            "Work session done!",
            "You've earned a long break. Step away and recharge.",
        ),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_text_for_each_round() {
        let (t, b) = round_notification_text(RoundType::Work);
        assert!(!t.is_empty() && !b.is_empty());

        let (t, b) = round_notification_text(RoundType::ShortBreak);
        assert!(!t.is_empty() && !b.is_empty());

        let (t, b) = round_notification_text(RoundType::LongBreak);
        assert!(!t.is_empty() && !b.is_empty());
    }
}
