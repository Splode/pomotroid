## Why

Compact mode (window < 300px in either dimension) shows only the timer dial with no controls. Users who keep the app in a corner as a tiny clock-like widget have no way to pause, restart, or skip without resizing the window. A slim control strip below the dial restores that access without disrupting the minimal aesthetic.

## What Changes

- A new `MiniControls.svelte` component renders three small icon-only buttons: restart current round, play/pause, skip round
- `Timer.svelte` is restructured to place the mini controls **outside** the zoom wrapper so they render at a fixed size independent of uiScale
- `COMPACT_BOTTOM_PAD` in `+page.svelte` increases from 20 → 48 to reserve space for the control strip in the uiScale calculation
- The existing `main.compact` bottom padding adjusts to account for the control strip taking that visual role

## Capabilities

### New Capabilities

- `mini-controls`: Compact timer controls (restart, play/pause, skip) displayed below the dial in compact/mini window mode.

### Modified Capabilities

*(none — no existing spec-level requirements change)*

## Impact

- `src/routes/+page.svelte`: `COMPACT_BOTTOM_PAD` constant change
- `src/lib/components/Timer.svelte`: restructure to wrap zoom div, add `MiniControls` below it when `isCompact`
- `src/lib/components/MiniControls.svelte`: new component
- No backend, IPC, settings, or localization changes
