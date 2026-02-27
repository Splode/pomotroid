## Why

When users encounter issues running Pomotroid, there is no persistent diagnostic record — all runtime errors go to stderr and are lost in production. A structured log file makes it possible for users to capture and submit diagnostics when filing bug reports.

## What Changes

- Wire up the already-declared `tauri-plugin-log` dependency to write a rotating log file to the OS-conventional log directory
- Remove the unused `tracing` and `tracing-subscriber` dependencies
- Replace all `eprintln!` calls throughout the Rust backend with structured `log::` macro calls
- Add logging at every error path and major operation (DB open, timer events, audio, WebSocket, shortcuts, tray, themes, notifications)
- Add a panic hook so Rust panics are captured in the log before the process terminates
- Add a `verbose_logging` boolean setting (default `false`) that switches the log level between INFO and DEBUG at runtime and persists across restarts
- Add a new `open_log_dir` Tauri command that opens the log directory in the OS file manager
- Add JS-side logging via `@tauri-apps/plugin-log` in both Svelte windows, applying the same error/info discipline as the Rust side
- Add a "Verbose Logging" toggle to Settings → Advanced
- Add an "Open Log Folder" button to Settings → About

## Capabilities

### New Capabilities

- `diagnostic-logging`: Persistent rotating log file with OS-conventional path, runtime log-level control via Verbose Logging setting, and UI access points for users to retrieve logs for bug reports

### Modified Capabilities

- `settings`: New `verbose_logging` boolean setting added to the settings schema, SQLite DB, and frontend types

## Impact

- **Rust**: `src-tauri/Cargo.toml` (remove tracing deps), `src-tauri/src/lib.rs` (plugin init, panic hook), all modules under `src-tauri/src/` (instrumentation), `src-tauri/src/settings/` (new key), `src-tauri/src/db/` (migration), `src-tauri/src/commands.rs` (new command + verbose_logging handler)
- **Frontend**: `package.json` (add `@tauri-apps/plugin-log`), `src/lib/ipc/index.ts` (new command wrapper), `src/lib/types.ts` (Settings type), `src/routes/+page.svelte`, `src/routes/settings/+page.svelte`, `src/lib/locale.svelte.ts` (JS logging), `src/lib/components/settings/sections/AdvancedSection.svelte`, `src/lib/components/settings/sections/AboutSection.svelte`
- **Capabilities**: `src-tauri/capabilities/default.json` (log plugin permissions)
- **Localization**: All 5 locale files under `messages/` (new keys for verbose_logging and open_log_folder labels)
