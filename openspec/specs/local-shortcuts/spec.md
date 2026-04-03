### Requirement: Local shortcuts are active while any app window has focus
The system SHALL handle a configurable set of keyboard shortcuts that fire when the user presses a bound key while any Pomotroid window (main or settings) has OS focus. Local shortcuts SHALL NOT fire when the focus is inside a text input element.

#### Scenario: Shortcut fires in main window
- **WHEN** the main timer window has OS focus
- **AND** the user presses the key bound to pause/resume
- **THEN** the timer SHALL toggle between running and paused

#### Scenario: Shortcut fires in settings window
- **WHEN** the settings window has OS focus
- **AND** the user presses the key bound to volume up
- **THEN** the volume SHALL increase by 5%

#### Scenario: Shortcut does not fire when input is focused
- **WHEN** a text input or shortcut capture field has keyboard focus
- **AND** the user presses a key that is bound to a local shortcut
- **THEN** the shortcut action SHALL NOT execute and the keypress SHALL be handled normally by the input

---

### Requirement: Default local shortcut bindings
The system SHALL provide the following default local shortcut bindings on all platforms:
- Pause/Resume: Space
- Reset current round: ArrowLeft
- Skip round: ArrowRight
- Volume down: ArrowDown
- Volume up: ArrowUp
- Mute toggle: m
- Fullscreen toggle: F11

#### Scenario: First launch defaults
- **WHEN** the application is launched for the first time with no existing settings
- **THEN** all seven local shortcut bindings SHALL match the defaults listed above

#### Scenario: Existing bindings preserved across launches
- **WHEN** the user has customized one or more local shortcut bindings and restarts the app
- **THEN** the customized bindings SHALL be restored from the database

---

### Requirement: Local shortcut actions
Each local shortcut SHALL invoke a specific action:

- **Pause/Resume**: toggles the timer between running and paused (same as `timer_toggle` IPC command)
- **Reset current round**: resets the current timer round to its full duration without advancing sequence (same as `timer_reset` IPC command)
- **Skip round**: ends the current round and advances to the next in sequence (same as `timer_skip` IPC command)
- **Volume down**: decreases volume by 5 percentage points, clamped to 0.0
- **Volume up**: increases volume by 5 percentage points, clamped to 1.0
- **Mute toggle**: toggles the volume between 0.0 and the last non-zero volume level
- **Fullscreen toggle**: toggles the main window between fullscreen and its previous size/position

#### Scenario: Volume up at maximum
- **WHEN** the volume is at 1.0 (100%)
- **AND** the user presses the volume up shortcut
- **THEN** the volume SHALL remain at 1.0 (no overflow)

#### Scenario: Volume down at minimum
- **WHEN** the volume is at 0.0 (0%)
- **AND** the user presses the volume down shortcut
- **THEN** the volume SHALL remain at 0.0 (no underflow)

#### Scenario: Mute restores previous volume
- **WHEN** the volume is at 0.6 (60%)
- **AND** the user presses the mute shortcut
- **THEN** the volume SHALL be set to 0.0
- **WHEN** the user presses the mute shortcut again
- **THEN** the volume SHALL be restored to 0.6

#### Scenario: Fullscreen toggle
- **WHEN** the main window is in windowed mode
- **AND** the user presses the fullscreen shortcut
- **THEN** the main window SHALL enter fullscreen mode
- **WHEN** the user presses the fullscreen shortcut again
- **THEN** the main window SHALL exit fullscreen and return to windowed mode

---

### Requirement: Local shortcuts are re-mappable in Settings
The system SHALL allow users to change any local shortcut binding via Settings → Shortcuts. Each binding field SHALL record the next keypress (excluding modifier-only keys) as the new binding. The new binding SHALL be saved immediately and take effect without restart.

#### Scenario: User rebinds pause/resume
- **WHEN** the user clicks the pause/resume local shortcut field in Settings → Shortcuts
- **AND** presses the P key
- **THEN** the binding SHALL be updated to "p" in the database
- **AND** pressing P while the main window is focused SHALL toggle the timer

#### Scenario: Binding takes effect immediately
- **WHEN** the user saves a new local shortcut binding
- **AND** the settings window remains open
- **THEN** pressing the newly bound key SHALL immediately trigger the corresponding action (no restart required)

---

### Requirement: Reset All Settings restores local shortcut defaults
When the user resets all settings to defaults, all local shortcut bindings SHALL be reverted to their default values.

#### Scenario: Reset restores default bindings
- **WHEN** the user triggers "Reset All Settings" via the Settings menu
- **THEN** all seven local shortcut bindings SHALL revert to their defaults (Space, ArrowLeft, ArrowRight, ArrowDown, ArrowUp, m, F11)
- **AND** any custom bindings the user had configured SHALL be discarded
