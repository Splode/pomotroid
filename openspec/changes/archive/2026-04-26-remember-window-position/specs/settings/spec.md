## ADDED Requirements

### Requirement: window_x and window_y settings
The system SHALL store optional integer settings `window_x` and `window_y` (physical pixel coordinates) in the settings database. These keys are absent on a fresh install and are only written after the first time the user moves the window. Absent keys SHALL be treated as "no saved position."

#### Scenario: Keys absent on first run
- **WHEN** the application runs for the first time with no existing settings
- **THEN** `window_x` and `window_y` SHALL NOT be present in the settings database

#### Scenario: Keys written after window move
- **WHEN** the user moves the main window
- **THEN** `window_x` and `window_y` SHALL be present in the settings database with the new coordinates

### Requirement: window_width and window_height settings
The system SHALL store optional unsigned integer settings `window_width` and `window_height` (physical pixels) in the settings database. These keys are absent on a fresh install and are written alongside `window_x`/`window_y` after the first move or resize.

#### Scenario: Keys absent on first run
- **WHEN** the application runs for the first time with no existing settings
- **THEN** `window_width` and `window_height` SHALL NOT be present in the settings database

#### Scenario: Keys written after window resize
- **WHEN** the user resizes the main window
- **THEN** `window_width` and `window_height` SHALL be present in the settings database with the new dimensions

### Requirement: Window position and size cleared by Reset All Settings
When the user triggers Reset All Settings, the four window geometry keys (`window_x`, `window_y`, `window_width`, `window_height`) SHALL be removed from the database along with all other settings. The reset takes effect on the next launch — the current session's window position is not disturbed. On the following launch the window opens at the OS default position and size.

#### Scenario: Reset All Settings removes geometry keys
- **WHEN** the user triggers Reset All Settings
- **THEN** `window_x`, `window_y`, `window_width`, and `window_height` SHALL NOT be present in the settings database

#### Scenario: Window opens at OS default after reset
- **WHEN** the application starts after a Reset All Settings has been performed
- **THEN** the main window SHALL open at the OS default position and size
