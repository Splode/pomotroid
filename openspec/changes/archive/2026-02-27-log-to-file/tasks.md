## 1. Dependencies and Cargo cleanup

- [x] 1.1 Remove `tracing` and `tracing-subscriber` from `src-tauri/Cargo.toml`
- [x] 1.2 Verify `tauri-plugin-log = "2"` is present in `Cargo.toml` (already declared)
- [x] 1.3 Add `@tauri-apps/plugin-log` to `package.json` and install

## 2. Settings schema — verbose_logging

- [x] 2.1 Add `("verbose_logging", "false")` to `defaults::DEFAULTS` in `src-tauri/src/settings/defaults.rs`
- [x] 2.2 Add `pub verbose_logging: bool` field to the `Settings` struct in `src-tauri/src/settings/mod.rs`
- [x] 2.3 Add `verbose_logging: parse_bool(&map, "verbose_logging", false)` to `settings::load()`
- [x] 2.4 Add `Default` value `verbose_logging: false` to `Settings::default()`
- [x] 2.5 Add Migration 4 to `src-tauri/src/db/migrations.rs`: `INSERT OR IGNORE INTO settings (key, value) VALUES ('verbose_logging', 'false')`
- [x] 2.6 Update `migrations::run()` to apply migration 4 when `version < 4`
- [x] 2.7 Update `settings` tests: add `assert!(!s.verbose_logging)` to `defaults_round_trip`; update migration idempotency test to expect version 4

## 3. Logging infrastructure — Rust

- [x] 3.1 Register `tauri_plugin_log` as the first plugin in the builder chain in `src-tauri/src/lib.rs`, configured with: file target (`LogDir`, filename `"pomotroid"`), `max_file_size(5 * 1024 * 1024)`, `RotationStrategy::KeepOne`, `LevelFilter::Debug` ceiling
- [x] 3.2 In `setup()`, after loading `initial_settings`, call `log::set_max_level(LevelFilter::Debug)` if `verbose_logging` is true, else `log::set_max_level(LevelFilter::Info)`
- [x] 3.3 In `setup()`, register a panic hook: `std::panic::set_hook(Box::new(|info| { log::error!("PANIC: {info}"); }))`
- [x] 3.4 Log startup metadata in `setup()`: app version (`env!("CARGO_PKG_VERSION")`), resolved `app_data_dir` path, and DB open success at INFO level

## 4. Rust instrumentation — replace eprintln! and add error logging

- [x] 4.1 `src-tauri/src/websocket/mod.rs`: replace 3× `eprintln!` with `log::error!`/`log::warn!`; add `log::info!` when server successfully binds (include address)
- [x] 4.2 `src-tauri/src/notifications/mod.rs`: replace `eprintln!` with `log::warn!`
- [x] 4.3 `src-tauri/src/timer/mod.rs`: replace `eprintln!` with `log::error!`; add `log::info!` on round completion (include round type and whether skipped)
- [x] 4.4 `src-tauri/src/tray/mod.rs`: replace 4× `eprintln!` with `log::warn!`
- [x] 4.5 `src-tauri/src/audio/mod.rs`: replace 5× `eprintln!` with `log::warn!`
- [x] 4.6 `src-tauri/src/shortcuts/mod.rs`: replace 2× `eprintln!` with `log::warn!`
- [x] 4.7 `src-tauri/src/themes/watcher.rs`: replace 5× `eprintln!` with `log::warn!`/`log::error!`
- [x] 4.8 `src-tauri/src/commands.rs`: add `log::error!` for any command that returns an error result (settings save, stats queries, audio commands)

## 5. verbose_logging runtime toggle — Rust command

- [x] 5.1 In `src-tauri/src/commands.rs`, add a special case in the `settings_set` handler (or a dedicated handler) for the `verbose_logging` key: after saving to DB, call `log::set_max_level()` and emit an INFO log entry announcing the change

## 6. New Tauri command — open_log_dir

- [x] 6.1 Add `open_log_dir` command in `src-tauri/src/commands.rs`: resolve `app.path().app_log_dir()`, call `tauri_plugin_opener::open_path(log_dir, None::<&str>)`, log a warning if resolution fails
- [x] 6.2 Add `get_log_dir` command in `src-tauri/src/commands.rs`: resolve and return `app.path().app_log_dir()` as a `String` (for displaying the path in the UI)
- [x] 6.3 Register `open_log_dir` and `get_log_dir` in the `invoke_handler!` macro in `lib.rs`

## 7. Capabilities

- [x] 7.1 Add `"log:default"` (or equivalent `tauri-plugin-log` permission) to `src-tauri/capabilities/default.json`

## 8. Frontend types and IPC

- [x] 8.1 Add `verbose_logging: boolean` to the `Settings` interface in `src/lib/types.ts`
- [x] 8.2 Add `openLogDir()` and `getLogDir()` wrappers to `src/lib/ipc/index.ts`

## 9. JS-side logging

- [x] 9.1 In `src/routes/+page.svelte`: import `{ info, warn, error }` from `@tauri-apps/plugin-log`; add `info()` on successful init (settings loaded, theme applied, locale set); add `error()` on IPC failures in `onMount`
- [x] 9.2 In `src/routes/settings/+page.svelte`: same pattern — `info()` on init, `error()` on IPC failures
- [x] 9.3 In `src/lib/locale.svelte.ts`: add `info()` call when locale changes

## 10. Settings UI — Verbose Logging toggle

- [x] 10.1 Add `verbose_logging` toggle to `src/lib/components/settings/sections/AdvancedSection.svelte` using the same `Toggle` component pattern as other boolean settings
- [x] 10.2 Add locale keys `"settings_verbose_logging"` and `"settings_verbose_logging_desc"` to all 5 locale files (`messages/en.json`, `es.json`, `fr.json`, `de.json`, `ja.json`) and recompile Paraglide
- [x] 10.3 Use the new locale keys in `AdvancedSection.svelte`

## 11. Settings UI — Open Log Folder (About)

- [x] 11.1 Add `get_log_dir` call in `onMount` of `src/routes/settings/+page.svelte` (or inside `AboutSection.svelte`) to resolve and display the log path
- [x] 11.2 Add "Open Log Folder" button and log path display to `src/lib/components/settings/sections/AboutSection.svelte`; button calls `openLogDir()` IPC wrapper
- [x] 11.3 Add locale keys `"settings_open_log_folder"` and `"settings_log_path"` to all 5 locale files and recompile Paraglide
- [x] 11.4 Use the new locale keys in `AboutSection.svelte`

## 12. Validation

- [x] 12.1 Run `cargo test` in `src-tauri/` — all tests pass including updated migration and settings tests
- [x] 12.2 Run `npm run check` — 0 errors, 0 warnings
