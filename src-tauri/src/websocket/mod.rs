/// Optional opt-in WebSocket server via tokio + axum.
///
/// Binds to `127.0.0.1:{port}` (localhost only — never all interfaces).
///
/// Protocol:
///   Client → Server: `{ "type": "getState" }`
///   Server → Client: `{ "type": "state",       "payload": TimerSnapshot }`              (getState response)
///                    `{ "type": "started",      "payload": { "total_secs": u32 } }`     (broadcast)
///                    `{ "type": "roundChange",  "payload": TimerSnapshot }`              (broadcast)
///                    `{ "type": "paused",       "payload": { "elapsed_secs": u32 } }`   (broadcast)
///                    `{ "type": "resumed",      "payload": { "elapsed_secs": u32 } }`   (broadcast)
///                    `{ "type": "reset" }`                                               (broadcast)
///                    `{ "type": "error",        "message": "..." }`                     (startup failure)
///
/// Lifecycle:
///   - `start(port, app)` spawns a Tokio task; sets running handle in `WsState`.
///   - `stop()` aborts the task.
///   - On port conflict: emits `websocket:error` Tauri event instead of panicking.
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State as AxumState},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use tauri::{AppHandle, Emitter, Manager};
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

use crate::timer::{TimerController, TimerSnapshot};

// ---------------------------------------------------------------------------
// Broadcast channel payload
// ---------------------------------------------------------------------------

/// Shared payload for events that carry elapsed time.
#[derive(Clone, serde::Serialize)]
pub struct ElapsedPayload {
    pub elapsed_secs: u32,
}

/// Payload for the `started` event.
#[derive(Clone, serde::Serialize)]
pub struct StartedPayload {
    pub total_secs: u32,
}

/// Events broadcast to all connected WebSocket clients.
#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WsEvent {
    Started { payload: StartedPayload },
    RoundChange { payload: TimerSnapshot },
    Paused { payload: ElapsedPayload },
    Resumed { payload: ElapsedPayload },
    Reset,
}

// ---------------------------------------------------------------------------
// Server state shared between connections
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct ServerState {
    broadcast_tx: broadcast::Sender<WsEvent>,
    snapshot_fn: Arc<dyn Fn() -> Option<TimerSnapshot> + Send + Sync>,
}

// ---------------------------------------------------------------------------
// Tauri-managed WebSocket state
// ---------------------------------------------------------------------------

pub struct WsState {
    task: tokio::sync::Mutex<Option<JoinHandle<()>>>,
    pub broadcast_tx: broadcast::Sender<WsEvent>,
}

impl WsState {
    pub fn new() -> Arc<Self> {
        let (broadcast_tx, _) = broadcast::channel(64);
        Arc::new(Self {
            task: tokio::sync::Mutex::new(None),
            broadcast_tx,
        })
    }
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

/// Start the WebSocket server on `127.0.0.1:{port}`.
///
/// Emits `websocket:error` if the port is already in use.
/// No-ops if already running (call `stop` first to change port).
pub async fn start(port: u16, app: AppHandle, state: &Arc<WsState>) {
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            log::error!("[ws] failed to bind {addr}: {e}");
            let _ = app.emit(
                "websocket:error",
                serde_json::json!({ "message": e.to_string(), "port": port }),
            );
            return;
        }
    };

    let app_clone = app.clone();
    let snapshot_fn = Arc::new(move || {
        app_clone
            .try_state::<TimerController>()
            .map(|t| t.get_snapshot())
    }) as Arc<dyn Fn() -> Option<TimerSnapshot> + Send + Sync>;

    let server_state = ServerState {
        broadcast_tx: state.broadcast_tx.clone(),
        snapshot_fn,
    };

    let router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(server_state);

    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            log::error!("[ws] server error: {e}");
        }
    });

    *state.task.lock().await = Some(handle);
    log::info!("[ws] listening on ws://127.0.0.1:{port}/ws");
}

/// Stop the WebSocket server (aborts the task).
pub async fn stop(state: &Arc<WsState>) {
    if let Some(handle) = state.task.lock().await.take() {
        handle.abort();
    }
}

// ---------------------------------------------------------------------------
// WebSocket handler
// ---------------------------------------------------------------------------

