## Context

The app uses `font-family: system-ui` with no custom font assets. `TimerDisplay` renders the countdown at `font-weight: 300` with a flat `font-stretch`. `ShortcutInput` uses `font-family: monospace`. This change introduces two embedded variable font files and wires up their axes at three levels: globally (optical sizing), component state (timer width animation), and layout mode (compact condensing).

## Goals / Non-Goals

**Goals:**

- Consistent cross-platform typography via a single embedded font family
- Optical sizing applied automatically across the entire UI with zero per-component effort
- Width-axis animation on the timer display that responds to running/paused state
- Compact mode condensing of timer digits via the width axis
- Monospaced shortcut display using the matching Mona Sans Mono companion

**Non-Goals:**

- Per-theme font overrides
- Custom fonts for any component other than those listed in the proposal
- Subsetting or tree-shaking the font files (file size is not a concern for a desktop app)
- Animating font axes on components other than `TimerDisplay`

## Decisions

### 1. WOFF2 only, no fallback format

Ship only `.woff2`. All Chromium versions that run Tauri 2 support WOFF2. Including `.woff` or `.ttf` fallbacks would double the asset size for no practical benefit.

### 2. `font-display: block`

The app is offline-first (no network dependency). `block` prevents any flash of unstyled text on the rare cold start where the font isn't yet in the browser's internal cache. `swap` would cause a visible relayout; `auto` is unpredictable.

### 3. Width animation via CSS `font-stretch` + `transition`, not `font-variation-settings`

`font-stretch` is the high-level CSS property for the `wdth` axis and is directly animatable. `font-variation-settings: 'wdth' 95` also works but is lower-level and harder to read. Using `font-stretch` with a CSS `transition` keeps the component CSS clean and lets the browser handle interpolation.

`TimerDisplay.svelte` binds a `style` attribute to set `font-stretch` dynamically based on `state.is_running` and the `isCompact` prop:

```
isCompact → 85%
is_running → 95%
otherwise  → 103%
```

The CSS transition (`font-stretch 400ms ease`) handles the animation. No JS animation library needed.

**Alternative considered**: Svelte `tweened` store driving `font-variation-settings`. Rejected — CSS transitions are sufficient, simpler, and GPU-accelerated without extra JS.

### 4. `font-optical-sizing: auto` on `:root`, not `body`

Setting it on `:root` ensures SVG `<text>` elements inside component styles also inherit it. SVG text does not inherit from `body` in some Chromium configurations. The stats views render chart labels as SVG text at 8–9px — these benefit most from low `opsz` values.

**Alternative considered**: Per-element `font-optical-sizing`. Rejected — `auto` on `:root` is a single line that covers everything correctly.

### 5. `isCompact` threaded as a prop into `TimerDisplay`

`TimerDisplay` currently has no knowledge of compact mode. Rather than using a CSS `:global` selector from `Timer.svelte`, a prop is cleaner: it keeps `TimerDisplay` self-contained and testable in isolation.

`Timer.svelte` already receives `isCompact` and passes it to `TimerDial`; adding it to `TimerDisplay` is a one-line change in the parent template.

### 6. Compact overrides running/paused

When `isCompact` is true, `font-stretch: 85%` is applied unconditionally, regardless of timer state. The compact layout's space constraint is more important than the state-expressive width shift. The transition still animates the change in and out as the user resizes.

## Risks / Trade-offs

- **[CSS `font-stretch` on variable fonts in Chromium]** — Chromium maps `font-stretch` to the `wdth` axis for variable fonts. This is well-established in Chromium 88+. Tauri 2 requires a newer Chromium. Risk: negligible.
- **[`font-optical-sizing: auto` on SVG text]** — Inheritance into inline SVG is consistent in Chromium but not guaranteed in all browsers. Mitigation: if any SVG labels look wrong, add explicit `font-optical-sizing` overrides to the affected components.
- **[font-stretch transition visible at low progress values]** — When the user starts the timer at 0% progress, the arc animates and the font-stretch transitions simultaneously. Both run on independent timers (800ms tween vs 400ms CSS). No conflict, but worth a visual check.
- **[Weight 350 is non-standard]** — CSS `font-weight: 350` is valid for variable fonts and supported in Chromium, but some tooling may warn. No runtime impact.
