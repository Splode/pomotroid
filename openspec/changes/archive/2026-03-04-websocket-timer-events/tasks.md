## 1. Extend WsEvent enum

- [x] 1.1 Add `Paused { payload: PausedPayload }` variant to `WsEvent` in `websocket/mod.rs`
- [x] 1.2 Add `Resumed { payload: ResumedPayload }` variant to `WsEvent`
- [x] 1.3 Add `Reset` variant to `WsEvent` (no payload)
- [x] 1.4 Define `PausedPayload` and `ResumedPayload` structs (or inline with `elapsed_secs: u64`)

## 2. Add broadcast functions

- [x] 2.1 Add `broadcast_paused(state: &Arc<WsState>, elapsed_secs: u64)` function
- [x] 2.2 Add `broadcast_resumed(state: &Arc<WsState>, elapsed_secs: u64)` function
- [x] 2.3 Add `broadcast_reset(state: &Arc<WsState>)` function

## 3. Wire into timer event loop

- [x] 3.1 Call `broadcast_paused` from the `TimerEvent::Paused` arm in `timer/mod.rs`
- [x] 3.2 Call `broadcast_resumed` from the `TimerEvent::Resumed` arm in `timer/mod.rs`
- [x] 3.3 Call `broadcast_reset` from the `TimerEvent::Reset` arm in `timer/mod.rs`

## 4. Update module documentation

- [x] 4.1 Update the protocol comment at the top of `websocket/mod.rs` to list the three new server→client message types

## 5. Tests and verification

- [x] 5.1 Add unit test asserting `WsEvent::Paused` serialises to `{ "type": "paused", "payload": { "elapsed_secs": … } }`
- [x] 5.2 Add unit test asserting `WsEvent::Resumed` serialises to `{ "type": "resumed", "payload": { "elapsed_secs": … } }`
- [x] 5.3 Add unit test asserting `WsEvent::Reset` serialises to `{ "type": "reset" }`
- [x] 5.4 Run `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — zero warnings
- [x] 5.5 Verify with a WebSocket client (e.g. `websocat`) that pause, resume, and reset each produce the expected JSON

## 6. Add started event

- [x] 6.1 Add `Started { total_secs: u32 }` variant to `TimerEvent` in `engine.rs`
- [x] 6.2 Emit `TimerEvent::Started` in the `Phase::Idle` + `TimerCommand::Start` arm of the engine loop
- [x] 6.3 Add `WsEvent::Started` variant and `StartedPayload { total_secs: u32 }` struct to `websocket/mod.rs`
- [x] 6.4 Add `broadcast_started(state, total_secs)` function to `websocket/mod.rs`
- [x] 6.5 Handle `TimerEvent::Started` in `timer/mod.rs` — emit `timer:started` Tauri event and broadcast WS event
- [x] 6.6 Update protocol doc comment in `websocket/mod.rs`
- [x] 6.7 Add serialisation unit test for `WsEvent::Started`
- [x] 6.8 Run `cargo clippy -- -D warnings` — zero warnings
