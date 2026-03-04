## Context

The WebSocket server (`src-tauri/src/websocket/mod.rs`) broadcasts a `WsEvent` enum over a `tokio::sync::broadcast` channel. Currently the enum has a single variant — `RoundChange { payload: TimerSnapshot }` — serialised with `#[serde(tag = "type", rename_all = "camelCase")]`.

The timer event loop in `src-tauri/src/timer/mod.rs` already handles `TimerEvent::Paused`, `TimerEvent::Resumed`, and `TimerEvent::Reset`, emitting Tauri window events for each. The WebSocket broadcast step is simply absent from those arms.

## Goals / Non-Goals

**Goals:**
- Add `paused`, `resumed`, and `reset` message types to the WebSocket broadcast protocol
- Keep the existing `roundChange` behaviour unchanged
- Expose a clean public API (`broadcast_paused`, `broadcast_resumed`, `broadcast_reset`) mirroring the existing `broadcast_round_change` function

**Non-Goals:**
- Changing the `getState` / `state` request-response flow
- Adding new client-to-server message types
- Changing the Tauri window event names or payloads

## Decisions

### Extend `WsEvent` rather than introduce a second channel

Adding new variants to the existing enum keeps all WebSocket messages on a single broadcast channel. All connected clients receive every event type without any per-client subscription logic. Given the low event rate of a Pomodoro timer, there is no fan-out pressure that would justify separate channels.

**Alternative considered:** a separate `tokio::sync::broadcast` channel per event category. Rejected — adds complexity with no benefit at this event volume.

### Payload shapes mirror the existing Tauri event payloads

- `paused` / `resumed` carry `{ "elapsed_secs": u64 }` — identical to what `app.emit("timer:paused", …)` sends, so integrations already familiar with the Tauri protocol can adopt the WebSocket version with no translation.
- `reset` carries no payload — identical to the reset broadcast contract proposed by the user; the client should call `getState` if it needs a full snapshot.

**Alternative considered:** including a full `TimerSnapshot` in `reset`. Rejected — the snapshot is already available via `getState` and adding it couples reset to snapshot construction unnecessarily.

### One broadcast function per event type

`broadcast_paused(state, elapsed_secs)`, `broadcast_resumed(state, elapsed_secs)`, `broadcast_reset(state)` — each a thin wrapper that constructs the variant and calls `broadcast_tx.send(…)`. This matches the existing `broadcast_round_change` pattern and keeps call sites in `timer/mod.rs` readable.

## Risks / Trade-offs

- **No subscribers:** `broadcast_tx.send` returns `Err` when there are no receivers; all three functions ignore this error (same behaviour as `broadcast_round_change`). No risk.
- **Protocol versioning:** Adding new message types is additive and backwards-compatible — existing clients that don't handle the new types will simply ignore unrecognised `type` values.
- **`reset` without payload:** A client that wants post-reset state must issue a `getState` request. This is a minor extra round-trip but keeps the event lean and consistent with the no-payload design of `timer:reset` in the Tauri layer.
