/// Rust audio playback via rodio.
///
/// All four audio assets are embedded at compile time so they are available
/// even when the app is hidden to the system tray.
///
/// Architecture:
///   A dedicated OS thread owns the `OutputStream` (not `Send` on macOS).
///   `AudioManager` communicates with that thread via a `SyncSender`.
///   Tauri manages `Arc<AudioManager>` as state; it is `Send + Sync`.
///
/// Custom audio:
///   Per-cue custom files are stored in `{app_data_dir}/audio/` with fixed
///   stems (`custom_work_alert`, `custom_short_break_alert`,
///   `custom_long_break_alert`). The audio thread tries the custom file first
///   and falls back to the embedded bytes if the file is missing or unreadable.
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use rodio::{Decoder, DeviceSinkBuilder, Player};

use crate::settings::Settings;

// ---------------------------------------------------------------------------
// Embedded audio assets
// ---------------------------------------------------------------------------

const ALERT_WORK: &[u8] = include_bytes!("../../../static/audio/alert-work.mp3");
const ALERT_SHORT_BREAK: &[u8] = include_bytes!("../../../static/audio/alert-short-break.mp3");
const ALERT_LONG_BREAK: &[u8] = include_bytes!("../../../static/audio/alert-long-break.mp3");
const TICK: &[u8] = include_bytes!("../../../static/audio/tick.mp3");

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum AudioCue {
    WorkAlert,
    ShortBreakAlert,
    LongBreakAlert,
    Tick,
}

/// Paths to currently active custom audio files (one per alert cue).
/// `None` means the embedded default is used.
#[derive(Default)]
pub struct CustomAudioPaths {
    pub work_alert: Option<PathBuf>,
    pub short_break_alert: Option<PathBuf>,
    pub long_break_alert: Option<PathBuf>,
}

/// Serialisable snapshot of custom audio file names (sent to the frontend).
#[derive(serde::Serialize)]
pub struct CustomAudioInfo {
    pub work_alert: Option<String>,
    pub short_break_alert: Option<String>,
    pub long_break_alert: Option<String>,
}

struct PlayRequest {
    cue: AudioCue,
    /// Resolved custom file path, if one is configured for this cue.
    custom_path: Option<PathBuf>,
    volume: f32,
}

/// Settings subset relevant to the audio engine.
#[derive(Clone)]
struct AudioSettings {
    volume: f32,
    tick_sounds_work: bool,
    tick_sounds_break: bool,
}

impl From<&Settings> for AudioSettings {
    fn from(s: &Settings) -> Self {
        Self {
            volume: s.volume,
            tick_sounds_work: s.tick_sounds_during_work,
            tick_sounds_break: s.tick_sounds_during_break,
        }
    }
}

/// Thread-safe audio manager. Register as `Arc<AudioManager>` in Tauri state.
pub struct AudioManager {
    tx: mpsc::SyncSender<PlayRequest>,
    settings: Arc<Mutex<AudioSettings>>,
    pub custom_paths: Arc<Mutex<CustomAudioPaths>>,
}

impl AudioManager {
    /// Spawn the audio thread and return an `Arc<AudioManager>`.
    /// If the system audio device is unavailable, returns `None` (app still works, just silent).
    pub fn new(initial: &Settings) -> Option<Arc<Self>> {
        let (tx, rx) = mpsc::sync_channel::<PlayRequest>(8);

        std::thread::Builder::new()
            .name("audio".to_string())
            .spawn(move || audio_thread(rx))
            .ok()?;

        Some(Arc::new(Self {
            tx,
            settings: Arc::new(Mutex::new(AudioSettings::from(initial))),
            custom_paths: Arc::new(Mutex::new(CustomAudioPaths::default())),
        }))
    }

    /// Update volume and tick-sound settings from a new `Settings` snapshot.
    pub fn apply_settings(&self, s: &Settings) {
        *self.settings.lock().unwrap() = AudioSettings::from(s);
    }

    /// Play the given cue at the current stored volume.
    /// Non-blocking: drops the request if the channel is full.
    pub fn play_cue(&self, cue: AudioCue) {
        let volume = self.settings.lock().unwrap().volume;
        if volume <= 0.0 {
            return;
        }
        // Resolve custom path (Tick always uses the embedded sound).
        let custom_path = {
            let paths = self.custom_paths.lock().unwrap();
            match cue {
                AudioCue::WorkAlert => paths.work_alert.clone(),
                AudioCue::ShortBreakAlert => paths.short_break_alert.clone(),
                AudioCue::LongBreakAlert => paths.long_break_alert.clone(),
                AudioCue::Tick => None,
            }
        };
        let _ = self.tx.try_send(PlayRequest { cue, custom_path, volume });
    }

    /// Returns true if tick sounds are enabled for the given round type string.
    pub fn tick_enabled_for(&self, round_type: &str) -> bool {
        let s = self.settings.lock().unwrap();
        match round_type {
            "work" => s.tick_sounds_work,
            _ => s.tick_sounds_break,
        }
    }

    /// Set a custom file path for the given cue slot.
    /// `cue` must be `"work_alert"`, `"short_break_alert"`, or `"long_break_alert"`.
    pub fn set_custom_path(&self, cue: &str, path: PathBuf) {
        let mut paths = self.custom_paths.lock().unwrap();
        match cue {
            "work_alert" => paths.work_alert = Some(path),
            "short_break_alert" => paths.short_break_alert = Some(path),
            "long_break_alert" => paths.long_break_alert = Some(path),
            _ => {}
        }
    }

