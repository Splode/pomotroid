## ADDED Requirements

### Requirement: Global shortcuts can be enabled or disabled as a unit
The system SHALL provide a `global_shortcuts_enabled` boolean setting (DB key: `global_shortcuts_enabled`, default `false`). When `false`, no global shortcuts SHALL be registered with the OS. When `true`, all four shortcuts SHALL be registered using the current key bindings. The change SHALL take effect immediately without requiring a restart.

#### Scenario: Global shortcuts disabled by default on first launch
- **WHEN** the application is launched for the first time with no existing settings
- **THEN** `global_shortcuts_enabled` SHALL be `false` and no global shortcuts SHALL be registered

#### Scenario: Enabling global shortcuts registers them immediately
- **WHEN** the user toggles global shortcuts on in Settings → Shortcuts
- **THEN** all four shortcuts SHALL be registered with the OS using the current key bindings before the settings window is dismissed

#### Scenario: Disabling global shortcuts unregisters them immediately
- **WHEN** the user toggles global shortcuts off in Settings → Shortcuts
- **THEN** all four shortcuts SHALL be unregistered from the OS immediately, and pressing the previously bound keys SHALL have no effect on the timer

#### Scenario: Shortcut key fields are not editable while disabled
- **WHEN** global shortcuts are disabled
- **THEN** the individual shortcut key fields SHALL be non-interactive (pointer events blocked) and visually dimmed, preventing edits until global shortcuts are re-enabled

#### Scenario: Re-enabling shortcuts uses stored key bindings
- **WHEN** the user re-enables global shortcuts after previously disabling them
- **THEN** the shortcuts registered SHALL reflect the key bindings currently stored in the database

#### Scenario: Reset All Settings disables global shortcuts
- **WHEN** the user resets all settings to defaults
- **THEN** `global_shortcuts_enabled` SHALL be `false` and any previously registered shortcuts SHALL be unregistered

#### Scenario: Disabled state persists across restarts
- **WHEN** global shortcuts are disabled and the application is restarted
- **THEN** no global shortcuts SHALL be registered on startup
