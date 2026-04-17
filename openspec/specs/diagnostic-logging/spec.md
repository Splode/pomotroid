## ADDED Requirements

### Requirement: Log file written to OS-conventional directory

The system SHALL write a persistent log file to the platform's conventional application log directory, resolved via `app.path().app_log_dir()` using the app identifier `com.splode.pomotroid`.

#### Scenario: Log directory on Linux

- **WHEN** Pomotroid runs on Linux
- **THEN** log files are written under `~/.local/share/com.splode.pomotroid/logs/`

#### Scenario: Log directory on macOS

- **WHEN** Pomotroid runs on macOS
- **THEN** log files are written under `~/Library/Logs/com.splode.pomotroid/`

#### Scenario: Log directory on Windows

- **WHEN** Pomotroid runs on Windows
- **THEN** log files are written under `%APPDATA%\com.splode.pomotroid\logs\`

---

### Requirement: Log rotation limits disk usage

The system SHALL rotate log files using a 5 MB per-file size limit and SHALL retain at most one archived log file alongside the current file (KeepOne strategy), bounding total log disk usage to approximately 10 MB.

#### Scenario: File rotates at 5 MB

- **WHEN** the active log file reaches 5 MB
- **THEN** the active file is archived and a new log file is started

#### Scenario: Only one archive retained

- **WHEN** a rotation occurs and an archived log file already exists
- **THEN** the previous archive is replaced by the newly archived file

---

### Requirement: Default log level captures errors, warnings, and key lifecycle events

The system SHALL default to INFO log level, capturing `error`, `warn`, and `info` messages while suppressing `debug` messages when Verbose Logging is disabled.

#### Scenario: Error logged at INFO level

- **WHEN** Verbose Logging is disabled and a runtime error occurs
- **THEN** the error is written to the log file

#### Scenario: Debug suppressed at INFO level

- **WHEN** Verbose Logging is disabled
- **THEN** debug-level messages are not written to the log file

---

### Requirement: Rust panics captured before process termination

The system SHALL install a custom panic hook that writes the panic information to the log file before the process terminates.

#### Scenario: Panic captured in log

- **WHEN** a Rust panic occurs anywhere in the process
- **THEN** a log entry at ERROR level containing the panic message and location is written to the log file before termination

---

### Requirement: Startup metadata logged

The system SHALL log the following information at INFO level on every startup: application version, resolved app data directory path, and successful database open.

#### Scenario: Startup info in log

- **WHEN** the application starts successfully
- **THEN** the log contains the app version, data directory path, and a DB open success message

#### Scenario: DB open failure logged

- **WHEN** the database cannot be opened
- **THEN** the failure is logged at ERROR level before the process exits

---

### Requirement: All Rust error paths instrumented

The system SHALL replace all existing `eprintln!` calls with `log::` macro calls at the appropriate level. Every code path that can produce a `Result::Err` or an unrecoverable condition SHALL emit a log entry.

#### Scenario: Audio failure logged

- **WHEN** the audio output stream cannot be opened
- **THEN** a WARN-level entry is written (audio is non-fatal)

#### Scenario: WebSocket bind failure logged

- **WHEN** the WebSocket server fails to bind to its configured port
- **THEN** an ERROR-level entry is written including the address and error detail

#### Scenario: Shortcut registration failure logged

- **WHEN** a global shortcut cannot be registered
- **THEN** a WARN-level entry is written with the key string and error detail

#### Scenario: Tray build failure logged

- **WHEN** the system tray icon cannot be built
- **THEN** a WARN-level entry is written with the error detail

#### Scenario: Theme watcher failure logged

- **WHEN** the file system watcher for custom themes cannot be created or fails
- **THEN** a WARN-level entry is written

#### Scenario: Notification failure logged

- **WHEN** a desktop notification cannot be sent
- **THEN** a WARN-level entry is written

#### Scenario: Timer session record failure logged

- **WHEN** writing a completed session to the database fails
- **THEN** an ERROR-level entry is written

---

### Requirement: Major successful operations logged at INFO

The system SHALL emit INFO-level log entries for major lifecycle operations that succeed, including: database open, WebSocket server successfully bound, timer round completion, and settings save.

#### Scenario: WebSocket server bind logged

- **WHEN** the WebSocket server successfully binds to a port
- **THEN** an INFO-level entry records the bound address

#### Scenario: Timer round completion logged

- **WHEN** a work or break round completes naturally or is skipped
- **THEN** an INFO-level entry records the round type and completion reason

---

### Requirement: JS-side events logged to the same file

The system SHALL use `@tauri-apps/plugin-log` in both Svelte windows to forward frontend log calls to the same log file. JS logging SHALL apply the same level discipline as Rust: errors on failure paths, info on successful major operations.

#### Scenario: Frontend IPC error logged

- **WHEN** an IPC call from a Svelte window fails
- **THEN** an error-level entry is written to the log file from the frontend context

#### Scenario: Frontend initialization logged

- **WHEN** a Svelte window completes its startup sequence (settings loaded, theme applied)
- **THEN** an info-level entry is written confirming successful initialization

#### Scenario: Locale change logged

- **WHEN** the active locale is changed
- **THEN** an info-level entry records the new locale value

---

### Requirement: Open Log Folder accessible from Settings → About

The system SHALL provide a button in Settings → About that opens the log directory in the OS file manager.

#### Scenario: Open Log Folder button opens file manager

- **WHEN** the user clicks "Open Log Folder" in Settings → About
- **THEN** the OS file manager opens at the log directory

#### Scenario: Log path displayed

- **WHEN** the user views Settings → About
- **THEN** the resolved log directory path is displayed as text alongside the button
