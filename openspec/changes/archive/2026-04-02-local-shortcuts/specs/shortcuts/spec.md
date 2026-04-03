## MODIFIED Requirements

### Requirement: Reset All Settings disables global shortcuts
When the user resets all settings to defaults, `global_shortcuts_enabled` SHALL be `false`, any previously registered global shortcuts SHALL be unregistered, and all local shortcut bindings SHALL be reverted to their default values (Space, ArrowLeft, ArrowRight, ArrowDown, ArrowUp, m, F11).

#### Scenario: Reset All Settings disables global shortcuts and restores local defaults
- **WHEN** the user resets all settings to defaults
- **THEN** `global_shortcuts_enabled` SHALL be `false` and any previously registered shortcuts SHALL be unregistered
- **AND** all seven local shortcut bindings SHALL revert to their defaults
