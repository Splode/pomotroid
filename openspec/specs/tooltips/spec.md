## Purpose

Defines the tooltip component contract, interaction model, and the complete inventory of UI elements that carry a tooltip in both the timer and settings windows.

---

## Requirements

### Requirement: Tooltip component

The system SHALL provide a reusable `Tooltip.svelte` component. It SHALL accept the following props:

- `text: string` — the tooltip string (translated by the caller via Paraglide)
- `delay?: number` — hover delay in milliseconds before the tooltip appears; defaults to `600`
- `placement?: 'above' | 'below'` — preferred placement; defaults to `'above'`

The trigger element is passed via the default slot.

#### Scenario: Tooltip appears after delay

- **WHEN** the user hovers over a trigger element for at least `delay` milliseconds
- **THEN** the tooltip SHALL appear with the configured `text`

#### Scenario: Tooltip disappears on mouse leave

- **WHEN** the user moves the mouse away from the trigger element
- **THEN** the tooltip SHALL disappear immediately

#### Scenario: Instant tooltip with delay=0

- **WHEN** `delay={0}` is set
- **AND** the user hovers over the trigger element
- **THEN** the tooltip SHALL appear without any delay

#### Scenario: Tooltip flips placement near viewport edge

- **WHEN** the trigger element is positioned such that the tooltip would overflow the viewport in the preferred direction
- **THEN** the tooltip SHALL render in the opposite direction

---

### Requirement: Info icon component

The system SHALL provide a `TooltipInfo.svelte` component that renders a styled info icon which, when hovered, shows an instant tooltip (`delay={0}`).

It SHALL accept:
- `text: string` — the tooltip string

It is intended for use alongside toggle labels in settings to surface contextual notes without cluttering the layout.

#### Scenario: Info icon shows tooltip immediately on hover

- **WHEN** the user hovers over the info icon
- **THEN** the tooltip SHALL appear immediately (no delay)

#### Scenario: Info icon is visually distinct but unobtrusive

- **WHEN** the info icon is rendered
- **THEN** it SHALL use `--color-foreground-darker` at rest and `--color-foreground` on hover, keeping it secondary to the adjacent label

---

### Requirement: Tooltip inventory — timer window

The following controls in the timer window SHALL have tooltips with the specified i18n keys and delay:

| Element | Location | i18n key | Delay |
|---|---|---|---|
| Settings button | Titlebar | `tooltip_settings` | 600 ms |
| Statistics button | Titlebar | `tooltip_statistics` | 600 ms |
| Restart Round button | Timer controls | `tooltip_restart_round` | 600 ms |
| Skip button | Timer controls | `tooltip_skip` | 600 ms |
| Reset button | Footer | `tooltip_reset` | 600 ms |
| Mute/Unmute button | Footer | `tooltip_mute` / `tooltip_unmute` | 600 ms |
| Round indicator | Footer | `tooltip_round_counter` / `tooltip_round_counter_session` | 600 ms |

#### Scenario: Reset tooltip communicates consequence

- **WHEN** the user hovers over the Reset button for 600 ms
- **THEN** the tooltip SHALL display the text for `tooltip_reset`, which communicates that the timer returns to the first work round and clears session progress

#### Scenario: Skip tooltip describes the action

- **WHEN** the user hovers over the Skip button for 600 ms
- **THEN** the tooltip SHALL display the text for `tooltip_skip`, which communicates that the current round is skipped and the next round begins

#### Scenario: Mute tooltip reflects current state

- **WHEN** the volume is greater than zero and the user hovers the mute button
- **THEN** the tooltip SHALL display `tooltip_mute`
- **WHEN** the volume is zero and the user hovers the mute button
- **THEN** the tooltip SHALL display `tooltip_unmute`

#### Scenario: Round indicator tooltip reflects long-break mode

- **WHEN** long breaks are enabled and the user hovers the round indicator (e.g. "2 / 4")
- **THEN** the tooltip SHALL display `tooltip_round_counter`, which explains that the number shows the current work round out of the total rounds before a long break
- **WHEN** long breaks are disabled and the user hovers the round indicator (e.g. "Round 3")
- **THEN** the tooltip SHALL display `tooltip_round_counter_session`, which explains that the number is a continuous session count that resets only when the timer is reset

---

### Requirement: Tooltip inventory — settings window

The following settings controls SHALL have a `TooltipInfo` icon (instant, no delay) with the specified i18n keys:

| Setting | Section | i18n key |
|---|---|---|
| Show in System Tray (Linux only) | System | `system_tray_gnome_hint` |
| Verbose Logging | System | `tooltip_verbose_logging` |
| WebSocket Server | System | `tooltip_websocket` |

The `TooltipInfo` icon for "Show in System Tray" SHALL only be rendered on Linux (guard: `isLinux`).

#### Scenario: GNOME hint shown only on Linux

- **WHEN** the app is running on Linux
- **THEN** a `TooltipInfo` icon SHALL appear alongside the "Show in System Tray" toggle label
- **WHEN** the app is running on macOS or Windows
- **THEN** no info icon SHALL be shown for that toggle

#### Scenario: Settings info icons show tooltip immediately

- **WHEN** the user hovers over any `TooltipInfo` icon in the settings window
- **THEN** the tooltip SHALL appear without delay

---

### Requirement: Tooltip styling

Tooltips SHALL be styled as follows:

- Background: `--color-background-light` (or a computed fallback if not defined by the theme)
- Text color: `--color-foreground`
- Font size: `0.72rem`
- Border radius: `4px`
- Max width: `240px`; text wraps
- A subtle box shadow and border for visual separation
- A small arrow indicating the trigger element
- Positioned using `position: fixed` so tooltip is never clipped by overflow containers

Tooltip appearance SHALL adapt to the active theme via CSS custom properties.

#### Scenario: Tooltip uses active theme colors

- **WHEN** the user switches themes
- **THEN** any visible tooltip SHALL immediately reflect the new theme's CSS custom properties

---

### Requirement: Accessibility

Tooltip triggers SHALL use `aria-describedby` pointing to the tooltip element's `id` when the tooltip is visible, so screen readers announce the tooltip text alongside the control label.

#### Scenario: Screen reader announces tooltip text

- **WHEN** a tooltip is visible
- **THEN** the trigger wrapper SHALL have `aria-describedby` set to the tooltip element's `id`
- **WHEN** the tooltip is hidden
- **THEN** the `aria-describedby` attribute SHALL be absent
