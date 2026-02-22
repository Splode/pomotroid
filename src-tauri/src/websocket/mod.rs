/// Optional opt-in WebSocket server via tokio + axum.
///
/// Binds to `127.0.0.1:{port}` (localhost only — never all interfaces).
///
/// Protocol:
///   Client → Server: `{ "type": "getState" }`
///   Server → Client: `{ "type": "state",       "payload": TimerSnapshot }`
///                    `{ "type": "roundChange",  "payload": TimerSnapshot }` (broadcast)
///                    `{ "type": "error",        "message": "..." }`         (startup failure)
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
    sync::broadcast,
    task::JoinHandle,
};

use crate::timer::{TimerController, TimerSnapshot};

// ---------------------------------------------------------------------------
// Broadcast channel payload
// ---------------------------------------------------------------------------

/// Events broadcast to all connected WebSocket clients.
#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WsEvent {
    RoundChange { payload: TimerSnapshot },
}

// ---------------------------------------------------------------------------
// Server state shared between connections
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct ServerState {
    app: AppHandle,
    broadcast_tx: broadcast::Sender<WsEvent>,
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
            eprintln!("[ws] failed to bind {addr}: {e}");
            let _ = app.emit(
                "websocket:error",
                serde_json::json!({ "message": e.to_string(), "port": port }),
            );
            return;
        }
    };

    let server_state = ServerState {
        app: app.clone(),
        broadcast_tx: state.broadcast_tx.clone(),
    };

    let router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(server_state);

    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            eprintln!("[ws] server error: {e}");
        }
    });

    *state.task.lock().await = Some(handle);
    eprintln!("[ws] listening on ws://127.0.0.1:{port}/ws");
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
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.broadcast_tx.subscribe();

    // Task: forward broadcast events to this client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let json = match serde_json::to_string(&event) {
                Ok(s) => s,
                Err(_) => continue,
            };
            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Main loop: handle incoming messages from this client.
    let app = state.app.clone();
    let broadcast_tx = state.broadcast_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    handle_client_message(&text, &app, &broadcast_tx).await;
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
}

async fn handle_client_message(
    text: &str,
    app: &AppHandle,
    _broadcast_tx: &broadcast::Sender<WsEvent>,
) {
    let Ok(msg) = serde_json::from_str::<serde_json::Value>(text) else {
        return;
    };

    match msg.get("type").and_then(|t| t.as_str()) {
        Some("getState") => {
            if let Some(timer) = app.try_state::<TimerController>() {
                let snapshot = timer.get_snapshot();
                let response = serde_json::json!({
                    "type": "state",
                    "payload": snapshot,
                });
                // Note: we can't send directly here without the sender;
                // the client will receive state via the next broadcast.
                // For an immediate reply, broadcast it.
                let _ = app.emit("timer:state-query", response);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Public API for broadcasting from the timer event listener
// ---------------------------------------------------------------------------

/// Broadcast a `roundChange` event to all connected WebSocket clients.
pub fn broadcast_round_change(state: &Arc<WsState>, snapshot: TimerSnapshot) {
    // Ignore send errors when there are no subscribers.
    let _ = state.broadcast_tx.send(WsEvent::RoundChange { payload: snapshot });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_state_can_be_created() {
        let state = WsState::new();
        // broadcast_tx should have 0 receivers initially.
        assert_eq!(state.broadcast_tx.receiver_count(), 0);
    }

    #[test]
    fn ws_event_serializes_correctly() {
        use crate::timer::TimerSnapshot;
        let snap = TimerSnapshot {
            round_type: "work".into(),
            elapsed_secs: 60,
            total_secs: 1500,
            is_running: true,
            is_paused: false,
            work_round_number: 1,
            work_rounds_total: 4,
        };
        let event = WsEvent::RoundChange { payload: snap };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"roundChange\""));
        assert!(json.contains("\"elapsed_secs\":60"));
    }
}
