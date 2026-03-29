## ADDED Requirements

### Requirement: Theme mode selection
The system SHALL provide three theme modes — Auto, Light, and Dark — that control how the active theme is resolved. The active mode SHALL be persisted in settings as `theme_mode` with values `"auto"`, `"light"`, or `"dark"`.

#### Scenario: Default mode is Auto
- **WHEN** a new user launches the app for the first time
- **THEN** `theme_mode` is `"auto"`

#### Scenario: User selects Light mode
- **WHEN** the user selects Light mode
- **THEN** `theme_mode` is saved as `"light"` and the light theme picker's selection becomes the active theme immediately

#### Scenario: User selects Dark mode
- **WHEN** the user selects Dark mode
- **THEN** `theme_mode` is saved as `"dark"` and the dark theme picker's selection becomes the active theme immediately

#### Scenario: User selects Auto mode
- **WHEN** the user selects Auto mode
- **THEN** `theme_mode` is saved as `"auto"` and the active theme is resolved from the OS color scheme immediately

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

### Requirement: Active theme resolution
The system SHALL resolve the active theme using: Auto → OS dark? `theme_dark` : `theme_light`; Light → `theme_light`; Dark → `theme_dark`. The `theme` settings field SHALL be removed; the active theme SHALL always be derived at runtime.

#### Scenario: Auto mode resolves to dark theme on dark OS
- **WHEN** `theme_mode` is `"auto"` and `prefers-color-scheme` is `dark`
- **THEN** the active theme is `theme_dark`

#### Scenario: Auto mode resolves to light theme on light OS
- **WHEN** `theme_mode` is `"auto"` and `prefers-color-scheme` is `light`
- **THEN** the active theme is `theme_light`

#### Scenario: Light mode ignores OS
- **WHEN** `theme_mode` is `"light"` regardless of OS color scheme
- **THEN** the active theme is always `theme_light`

#### Scenario: Dark mode ignores OS
- **WHEN** `theme_mode` is `"dark"` regardless of OS color scheme
- **THEN** the active theme is always `theme_dark`

### Requirement: Live OS color scheme response
When `theme_mode` is `"auto"`, the application SHALL respond to OS color scheme changes without requiring a restart. Both the main window and the settings window SHALL update simultaneously.

#### Scenario: OS switches to dark while app is open in Auto mode
- **WHEN** `theme_mode` is `"auto"` and the OS switches to dark mode
- **THEN** the active theme changes to `theme_dark` immediately in all open windows

#### Scenario: OS switches to light while app is open in Auto mode
- **WHEN** `theme_mode` is `"auto"` and the OS switches to light mode
- **THEN** the active theme changes to `theme_light` immediately in all open windows

#### Scenario: OS change ignored when not in Auto mode
- **WHEN** `theme_mode` is `"light"` or `"dark"` and the OS color scheme changes
- **THEN** the active theme does not change

### Requirement: Migration for existing users
On first launch after the update, the system SHALL migrate existing single-theme settings to the new three-field model with no visible change to the user.

#### Scenario: Existing user with custom theme
- **WHEN** `theme_light` is absent from the settings DB and `theme` exists with value e.g. `"Nord"`
- **THEN** `theme_light` and `theme_dark` are both set to `"Nord"` and `theme_mode` is set to `"auto"`

#### Scenario: Brand new install
- **WHEN** no settings exist in the DB
- **THEN** `theme_mode = "auto"`, `theme_light = "Pomotroid"`, `theme_dark = "Pomotroid"` are seeded as defaults
