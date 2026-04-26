## ADDED Requirements

### Requirement: Window position is persisted on move and resize
The system SHALL save the main window's position (`window_x`, `window_y`) and size (`window_width`, `window_height`) to the settings database whenever the window is moved or resized. Values are physical pixel coordinates and dimensions as reported by the OS.

#### Scenario: Position saved after window is moved
- **WHEN** the user drags the main window to a new position
- **THEN** the updated `window_x` and `window_y` values SHALL be written to the settings database

#### Scenario: Size saved after window is resized
- **WHEN** the user resizes the main window
- **THEN** the updated `window_width` and `window_height` values SHALL be written to the settings database

### Requirement: Window position is restored on startup
The system SHALL read the saved position and size from the settings database on startup and apply them to the main window before it is shown, placing the window at the same location it occupied when last used.

#### Scenario: Position restored on next launch
- **WHEN** the application starts
- **AND** valid saved position values exist in the database
- **AND** the saved position is on an available monitor
- **THEN** the main window SHALL open at the saved position with the saved size

#### Scenario: No saved position on first launch
- **WHEN** the application starts for the first time
- **AND** no position values are present in the settings database
- **THEN** the main window SHALL open at the OS default position

### Requirement: Saved position is validated against current display layout
Before restoring a saved position, the system SHALL verify that the saved window rectangle intersects at least one currently available monitor by at least 1 pixel. If the saved position is entirely off all available monitors, the saved values SHALL be discarded and the window SHALL open at the OS default position.

#### Scenario: Saved monitor is disconnected
- **WHEN** the application starts
- **AND** a saved position exists that was on a monitor no longer connected
- **THEN** the saved position SHALL be discarded
- **AND** the window SHALL open at the OS default position

#### Scenario: Display resolution decreased, window is off-screen
- **WHEN** the application starts
- **AND** a saved position exists but the window rectangle does not intersect any available monitor
- **THEN** the saved position SHALL be discarded
- **AND** the window SHALL open at the OS default position

#### Scenario: Multi-monitor position valid
- **WHEN** the application starts on a system with multiple monitors
- **AND** the saved position is within the bounds of a connected monitor
- **THEN** the window SHALL be restored on that monitor at the saved coordinates
