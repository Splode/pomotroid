## Context

Pomotroid uses icon-only buttons in the timer window and terse toggle labels in settings. Many controls carry no discoverable description beyond an `aria-label`. A tooltip system provides just-in-time context without cluttering the UI. One specific driver is the GNOME AppIndicator note, currently shown as a static `.note` paragraph visible to all platforms; a tooltip on an info icon is a cleaner, Linux-only pattern.

## Goals / Non-Goals

**Goals**

- Reusable `Tooltip.svelte` component usable anywhere in both windows.
- Two interaction modes: delayed (buttons, toggles) and instant (info icons).
- CSS-driven positioning with a JS fallback to flip above↔below when near a viewport edge.
- Full i18n — all tooltip strings go through Paraglide.
- Replace the existing GNOME tray `.note` paragraph with an info icon + instant tooltip on Linux.

**Non-Goals**

- Rich content (HTML, images) inside tooltips — plain text only.
- Touch/mobile support — Pomotroid is desktop-only.
- Third-party tooltip library — keep the dependency footprint flat.

## Decisions

### 1. Svelte component wrapper, not a `use:` action

**Decision**: `Tooltip.svelte` wraps the trigger element via a slot (`<Tooltip><button>...</button></Tooltip>`).

**Rationale**: Actions cannot easily render DOM nodes outside the element they are attached to. A wrapper component can append a sibling tooltip div to a shared portal root or use CSS `position: absolute` relative to the nearest positioned ancestor — simpler to style and test. The slot pattern also makes the relationship between trigger and tooltip explicit in templates.

**Alternative considered**: `use:tooltip` action — rejected because it requires manual DOM node creation and teardown, and makes accessibility wiring (aria-describedby) harder.

### 2. Two delay modes: `delayed` (600 ms) and `instant` (0 ms)

**Decision**: `Tooltip` accepts a `delay` prop defaulting to `600`. Info icon wrappers pass `delay={0}`.

**Rationale**: A 600 ms delay prevents tooltips firing during casual mouse movement over buttons, which would feel noisy. Info icons `(i)` exist solely to be hovered for information, so instant display is expected.

### 3. CSS `position: absolute` + JS viewport flip

**Decision**: The tooltip div is positioned above the trigger by default using `bottom: 100%`. On mount and on each show, a JS check measures the trigger's `getBoundingClientRect()` against `window.innerHeight` and adds a `placement="below"` attribute if insufficient space exists above.

**Rationale**: Pure CSS cannot read viewport position. The flip logic is a few lines and avoids importing a full positioning library (Floating UI, Popper.js). Given Pomotroid's small, fixed-size window this is sufficient.

### 4. Single portal vs inline positioning

**Decision**: Tooltip divs are rendered inline (sibling to the trigger), positioned with `position: absolute` inside a `position: relative` wrapper.

**Rationale**: A portal (appending to `document.body`) would require teleporting DOM nodes and complicates scoped CSS. Since Pomotroid's window is compact and overflow is controlled, inline absolute positioning is reliable.

### 5. Info icon component (`TooltipInfo.svelte`)

**Decision**: A thin wrapper `TooltipInfo.svelte` renders a styled `(i)` icon and composes `Tooltip` with `delay={0}`. Used in settings for contextual notes.

**Rationale**: Keeps usage at the call site minimal (`<TooltipInfo text={m.some_hint()} />`) and keeps the icon style consistent.

## Risks / Trade-offs

- **Overflow clipping**: If a parent element has `overflow: hidden`, the tooltip may be clipped. → Mitigation: audit parent containers; add `overflow: visible` where needed or switch to portal for those instances.
- **Compact mode**: In compact mode the footer and controls are hidden; tooltips on those elements will never fire. → No action needed; the elements simply don't exist.
- **Translation quality**: Tooltip strings for 7 non-English locales will be machine-translated. → Acceptable for initial release; community can improve later.

## Migration Plan

The existing `.note` paragraph in `SystemSection.svelte` (GNOME AppIndicator hint) is removed and replaced with a `TooltipInfo` icon next to the tray toggle label. This is a UI-only change with no settings impact. The `system_tray_gnome_hint` i18n key already exists and will be reused.

## Open Questions

- Should the Play/Pause button have a tooltip? It is icon-only but its function is universally understood. Lean toward **no** to avoid over-tooltipping obvious controls — revisit based on feedback.
- Maximum tooltip width: 240 px is proposed; adjust if long translated strings wrap awkwardly.
