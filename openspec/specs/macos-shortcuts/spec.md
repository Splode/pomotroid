### Requirement: macOS Accessibility permission detection
On macOS, the system SHALL expose an IPC command `accessibility_trusted` that returns a boolean indicating whether the application has been granted Accessibility access by the operating system. On all other platforms the command SHALL return `true` unconditionally.

#### Scenario: Accessibility not granted on macOS
- **WHEN** the app has not been granted Accessibility access in macOS System Settings
- **THEN** `accessibility_trusted` SHALL return `false`

#### Scenario: Accessibility granted on macOS
- **WHEN** the app has been granted Accessibility access in macOS System Settings
- **THEN** `accessibility_trusted` SHALL return `true`

#### Scenario: Non-macOS platform
- **WHEN** `accessibility_trusted` is called on Windows or Linux
- **THEN** it SHALL return `true`

### Requirement: Accessibility permission notice in Shortcuts settings
On macOS, when Accessibility access is not granted, the system SHALL display a notice at the top of the Shortcuts settings section informing the user that global shortcuts require Accessibility access, and providing a button that opens the relevant macOS System Settings pane directly.

#### Scenario: Notice shown when not trusted
- **WHEN** the user opens the Shortcuts section on macOS
- **AND** Accessibility access has not been granted
- **THEN** a notice SHALL be displayed above the shortcut inputs explaining the requirement

#### Scenario: Notice includes action link
- **WHEN** the notice is visible
- **THEN** it SHALL contain a control that opens System Settings → Privacy & Security → Accessibility when activated

#### Scenario: Notice absent when trusted
- **WHEN** Accessibility access has been granted
- **THEN** no notice SHALL be shown and the Shortcuts section SHALL display normally

#### Scenario: Notice absent on non-macOS
- **WHEN** the app is running on Windows or Linux
- **THEN** no Accessibility notice SHALL appear in the Shortcuts section

### Requirement: Accessibility status re-checked on window focus
When Accessibility access has not yet been granted, the system SHALL re-check trust status each time the settings window regains focus, and SHALL automatically dismiss the notice once access is confirmed.

#### Scenario: Notice dismisses after access granted
- **WHEN** the user grants Accessibility access in System Settings
- **AND** returns focus to the Pomotroid settings window
- **THEN** the notice SHALL disappear without requiring a restart or manual refresh
