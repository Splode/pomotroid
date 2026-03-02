## Context

The timer UI scales via CSS `zoom` applied to a `.timer` wrapper div in `Timer.svelte`. In compact mode the dial fills all available space calculated as `Math.min(w - 16, h - TITLEBAR_H - 16 - COMPACT_BOTTOM_PAD) / 220`. Adding controls *inside* the zoom wrapper would make them tiny at small window sizes (uiScale can be as low as 0.4). They must therefore live **outside** the zoom wrapper at a fixed rendered size.

`Timer.svelte` currently renders one root element: the zoomed `.timer` div. The restructure adds a `.timer-outer` wrapper (flex column, centered) with the zoom div on top and the mini controls below — only in compact mode.

## Goals / Non-Goals

**Goals:**
- Restart-round, play/pause, and skip buttons visible in compact mode
- Controls render at a fixed size regardless of uiScale
- Visual weight is low — does not compete with the dial
- No new IPC commands, no new backend changes

**Non-Goals:**
- Round label or footer in compact mode
- Configurable control visibility
- Touch/swipe interactions

## Decisions

### D1: Controls outside the zoom wrapper

**Decision**: Wrap the existing zoomed `.timer` div and the new mini controls in a `.timer-outer` flex-column div. The zoom only applies to the inner div; the controls inherit no scale.

**Alternative considered**: Absolute-positioned overlay on `<main>` in `+page.svelte`. Rejected — spreads timer control logic into the page, breaks component encapsulation.

### D2: New `MiniControls.svelte` component

**Decision**: Extract the three compact buttons into a dedicated component. It imports `timerToggle`, `timerRestartRound`, `timerSkip` from `$lib/ipc` and `$timerState` from `$lib/stores/timer` directly (for the play/pause icon state).

**Rationale**: Keeps `Timer.svelte` lean; the component can be independently styled and tested.

### D3: Icon sizes and button dimensions

**Decision**: Buttons are 24×24px with 10px icons (restart/skip) and 12px play/pause icon. The strip's total height is ~28px. Gap between buttons: 14px. No text labels. Colors: `--color-foreground-darker` at rest, `--color-foreground` on hover for side buttons; play/pause gets a thin circular border matching the main play/pause button's style but at 24px diameter.

**Rationale**: Compact enough to fit in the reserved bottom space without obscuring the dial; recognisable as the same actions as the full controls.

### D4: COMPACT_BOTTOM_PAD increase

**Decision**: Increase `COMPACT_BOTTOM_PAD` from `20` to `48` in `+page.svelte`. This reserves 28px for the control strip + 8px gap + 12px breathing room below, ensuring the uiScale calculation doesn't allocate that space to the dial.

`main.compact` padding-bottom decreases from `20px` to `8px` — the controls themselves provide visual anchoring at the bottom.

## Risks / Trade-offs

- **Slightly smaller dial in compact mode**: At 300px height, uiScale drops from ~1.0 to ~0.89. Acceptable — the dial is still large and the controls add functional value.
- **Very small windows** (< 200px): At extreme sizes the controls may feel crowded. The 0.4 minimum uiScale clamp is unchanged; controls stay at their fixed 24px height regardless.

## Migration Plan

Pure frontend change. No data migration. No backend changes. Ships in a single PR.
