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

---

### Requirement: Global shortcuts can be enabled or disabled as a unit

The system SHALL provide a `global_shortcuts_enabled` boolean setting (DB key: `global_shortcuts_enabled`, default `false`). When `false`, no global shortcuts SHALL be registered with the OS. When `true`, all four shortcuts SHALL be registered using the current key bindings. The change SHALL take effect immediately without requiring a restart. When settings are reset to defaults, `global_shortcuts_enabled` SHALL revert to `false` and all local shortcut bindings SHALL also revert to their defaults.

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

#### Scenario: Reset All Settings disables global shortcuts and restores local defaults

- **WHEN** the user resets all settings to defaults
- **THEN** `global_shortcuts_enabled` SHALL be `false` and any previously registered shortcuts SHALL be unregistered
- **AND** all seven local shortcut bindings SHALL revert to their defaults

#### Scenario: Disabled state persists across restarts

- **WHEN** global shortcuts are disabled and the application is restarted
- **THEN** no global shortcuts SHALL be registered on startup
