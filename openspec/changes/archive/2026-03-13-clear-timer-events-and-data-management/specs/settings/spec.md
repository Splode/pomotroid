## MODIFIED Requirements

### Requirement: Reset All Settings action is located in Settings → System
The system SHALL provide a "Reset All Settings" action in Settings → System. The action SHALL use an inline two-step confirmation: the first interaction reveals a confirmation prompt with Cancel and Reset controls; only the second (Reset) interaction fires `settings_reset_defaults`. Cancelling at any point SHALL restore the original button without performing a reset. The action resets all settings globally (all 26 keys) to factory defaults, including clearing any custom alert sounds from disk and from the audio engine's in-memory state.

#### Scenario: Cancel dismisses confirmation without resetting
- **WHEN** the user clicks Reset All Settings and then clicks Cancel
- **THEN** no settings SHALL be changed and the row SHALL return to its initial button state

#### Scenario: Confirm fires global reset
- **WHEN** the user clicks Reset All Settings and then clicks Confirm
- **THEN** `settings_reset_defaults` SHALL be invoked, all settings SHALL revert to factory defaults, and the row SHALL return to the initial button state

#### Scenario: Custom alert sounds are cleared on reset
- **WHEN** the user confirms a full settings reset
- **THEN** any custom alert sound files SHALL be deleted from disk and the audio engine's in-memory custom paths SHALL be cleared

#### Scenario: About section has no reset button
- **WHEN** the user navigates to Settings → About
- **THEN** no reset button SHALL be present in that section
