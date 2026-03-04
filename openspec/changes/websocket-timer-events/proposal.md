## Why

The WebSocket server currently only broadcasts `roundChange`, leaving third-party clients (browser extensions, integrations) with no way to react to pause and resume events without polling. Forwarding `paused`, `resumed`, and `reset` events directly over the WebSocket makes the protocol useful for real-time integrations and eliminates the need for workarounds.

## What Changes

- Add `Paused`, `Resumed`, and `Reset` variants to the `WsEvent` broadcast enum
- Add `broadcast_paused`, `broadcast_resumed`, and `broadcast_reset` functions to the WebSocket public API
- Call the new broadcast functions from the timer event handler alongside the existing Tauri emits for each of the three events
- Update the WebSocket module doc comment to reflect the expanded protocol

## Capabilities

### New Capabilities

- `websocket-protocol`: The WebSocket broadcast protocol — message types, payloads, and the client request/response contract

### Modified Capabilities

_(none — no existing spec covers the WebSocket protocol)_

## Impact

- `src-tauri/src/websocket/mod.rs` — `WsEvent` enum, new broadcast functions, doc comment
- `src-tauri/src/timer/mod.rs` — three call sites added (Paused, Resumed, Reset arms of the event loop)
- No frontend changes; no new Tauri commands or capabilities required
- No dependency changes