async fn ws_handler(
    ws: WebSocketUpgrade,
    AxumState(state): AxumState<ServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: ServerState) {
    log::debug!("[ws] client connected");
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.broadcast_tx.subscribe();
    let (direct_tx, mut direct_rx) = mpsc::unbounded_channel::<String>();

    // Task: forward broadcast events and direct replies to this client.
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                result = rx.recv() => {
                    let Ok(event) = result else { break };
                    let Ok(json) = serde_json::to_string(&event) else { continue };
                    if sender.send(Message::Text(json.into())).await.is_err() { break }
                }
                msg = direct_rx.recv() => {
                    let Some(json) = msg else { break };
                    if sender.send(Message::Text(json.into())).await.is_err() { break }
                }
            }
        }
    });

    // Main loop: handle incoming messages from this client.
    let snapshot_fn = Arc::clone(&state.snapshot_fn);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let snapshot = (snapshot_fn)();
                    handle_client_message(&text, snapshot, &direct_tx).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // If either task finishes, abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
    log::debug!("[ws] client disconnected");
}

async fn handle_client_message(
    text: &str,
    snapshot: Option<TimerSnapshot>,
    direct_tx: &mpsc::UnboundedSender<String>,
) {
    let Ok(msg) = serde_json::from_str::<serde_json::Value>(text) else {
        return;
    };

    if let Some("getState") = msg.get("type").and_then(|t| t.as_str()) {
        if let Some(snap) = snapshot {
            let json = serde_json::to_string(
                &serde_json::json!({ "type": "state", "payload": snap })
            ).unwrap_or_default();
            let _ = direct_tx.send(json);
        }
    }
}

// ---------------------------------------------------------------------------
// Public API for broadcasting from the timer event listener
// ---------------------------------------------------------------------------

/// Broadcast a `started` event to all connected WebSocket clients.
pub fn broadcast_started(state: &Arc<WsState>, total_secs: u32) {
    let _ = state.broadcast_tx.send(WsEvent::Started { payload: StartedPayload { total_secs } });
}

/// Broadcast a `roundChange` event to all connected WebSocket clients.
pub fn broadcast_round_change(state: &Arc<WsState>, snapshot: TimerSnapshot) {
    let _ = state.broadcast_tx.send(WsEvent::RoundChange { payload: snapshot });
}

/// Broadcast a `paused` event to all connected WebSocket clients.
pub fn broadcast_paused(state: &Arc<WsState>, elapsed_secs: u32) {
    let _ = state.broadcast_tx.send(WsEvent::Paused { payload: ElapsedPayload { elapsed_secs } });
}

/// Broadcast a `resumed` event to all connected WebSocket clients.
pub fn broadcast_resumed(state: &Arc<WsState>, elapsed_secs: u32) {
    let _ = state.broadcast_tx.send(WsEvent::Resumed { payload: ElapsedPayload { elapsed_secs } });
}

