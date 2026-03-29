## Purpose

Defines the collapsible accordion UI for selecting light and dark themes in the Appearance settings section, including trigger row previews, active-mode indicators, checkmark behavior, and interaction isolation.

## Requirements

### Requirement: Collapsible theme picker rows
The Appearance section SHALL present each theme picker (Light, Dark) as a collapsible row. Each row SHALL have a trigger header that is always visible and an expandable list of theme cards. At most one picker SHALL be expanded at a time (accordion behavior). Both pickers SHALL be collapsed by default when the settings window opens.

#### Scenario: Light picker expands on trigger click
- **WHEN** the user clicks the Light picker trigger
- **THEN** the light theme list becomes visible and the dark theme list collapses if open

#### Scenario: Dark picker expands on trigger click
- **WHEN** the user clicks the Dark picker trigger
- **THEN** the dark theme list becomes visible and the light theme list collapses if open

#### Scenario: Clicking open trigger collapses it
- **WHEN** the user clicks the trigger of the currently open picker
- **THEN** that picker collapses and no picker is open

#### Scenario: Both pickers collapsed on open
- **WHEN** the settings window opens
- **THEN** both the light and dark pickers are collapsed

### Requirement: Trigger row preview
Each picker's trigger header SHALL display the currently configured theme for that picker as an inline preview consisting of the theme name and a color chip. The color chip SHALL show the three round-type swatches (focus, short break, long break) on the theme's own background color.

#### Scenario: Light trigger shows configured light theme
- **WHEN** the settings window opens or `theme_light` changes
- **THEN** the light picker trigger displays the `theme_light` name and its color chip

#### Scenario: Dark trigger shows configured dark theme
- **WHEN** the settings window opens or `theme_dark` changes
- **THEN** the dark picker trigger displays the `theme_dark` name and its color chip

### Requirement: Active-mode indicator on trigger
The trigger row for the currently active picker SHALL carry a visible "active" indicator. When the active picker changes (due to a mode selector change or OS scheme change), the indicator SHALL update immediately.

#### Scenario: Active indicator on light trigger in light mode
- **WHEN** `theme_mode` is `"light"`
- **THEN** the light picker trigger shows the active indicator and the dark trigger does not

#### Scenario: Active indicator on dark trigger in dark mode
- **WHEN** `theme_mode` is `"dark"`
- **THEN** the dark picker trigger shows the active indicator and the light trigger does not

#### Scenario: Active indicator follows OS in auto mode
- **WHEN** `theme_mode` is `"auto"` and the OS switches from light to dark
- **THEN** the active indicator moves from the light trigger to the dark trigger

### Requirement: Selected theme checkmark
Within an expanded picker list, the configured theme SHALL display a checkmark regardless of whether that picker is currently active.

#### Scenario: Checkmark on configured theme in active picker
- **WHEN** the light picker is expanded and `theme_mode` is `"light"`
- **THEN** the theme matching `theme_light` shows a checkmark

#### Scenario: Checkmark on configured theme in inactive picker
- **WHEN** the dark picker is expanded and `theme_mode` is `"light"`
- **THEN** the theme matching `theme_dark` still shows a checkmark

### Requirement: Picker interaction does not reset theme selection
Expanding, collapsing, or switching between pickers SHALL NOT change `theme_light`, `theme_dark`, `theme_mode`, or the currently applied theme.

#### Scenario: Toggling pickers preserves applied theme
- **WHEN** the user opens and closes the dark picker without selecting a theme
- **THEN** the currently applied theme, `theme_light`, `theme_dark`, and `theme_mode` are all unchanged
