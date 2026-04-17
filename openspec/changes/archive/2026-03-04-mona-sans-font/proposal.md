## Why

The app currently uses `system-ui` which renders differently on every platform (Cantarell on Linux, San Francisco on macOS, Segoe UI on Windows), producing an inconsistent experience. Embedding Mona Sans — a variable font with weight, width, optical-size, and italic axes — gives a consistent, polished appearance across all platforms and unlocks expressive typographic features that reinforce the app's interaction model.

## What Changes

- **`static/fonts/`** (new directory) — two variable font files added:
  - `MonaSansVF[wdth,wght,opsz,ital].woff2` — main sans-serif, all four axes
  - `MonaSansMonoVF[wght].woff2` — monospaced companion, weight axis
- **`src/app.css`** — `@font-face` declarations for both fonts; `font-family: 'Mona Sans'` on `body`; `font-optical-sizing: auto` globally so the optical-size axis adapts to each element's font-size automatically
- **`src/lib/components/TimerDisplay.svelte`** — font-weight raised 300 → 350; `font-stretch` transitions between 95% (timer running) and 103% (paused/idle) via a CSS transition; accepts new `isCompact` prop and applies `font-stretch: 85%` in compact mode
- **`src/lib/components/Timer.svelte`** — threads `isCompact` prop into `TimerDisplay`
- **`src/lib/components/ShortcutInput.svelte`** — `font-family: monospace` → `'Mona Sans Mono', monospace`

## Capabilities

### New Capabilities

- `variable-font-typography`: Embedded variable font with axis-driven typographic variation — optical sizing globally, width-axis animation on timer state, compact-mode condensing, and monospaced companion for shortcut display

### Modified Capabilities

<!-- none — no existing spec-level requirements change -->

## Impact

- **Frontend only** — no Rust, IPC, settings, or DB changes
- `TimerDisplay.svelte` gains an `isCompact` prop (non-breaking default: `false`)
- Font files add ~150–200KB to the app bundle (negligible for a desktop Tauri app)
- All 37 bundled themes unaffected — font tokens are separate from color tokens
- `font-optical-sizing: auto` is a global addition; no component-level overrides needed
