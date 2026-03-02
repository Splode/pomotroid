/// Desktop notification dispatch.
///
/// Notification strings are constructed on the frontend (translated via
/// Paraglide) and passed to `dispatch` as plain `title` and `body` strings.
/// This keeps all i18n logic in the frontend; Rust is locale-agnostic.
///
/// On Linux, `tauri-plugin-notification` uses `notify-rust` / `zbus` which
/// fails to connect to the D-Bus session bus from within the Tauri process
/// context.  We spawn `notify-send` (libnotify-bin, installed by default on
/// Ubuntu/GNOME) as a subprocess instead — it inherits the correct session
/// environment and works reliably.
///
/// On macOS and Windows the Tauri plugin is used as normal.
use tauri::AppHandle;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Send a notification with the given title and body if enabled.
/// Called from the `notification_show` Tauri command.
pub fn show(app: &AppHandle, title: &str, body: &str) {
    dispatch(app, title, body);
}

// ---------------------------------------------------------------------------
// Platform dispatch
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
pub fn dispatch(_app: &AppHandle, title: &str, body: &str) {
    let _ = std::process::Command::new("notify-send")
        .args(["--app-name=Pomotroid", "--urgency=normal", "--expire-time=5000", title, body])
        .spawn();
}

#[cfg(not(target_os = "linux"))]
pub fn dispatch(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app.notification().builder().title(title).body(body).show() {
        log::warn!("[notifications] failed to send notification: {e}");
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn show_fn_exists() {
        // Compile-time check: `show` is callable with the right signature.
        // Runtime dispatch is not testable without an AppHandle.
        let _: fn(&tauri::AppHandle, &str, &str) = super::show;
    }
}