    /// Remove the custom path for the given cue slot, reverting to the built-in sound.
    pub fn clear_custom_path(&self, cue: &str) {
        let mut paths = self.custom_paths.lock().unwrap();
        match cue {
            "work_alert" => paths.work_alert = None,
            "short_break_alert" => paths.short_break_alert = None,
            "long_break_alert" => paths.long_break_alert = None,
            _ => {}
        }
    }

    /// Return the display names (file names only) of any configured custom files.
    pub fn get_custom_info(&self) -> CustomAudioInfo {
        let paths = self.custom_paths.lock().unwrap();
        let name = |p: &Option<PathBuf>| -> Option<String> {
            p.as_ref()
                .and_then(|pb| pb.file_name())
                .and_then(|n| n.to_str())
                .map(String::from)
        };
        CustomAudioInfo {
            work_alert: name(&paths.work_alert),
            short_break_alert: name(&paths.short_break_alert),
            long_break_alert: name(&paths.long_break_alert),
        }
    }
}

// ---------------------------------------------------------------------------
// Startup helper — scan disk for previously saved custom files
// ---------------------------------------------------------------------------

/// Fixed file stems used when copying custom audio files into the config dir.
pub const STEM_WORK: &str = "custom_work_alert";
pub const STEM_SHORT: &str = "custom_short_break_alert";
pub const STEM_LONG: &str = "custom_long_break_alert";

/// Scan `audio_dir` for any saved custom audio files and return the paths.
pub fn find_custom_files(audio_dir: &Path) -> CustomAudioPaths {
    let find = |stem: &str| -> Option<PathBuf> {
        let entries = std::fs::read_dir(audio_dir).ok()?;
        entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .find(|p| p.file_stem().and_then(|s| s.to_str()) == Some(stem))
    };
    CustomAudioPaths {
        work_alert: find(STEM_WORK),
        short_break_alert: find(STEM_SHORT),
        long_break_alert: find(STEM_LONG),
    }
}

// ---------------------------------------------------------------------------
// Audio thread
// ---------------------------------------------------------------------------

fn audio_thread(rx: mpsc::Receiver<PlayRequest>) {
    // MixerDeviceSink must stay alive for the lifetime of this thread.
    let device_sink = match DeviceSinkBuilder::open_default_sink() {
        Ok(s) => s,
        Err(e) => {
            log::warn!("[audio] failed to open output stream: {e}");
            return;
        }
    };

    while let Ok(req) = rx.recv() {
        let player = Player::connect_new(device_sink.mixer());
        player.set_volume(req.volume);

        // Try the custom file first; fall back to the embedded asset on any error.
        let used_custom = if let Some(path) = req.custom_path {
            match std::fs::File::open(&path).map(std::io::BufReader::new) {
                Ok(reader) => match Decoder::new(reader) {
                    Ok(source) => { player.append(source); true }
                    Err(e) => {
                        log::warn!("[audio] decode error for {path:?}: {e}");
                        false
                    }
                },
                Err(e) => {
                    log::warn!("[audio] cannot open {path:?}: {e}");
                    false
                }
            }
        } else {
            false
        };

        if !used_custom {
            let bytes: &'static [u8] = match req.cue {
                AudioCue::WorkAlert => ALERT_WORK,
                AudioCue::ShortBreakAlert => ALERT_SHORT_BREAK,
                AudioCue::LongBreakAlert => ALERT_LONG_BREAK,
                AudioCue::Tick => TICK,
            };
            match Decoder::new(Cursor::new(bytes)) {
                Ok(source) => player.append(source),
                Err(e) => log::warn!("[audio] embedded decode error: {e}"),
            }
        }

        player.detach(); // let it finish without blocking
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_cue_bytes_are_non_empty() {
        assert!(!ALERT_WORK.is_empty());
        assert!(!ALERT_SHORT_BREAK.is_empty());
        assert!(!ALERT_LONG_BREAK.is_empty());
        assert!(!TICK.is_empty());
    }

    #[test]
    fn tick_enabled_for_round_types() {
        let settings = Settings {
            tick_sounds_during_work: true,
            tick_sounds_during_break: false,
            ..Settings::default()
        };
        let mgr = AudioManager {
            tx: mpsc::sync_channel(1).0,
            settings: Arc::new(Mutex::new(AudioSettings::from(&settings))),
            custom_paths: Arc::new(Mutex::new(CustomAudioPaths::default())),
        };
        assert!(mgr.tick_enabled_for("work"));
        assert!(!mgr.tick_enabled_for("short-break"));
        assert!(!mgr.tick_enabled_for("long-break"));
    }

    #[test]
    fn custom_paths_set_and_clear() {
        let settings = Settings::default();
        let mgr = AudioManager {
            tx: mpsc::sync_channel(1).0,
            settings: Arc::new(Mutex::new(AudioSettings::from(&settings))),
            custom_paths: Arc::new(Mutex::new(CustomAudioPaths::default())),
        };
        mgr.set_custom_path("work_alert", PathBuf::from("/tmp/test.mp3"));
        assert_eq!(
            mgr.custom_paths.lock().unwrap().work_alert,
            Some(PathBuf::from("/tmp/test.mp3"))
        );
        mgr.clear_custom_path("work_alert");
        assert!(mgr.custom_paths.lock().unwrap().work_alert.is_none());
    }
}
