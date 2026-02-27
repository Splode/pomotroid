## Context

Pomotroid currently has no persistent diagnostic output. All error handling uses `eprintln!` (stderr), which is discarded in production AppImage/dmg/exe builds. When users encounter bugs they have nothing to submit. `tauri-plugin-log` is already declared in `Cargo.toml` but never registered; `tracing` and `tracing-subscriber` are declared but entirely unused. The work is primarily wiring and instrumentation rather than new infrastructure.

## Goals / Non-Goals

**Goals:**
- Rotating log file written to the OS-conventional application log directory
- All existing `eprintln!` replaced with structured `log::` calls at the appropriate level
- Every error path and major lifecycle event instrumented
- Runtime-switchable log level (INFO ↔ DEBUG) controlled by a persisted `verbose_logging` setting
- Rust panics captured to the log file before process termination
- JS-side logging (Svelte windows) routed to the same file via `@tauri-apps/plugin-log`
- User can open the log directory from Settings → About

**Non-Goals:**
- Structured / machine-parseable log format (plain text is sufficient for bug reports)
- Remote log shipping or crash reporting services
- Log viewing UI within the app
- Per-module log-level filtering

## Decisions

### Use `tauri-plugin-log`, remove `tracing`

**Chosen:** `tauri-plugin-log` (already a declared dependency, uses the `log` crate).

`tracing` is a richer framework (spans, structured fields, async-native) but `tauri-plugin-log` does not integrate with it and adding a compatible bridge (`tracing-log`) adds complexity for no user-visible benefit. All instrumentation in Pomotroid is sequential enough that flat log lines are sufficient. `tracing` and `tracing-subscriber` are removed from `Cargo.toml`.

**Macros used:** `log::error!`, `log::warn!`, `log::info!`, `log::debug!` from the `log` crate (re-exported by the plugin).

---

### Log level strategy: Debug ceiling, runtime-switchable global gate

The plugin is initialized with `LevelFilter::Debug` as its ceiling (so the file writer can always accept DEBUG messages if the gate opens). The global `log::set_max_level()` gate is set at startup from the `verbose_logging` setting:

```
verbose_logging = false  →  log::set_max_level(LevelFilter::Info)
verbose_logging = true   →  log::set_max_level(LevelFilter::Debug)
```

When the setting is toggled at runtime, the `settings_set` command handler calls `log::set_max_level()` immediately — no restart required. A log line is emitted at INFO level each time the level changes, so the boundary is visible in the file.

**Why not TRACE?** TRACE-level `rodio`/`tokio` output from dependencies would dominate the file. DEBUG is sufficient and targeted.

---

### Rotation: KeepOne, 5 MB per file

`RotationStrategy::KeepOne` keeps the current log plus one archived file — a bounded ~10 MB total disk footprint. 5 MB per file is large enough to capture a full debugging session without excessive disk use.

---

### Log file path

`tauri-plugin-log` resolves via `app.path().app_log_dir()` using the app identifier `com.splode.pomotroid`:

| Platform | Path |
|---|---|
| Linux   | `~/.local/share/com.splode.pomotroid/logs/pomotroid.log` |
| macOS   | `~/Library/Logs/com.splode.pomotroid/pomotroid.log` |
| Windows | `%APPDATA%\com.splode.pomotroid\logs\pomotroid.log` |

---

### Panic hook placement

The hook is registered inside `setup()` after the plugin chain has run (the logger is initialized during `.plugin()` calls in the builder). Registering inside `setup()` guarantees the hook can call `log::error!` and reach the file writer.

```rust
std::panic::set_hook(Box::new(|info| {
    log::error!("PANIC: {info}");
}));
```

---

### `verbose_logging` as a persisted setting

Stored as `"true"/"false"` under the key `verbose_logging` in the settings SQLite table, with `false` as the default. Added via a new DB migration (Migration 4). Added to the `Settings` struct and `defaults::DEFAULTS`. The `settings_set` command already handles arbitrary key/value pairs; the only special handling needed is the `log::set_max_level()` side-effect when the key is `verbose_logging`.

---

### `open_log_dir` command

A new Tauri command that calls `tauri_plugin_opener::open_path(app.path().app_log_dir())`. The frontend calls it via `ipc/index.ts`. No return value needed — opener handles the OS file manager interaction. Gracefully logs a warning if the path cannot be resolved.

---

### JS-side logging

`@tauri-apps/plugin-log` (the JS counterpart) forwards `info()`, `warn()`, `error()`, `debug()` calls from the Svelte webviews to the same file via IPC. Applied at:
- `+page.svelte`: startup lifecycle, IPC errors, theme/locale initialization
- `settings/+page.svelte`: same
- `locale.svelte.ts`: locale changes

Discipline mirrors Rust: errors on failure paths, info on successful major operations.

## Risks / Trade-offs

- **`log::set_max_level()` is global** — affects all crates, including third-party deps (rodio, tokio). At DEBUG level, noisy crates may emit spurious lines. Mitigation: the plugin's per-target level filter (`LevelFilter::Debug`) provides a ceiling, and if dependency noise is a problem a module filter can be added in a follow-up.
- **Panic hook may not reach the file** — if the panic occurs on the logging thread itself or before plugin init. Mitigation: accepted risk; the hook covers the vast majority of panics.
- **5 MB may rotate mid-session** — a heavy debug session could roll the file, losing earlier entries. Mitigation: `KeepOne` means the previous file is still present; combined they cover ~10 MB.

## Migration Plan

1. DB Migration 4 added to `migrations.rs`: `INSERT OR IGNORE INTO settings (key, value) VALUES ('verbose_logging', 'false')`
2. `Settings` struct gains `verbose_logging: bool` field
3. `defaults::DEFAULTS` gains `("verbose_logging", "false")`
4. `settings::load()` gains `verbose_logging: parse_bool(&map, "verbose_logging", false)`
5. Plugin registered as first plugin in `lib.rs` builder chain (so logging is available to all subsequent setup code)
6. `log::set_max_level()` called in `setup()` after loading initial settings
7. Existing tests updated: `defaults_round_trip` asserts `!s.verbose_logging`

## Open Questions

- None. All decisions made during exploration.