/// Broadcast a `reset` event to all connected WebSocket clients.
pub fn broadcast_reset(state: &Arc<WsState>) {
    let _ = state.broadcast_tx.send(WsEvent::Reset);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_snapshot() -> TimerSnapshot {
        TimerSnapshot {
            round_type: "work".into(),
            previous_round_type: "short-break".into(),
            elapsed_secs: 60,
            total_secs: 1500,
            is_running: true,
            is_paused: false,
            work_round_number: 1,
            work_rounds_total: 4,
            session_work_count: 1,
        }
    }

    // -- existing serialization tests --

    #[test]
    fn ws_state_can_be_created() {
        let state = WsState::new();
        assert_eq!(state.broadcast_tx.receiver_count(), 0);
    }

    #[test]
    fn ws_event_serializes_correctly() {
        let event = WsEvent::RoundChange { payload: make_snapshot() };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"roundChange\""));
        assert!(json.contains("\"elapsed_secs\":60"));
    }

    #[test]
    fn ws_event_started_serializes_correctly() {
        let event = WsEvent::Started { payload: StartedPayload { total_secs: 1500 } };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"started\""));
        assert!(json.contains("\"total_secs\":1500"));
    }

    #[test]
    fn ws_event_paused_serializes_correctly() {
        let event = WsEvent::Paused { payload: ElapsedPayload { elapsed_secs: 300 } };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"paused\""));
        assert!(json.contains("\"elapsed_secs\":300"));
    }

    #[test]
    fn ws_event_resumed_serializes_correctly() {
        let event = WsEvent::Resumed { payload: ElapsedPayload { elapsed_secs: 180 } };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"resumed\""));
        assert!(json.contains("\"elapsed_secs\":180"));
    }

    #[test]
    fn ws_event_reset_serializes_correctly() {
        let event = WsEvent::Reset;
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#"{"type":"reset"}"#);
    }

    // -- handle_client_message unit tests --

    #[tokio::test]
    async fn getstate_sends_state_reply() {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        handle_client_message(r#"{"type":"getState"}"#, Some(make_snapshot()), &tx).await;
        let reply = rx.try_recv().expect("expected a reply on direct channel");
        let val: serde_json::Value = serde_json::from_str(&reply).unwrap();
        assert_eq!(val["type"], "state");
        assert_eq!(val["payload"]["elapsed_secs"], 60);
        assert_eq!(val["payload"]["total_secs"], 1500);
        assert_eq!(val["payload"]["round_type"], "work");
        assert_eq!(val["payload"]["is_running"], true);
    }

    #[tokio::test]
    async fn getstate_no_timer_state_sends_nothing() {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        handle_client_message(r#"{"type":"getState"}"#, None, &tx).await;
        assert!(rx.try_recv().is_err(), "expected no reply when snapshot is None");
    }

    #[tokio::test]
    async fn malformed_json_is_silently_ignored() {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        handle_client_message("not valid json {{{", Some(make_snapshot()), &tx).await;
        assert!(rx.try_recv().is_err(), "expected no reply for malformed JSON");
    }

    #[tokio::test]
    async fn unknown_message_type_is_ignored() {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        handle_client_message(r#"{"type":"unknownCommand"}"#, Some(make_snapshot()), &tx).await;
        assert!(rx.try_recv().is_err(), "expected no reply for unknown message type");
    }

    #[tokio::test]
    async fn reply_uses_direct_channel_not_broadcast() {
        let (broadcast_tx, _) = broadcast::channel::<WsEvent>(8);
        let (direct_tx, mut direct_rx) = mpsc::unbounded_channel::<String>();
        handle_client_message(r#"{"type":"getState"}"#, Some(make_snapshot()), &direct_tx).await;
        // Reply appeared on the direct channel
        assert!(direct_rx.try_recv().is_ok(), "expected reply on direct channel");
        // Nothing sent to the broadcast channel
        assert_eq!(broadcast_tx.receiver_count(), 0);
    }

    // -- network-level integration test --

    #[tokio::test]
    async fn integration_getstate_round_trip() {
        use axum::Router;
        use tokio::net::TcpListener;
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::Message as TungMessage;
        use futures_util::{SinkExt, StreamExt};

        let snap = make_snapshot();
        let snap_clone = snap.clone();
        let snapshot_fn = Arc::new(move || Some(snap_clone.clone()))
            as Arc<dyn Fn() -> Option<TimerSnapshot> + Send + Sync>;

        let (broadcast_tx, _) = broadcast::channel::<WsEvent>(8);
        let server_state = ServerState { broadcast_tx, snapshot_fn };

        let router = Router::new()
            .route("/ws", get(ws_handler))
            .with_state(server_state);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let url = format!("ws://127.0.0.1:{port}/ws");
        let (mut ws, _) = connect_async(&url).await.expect("WebSocket connect failed");

        ws.send(TungMessage::Text(r#"{"type":"getState"}"#.into())).await.unwrap();

        let msg = ws.next().await.expect("expected a message").unwrap();
        let TungMessage::Text(text) = msg else { panic!("expected text frame") };
        let val: serde_json::Value = serde_json::from_str(&text).unwrap();

        assert_eq!(val["type"], "state", "response type should be 'state'");
        assert_eq!(val["payload"]["elapsed_secs"], snap.elapsed_secs);
        assert_eq!(val["payload"]["total_secs"], snap.total_secs);
        assert_eq!(val["payload"]["round_type"], snap.round_type);
    }
}
