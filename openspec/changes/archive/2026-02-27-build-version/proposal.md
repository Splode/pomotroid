## Why

The About screen currently shows a hardcoded `VERSION = '1.0.0'` string that provides no information about the exact build being run. When debugging issues, there is no way to tell which commit a binary was compiled from. A semver-conformant build identifier — baked in at compile time via `build.rs` — gives every binary a unique, traceable version string at zero runtime cost.

## What Changes

- `build.rs` is extended to call `git describe` at compile time, parse the output, and emit a `APP_BUILD_VERSION` environment variable baked into the binary.
- A new Tauri command `app_version()` exposes the build version string to the frontend.
- `AboutSection.svelte` replaces its hardcoded `VERSION` constant with an IPC call to `app_version()`.
- The full commit SHA is logged to the log file on startup, while the short SHA is displayed in the UI.
- The CI workflow gains `fetch-depth: 0` so commit counts are available in CI artifacts.
- `Cargo.toml` version is aligned to `1.0.0` to match `tauri.conf.json`.

## Capabilities

### New Capabilities

- `build-version`: Compile-time build version string derived from `git describe`, formatted as semver with pre-release and build metadata. Exposed via IPC and displayed in Settings → About.

### Modified Capabilities

_(none — no existing spec-level requirements change)_

## Impact

- **`src-tauri/build.rs`**: New git describe logic + `cargo:rerun-if-changed` triggers.
- **`src-tauri/src/commands.rs`**: New `app_version` command.
- **`src-tauri/src/lib.rs`**: Log full SHA on startup.
- **`src/lib/ipc/index.ts`**: New `appVersion()` wrapper.
- **`src/lib/components/settings/sections/AboutSection.svelte`**: Replace hardcoded version with IPC-fetched value.
- **`.github/workflows/build.yml`**: Add `fetch-depth: 0` to all checkout steps.
- **`src-tauri/Cargo.toml`**: Align version to `1.0.0`.
- No new dependencies; no DB migrations; no settings changes.
