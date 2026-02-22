/// Rust audio playback via rodio.
///
/// All four audio assets are embedded at compile time so they are available
/// even when the app is hidden to the system tray.
///
/// Architecture:
///   A dedicated OS thread owns the `OutputStream` (not `Send` on macOS).
///   `AudioManager` communicates with that thread via a `SyncSender`.
///   Tauri manages `Arc<AudioManager>` as state; it is `Send + Sync`.
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use rodio::{Decoder, OutputStream, Sink};

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

struct PlayRequest {
    cue: AudioCue,
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
        let _ = self.tx.try_send(PlayRequest { cue, volume });
    }

    /// Returns true if tick sounds are enabled for the given round type string.
    pub fn tick_enabled_for(&self, round_type: &str) -> bool {
        let s = self.settings.lock().unwrap();
        match round_type {
            "work" => s.tick_sounds_work,
            _ => s.tick_sounds_break,
        }
    }
}

// ---------------------------------------------------------------------------
// Audio thread
// ---------------------------------------------------------------------------

fn audio_thread(rx: mpsc::Receiver<PlayRequest>) {
    // OutputStream must stay alive for the lifetime of this thread.
    let (_stream, handle) = match OutputStream::try_default() {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("[audio] failed to open output stream: {e}");
            return;
        }
    };

    while let Ok(req) = rx.recv() {
        let bytes: &'static [u8] = match req.cue {
            AudioCue::WorkAlert => ALERT_WORK,
            AudioCue::ShortBreakAlert => ALERT_SHORT_BREAK,
            AudioCue::LongBreakAlert => ALERT_LONG_BREAK,
            AudioCue::Tick => TICK,
        };

        let source = match Decoder::new(Cursor::new(bytes)) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("[audio] decode error: {e}");
                continue;
            }
        };

        match Sink::try_new(&handle) {
            Ok(sink) => {
                sink.set_volume(req.volume);
                sink.append(source);
                sink.detach(); // let it finish without blocking
            }
            Err(e) => eprintln!("[audio] sink error: {e}"),
        }
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
        };
        assert!(mgr.tick_enabled_for("work"));
        assert!(!mgr.tick_enabled_for("short-break"));
        assert!(!mgr.tick_enabled_for("long-break"));
    }
}
