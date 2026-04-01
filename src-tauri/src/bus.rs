/// Internal synchronous event bus.
///
/// Business-logic modules publish domain events here; subsystems (currently
/// achievements) subscribe to them.  The bus has no dependency on any
/// specific subscriber, so subscribers can be added or removed without
/// touching publisher code.
///
/// All subscribers are called **synchronously** on the publishing thread in
/// registration order.  Handlers must not block for long; spawn a thread or
/// Tokio task inside the handler for any heavy work.
use std::sync::{Arc, RwLock};
use tauri::AppHandle;

// ---------------------------------------------------------------------------
// Domain event vocabulary
// ---------------------------------------------------------------------------

/// Every notable domain event in the app.  Publishers use these variants;
/// subscribers pattern-match on them.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// The application process started.
    AppLaunched,

    /// A timer round ended (either completed naturally or skipped).
    /// Context flags are pre-gated to `false` for non-work rounds.
    SessionCompleted {
        round_type: String,
        was_skipped: bool,
        /// Seconds elapsed when the round ended.
        elapsed_secs: u32,
        /// Configured duration of the round in seconds.
        round_duration_secs: u32,
        /// Window was hidden to the tray when this session finished.
        in_tray: bool,
        /// Always-on-top was active during this session.
        always_on_top: bool,
        /// WebSocket server was enabled during this session.
        websocket_active: bool,
        /// All tick sounds and volume were off during this session.
        silent: bool,
        /// Window was in compact mode (either dimension < 300 logical px).
        compact: bool,
    },

    /// All session rows were deleted by the user.
    SessionsCleared,

    /// A settings key was written (key is the DB key name, e.g. "language").
    SettingsSaved { key: String },

    /// The active theme was changed (name is the theme's display name).
    ThemeApplied { name: String },

    /// The UI language was changed.
    LanguageChanged { language: String },

    /// The WebSocket server was enabled for the first time in this install.
    WebSocketEnabled,

    /// A WebSocket client sent a message.
    WebSocketMessage { msg_type: String },

    /// A global keyboard shortcut was fired.
    ShortcutUsed { action: String },

    /// A custom audio file was loaded for a cue slot.
    AudioCustomLoaded,

    /// A custom theme file was created or modified on disk.
    ThemeCreated,
}

// ---------------------------------------------------------------------------
// Event bus
// ---------------------------------------------------------------------------

type Handler = Box<dyn Fn(&AppEvent, &AppHandle) + Send + Sync>;

/// Synchronous, multi-subscriber event bus.
///
/// Create one `Arc<EventBus>`, register it as Tauri state, subscribe handlers
/// during setup, then call `publish` from any module that holds an `AppHandle`.
pub struct EventBus {
    handlers: RwLock<Vec<Handler>>,
}

impl EventBus {
    /// Create a new empty bus wrapped in an `Arc`.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            handlers: RwLock::new(Vec::new()),
        })
    }

    /// Register a handler.  Called only during app setup; uses a write lock.
    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(&AppEvent, &AppHandle) + Send + Sync + 'static,
    {
        self.handlers.write().unwrap().push(Box::new(handler));
    }

    /// Publish an event, calling every registered handler in registration order.
    /// Uses a read lock — multiple threads can publish concurrently.
    pub fn publish(&self, event: AppEvent, app: &AppHandle) {
        let handlers = self.handlers.read().unwrap();
        for handler in handlers.iter() {
            handler(&event, app);
        }
    }
}
