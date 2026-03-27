## Why

Many of Pomotroid's controls are icon-only or use terse labels, leaving their exact behaviour or platform-specific caveats undiscoverable without reading documentation. Tooltips provide just-in-time context directly in the UI — reducing friction for new users and surfacing important notes (such as the GNOME AppIndicator requirement) at the exact moment they are relevant.

## What Changes

- A reusable `Tooltip` Svelte component is introduced and used across both windows.
- Timer window controls (Reset, Skip, Restart Round) receive hover tooltips.
- Settings toggles and inputs that benefit from extra context receive a small **(i)** icon that triggers an immediate tooltip on hover (no delay).
- Standard interactive controls (buttons, toggles) use a short hover delay (~600 ms) before the tooltip appears to avoid noise during normal use.
- All tooltip strings are added to every locale message file and are fully translated.
- A canonical list of tooltip targets is maintained in the spec.

## Capabilities

### New Capabilities

- `tooltips`: Reusable tooltip component, interaction model (delay, positioning, dismiss), and the full inventory of UI elements that carry a tooltip and their copy.

### Modified Capabilities

- `localization`: New message keys added for all tooltip strings across all supported locales.

## Impact

- **Frontend**: New `Tooltip.svelte` component; changes to timer page, settings section components.
- **i18n**: New keys in all 8 locale message files (`src/messages/*.json`); Paraglide rebuild required.
- **No backend changes**: Tooltips are purely a frontend concern.
- **No new dependencies**: Tooltip behaviour implemented with CSS/Svelte, no third-party library.
