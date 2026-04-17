## MODIFIED Requirements

### Requirement: Reset to factory defaults via Settings → About

The system SHALL provide a "Reset All Settings" action in Settings → About. The action SHALL use an inline two-step confirmation: the first interaction reveals a confirmation prompt with Cancel and Reset controls; only the second (Reset) interaction fires `settings_reset_defaults`. Cancelling at any point SHALL restore the original button without performing a reset. The action resets all settings globally (all 26 keys) to factory defaults.

#### Scenario: Reset button is visible in About section

- **WHEN** the user navigates to Settings → About
- **THEN** a "Reset All Settings" row SHALL be visible below the navigation links

#### Scenario: First click enters confirmation state

- **WHEN** the user clicks "Reset All Settings"
- **THEN** the row SHALL replace the button with a confirmation prompt and Cancel / Reset buttons

#### Scenario: Cancel dismisses confirmation without resetting

- **WHEN** the user clicks Cancel in the confirmation state
- **THEN** the row SHALL return to the initial "Reset All Settings" button and no settings SHALL be changed

#### Scenario: Confirm fires global reset

- **WHEN** the user clicks Reset in the confirmation state
- **THEN** `settings_reset_defaults` SHALL be invoked, all settings SHALL revert to factory defaults, and the row SHALL return to the initial button

#### Scenario: Reset no longer available in Timer section

- **WHEN** the user navigates to Settings → Timer
- **THEN** no reset button SHALL be present in that section
