## MODIFIED Requirements

### Requirement: Platform-aware default shortcut bindings
The system SHALL seed shortcut defaults that are appropriate for the host operating system. On macOS, defaults SHALL use the `Command+Shift` modifier with digit keys (`1`–`4`) to avoid conflicts with macOS media keys and follow platform conventions. On Windows and Linux, defaults SHALL use `Control+F1`–`F4`.

#### Scenario: macOS first launch shortcut defaults
- **WHEN** the application is launched for the first time on macOS
- **AND** no shortcut preferences have been saved
- **THEN** the default shortcuts SHALL be `Command+Shift+1` (toggle), `Command+Shift+2` (reset), `Command+Shift+3` (skip), `Command+Shift+4` (restart round)

#### Scenario: Windows/Linux first launch shortcut defaults
- **WHEN** the application is launched for the first time on Windows or Linux
- **AND** no shortcut preferences have been saved
- **THEN** the default shortcuts SHALL be `Control+F1` (toggle), `Control+F2` (reset), `Control+F3` (skip), `Control+F4` (restart round)

#### Scenario: Existing preferences preserved
- **WHEN** the application launches and shortcut preferences already exist in the database
- **THEN** the existing saved shortcuts SHALL be used regardless of platform
