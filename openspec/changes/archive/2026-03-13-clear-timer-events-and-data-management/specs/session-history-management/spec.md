## ADDED Requirements

### Requirement: User can clear all session history

The system SHALL provide a `sessions_clear` Tauri command that executes `DELETE FROM sessions` within a transaction, removing all rows from the `sessions` table. The command SHALL return `Ok(())` on success and propagate a string error on failure. No event SHALL be emitted after clearing — the operation has no effect on running timer state.

The command SHALL emit structured log messages at critical junctures using the `[sessions]` module prefix, consistent with the existing logging conventions in `commands.rs`:

- `log::info!` before the delete, indicating the operation is starting
- `log::info!` after a successful delete, including the number of rows removed
- `log::error!` if the delete fails, including the error detail

The system SHALL expose a `clearSessionHistory()` wrapper in `src/lib/ipc/index.ts` that invokes the `sessions_clear` command.

#### Scenario: Clear session history removes all rows

- **WHEN** `sessions_clear` is invoked and the `sessions` table contains one or more rows
- **THEN** all rows SHALL be deleted, the table SHALL be empty, and a `log::info!` message SHALL record the number of rows removed

#### Scenario: Clear on already-empty table succeeds

- **WHEN** `sessions_clear` is invoked and the `sessions` table is already empty
- **THEN** the command SHALL return `Ok(())` without error and log that 0 rows were removed

#### Scenario: Failed clear is logged as an error

- **WHEN** `sessions_clear` is invoked and the database operation fails
- **THEN** a `log::error!` message SHALL be emitted with the `[sessions]` prefix and error detail before the error is propagated to the caller

### Requirement: Settings → System exposes data management actions under a named subsection

The Settings → System section SHALL contain a **Data** subsection, introduced by a `group-heading` element (consistent with the existing Integrations, Language, Logging, and System Tray subsections). The Data subsection SHALL contain two destructive actions: **Clear Session History** and **Reset All Settings**. Each action SHALL use the same two-step inline confirmation pattern: an initial button reveals Cancel and Confirm controls inline; only Confirm fires the destructive operation; Cancel restores the button without side effects.

The Clear Session History confirmation copy SHALL make clear that the action is permanent and cannot be undone.

#### Scenario: Data subsection heading is visible in System settings

- **WHEN** the user navigates to Settings → System
- **THEN** a "Data" group heading SHALL be visible, styled consistently with the other group headings (Integrations, Language, Logging, System Tray)

#### Scenario: Clear Session History button shows confirmation

- **WHEN** the user clicks the Clear Session History button
- **THEN** the button SHALL be replaced inline with a confirmation prompt containing Cancel and Confirm controls

#### Scenario: Confirm clears history and dismisses confirmation

- **WHEN** the user clicks Confirm in the Clear Session History confirmation
- **THEN** `clearSessionHistory()` SHALL be invoked, all session rows SHALL be deleted, and the row SHALL return to its initial button state

#### Scenario: Cancel dismisses without clearing

- **WHEN** the user clicks Cancel in the Clear Session History confirmation
- **THEN** no data SHALL be deleted and the row SHALL return to its initial button state

### Requirement: Settings → About no longer contains data management actions

The Settings → About section SHALL contain only static app metadata: version number, relevant external links, and the log directory opener. It SHALL NOT contain Reset All Settings or any other destructive action.

#### Scenario: About section has no reset button

- **WHEN** the user navigates to Settings → About
- **THEN** no reset or clear button SHALL be present
