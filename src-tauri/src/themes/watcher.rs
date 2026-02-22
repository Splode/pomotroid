/// Directory watcher for `{app_data_dir}/themes/` using the `notify` crate.
///
/// Debounces filesystem events by 500 ms to avoid multiple reloads when a
/// text editor writes a file in multiple stages.
///
/// On a debounced event:
///   1. Re-scans the custom themes directory.
///   2. Re-builds the merged `Vec<Theme>` (bundled + custom).
///   3. Emits `themes:changed` with the new list to all frontend windows.
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

use super::{list_all, Theme};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Spawn a background thread that watches `themes_dir` for changes and emits
/// `themes:changed` on the `app` handle whenever the custom theme list changes.
///
/// Returns the `RecommendedWatcher` — drop it to stop watching.
/// The watcher must outlive the app; store it in Tauri managed state or a
/// `static` to prevent it from being dropped early.
pub fn spawn_watcher(
    app_data_dir: PathBuf,
    app: AppHandle,
) -> Option<RecommendedWatcher> {
    let themes_dir = app_data_dir.join("themes");

    // Ensure the custom themes directory exists.
    if let Err(e) = std::fs::create_dir_all(&themes_dir) {
        eprintln!("[themes/watcher] failed to create themes dir: {e}");
        return None;
    }

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let mut watcher = match RecommendedWatcher::new(tx, notify::Config::default()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[themes/watcher] failed to create watcher: {e}");
            return None;
        }
    };

    if let Err(e) = watcher.watch(&themes_dir, RecursiveMode::NonRecursive) {
        eprintln!("[themes/watcher] failed to watch {}: {e}", themes_dir.display());
        return None;
    }

    // Debounce thread.
    std::thread::Builder::new()
        .name("themes-watcher".to_string())
        .spawn(move || debounce_loop(rx, &app_data_dir, &app))
        .ok();

    Some(watcher)
}

// ---------------------------------------------------------------------------
// Debounce loop (runs on the themes-watcher thread)
// ---------------------------------------------------------------------------

const DEBOUNCE: Duration = Duration::from_millis(500);

fn debounce_loop(
    rx: mpsc::Receiver<notify::Result<Event>>,
    app_data_dir: &Path,
    app: &AppHandle,
) {
    while let Ok(first) = rx.recv() {
        // Log and ignore watcher errors.
        if let Err(e) = first {
            eprintln!("[themes/watcher] watch error: {e}");
            continue;
        }

        // Drain additional events that arrive within the debounce window.
        drain_within(&rx, DEBOUNCE);

        // Reload and emit.
        reload_and_emit(app_data_dir, app);
    }
}

/// Consume all events from `rx` that arrive within `window` ms of the first.
fn drain_within(rx: &mpsc::Receiver<notify::Result<Event>>, window: Duration) {
    let deadline = std::time::Instant::now() + window;
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match rx.recv_timeout(remaining) {
            Ok(_) => continue, // keep draining
            Err(_) => break,   // timeout or channel closed
        }
    }
}

/// Re-scan themes directory and emit `themes:changed` with the updated list.
fn reload_and_emit(app_data_dir: &Path, app: &AppHandle) {
    let themes: Vec<Theme> = list_all(app_data_dir);
    if let Err(e) = app.emit("themes:changed", &themes) {
        eprintln!("[themes/watcher] emit error: {e}");
    }
}
