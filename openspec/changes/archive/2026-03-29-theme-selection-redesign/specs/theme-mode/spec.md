## MODIFIED Requirements

### Requirement: Independent light and dark theme pickers

The system SHALL maintain two independent theme selections: `theme_light` and `theme_dark`. Both pickers SHALL display all available themes and SHALL be accessible via the collapsible theme picker UI. Defaults SHALL be `"Pomotroid"` for both.

#### Scenario: Light theme selection when Light mode active

- **WHEN** mode is `"light"` and the user selects a theme from the light picker
- **THEN** `theme_light` is saved and the selected theme is applied immediately

#### Scenario: Dark theme selection when Dark mode active

- **WHEN** mode is `"dark"` and the user selects a theme from the dark picker
- **THEN** `theme_dark` is saved and the selected theme is applied immediately

#### Scenario: Deferred preview for inactive picker

- **WHEN** the user selects a theme from the picker that is not currently active
- **THEN** the selection is saved but the visible theme does not change

#### Scenario: Deferred light picker in Auto mode with OS dark

- **WHEN** mode is `"auto"`, the OS is dark, and the user selects a theme from the light picker
- **THEN** `theme_light` is saved but the active theme remains the dark theme

#### Scenario: Both pickers accessible regardless of active mode

- **WHEN** the user opens the settings Appearance section
- **THEN** both the light and dark pickers are reachable by expanding their rows, regardless of `theme_mode`
