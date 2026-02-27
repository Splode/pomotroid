## 1. Build Script

- [x] 1.1 In `src-tauri/build.rs`, add a function that runs `git describe --tags --long --always --dirty` and captures stdout
- [x] 1.2 Parse the describe output into its components: base version tag, commit count, short SHA, dirty flag
- [x] 1.3 Read the base version from `tauri.conf.json` as the fallback source of truth
- [x] 1.4 Implement the semver formatting logic: `{base}-dev.{n}+{sha}` for dev, `{base}+{sha}` for release, `.dirty` suffix when applicable
- [x] 1.5 Handle the fallback case (`git describe` fails or no tags): emit `{base}+unknown`
- [x] 1.6 Run `git rev-parse HEAD` and capture the full 40-char SHA
- [x] 1.7 Emit `cargo:rustc-env=APP_BUILD_VERSION=<string>` with the computed semver string
- [x] 1.8 Emit `cargo:rustc-env=APP_BUILD_SHA=<full-sha>` with the full commit SHA
- [x] 1.9 Emit `cargo:rerun-if-changed=.git/HEAD` to invalidate cache on new commits
- [x] 1.10 Emit `cargo:rerun-if-changed=.git/refs/` to invalidate cache on tag changes
- [x] 1.11 Keep the existing `tauri_build::build()` call intact

## 2. Rust Backend

- [x] 2.1 In `src-tauri/src/commands.rs`, add `app_version` command that returns `env!("APP_BUILD_VERSION")` as `&'static str`
- [x] 2.2 Register `app_version` in the `tauri::Builder::invoke_handler` list in `src-tauri/src/lib.rs`
- [x] 2.3 In `src-tauri/src/lib.rs` startup, add an INFO log entry: `[app] version={APP_BUILD_VERSION} sha={APP_BUILD_SHA}`
- [x] 2.4 Align `src-tauri/Cargo.toml` version to `1.0.0` to match `tauri.conf.json`

## 3. Frontend IPC

- [x] 3.1 In `src/lib/ipc/index.ts`, add `appVersion(): Promise<string>` wrapper that invokes `app_version`

## 4. About Section

- [x] 4.1 In `AboutSection.svelte`, remove the hardcoded `const VERSION = '1.0.0'` constant
- [x] 4.2 Add `import { appVersion } from '$lib/ipc'`
- [x] 4.3 Declare `let version = $state('...')` with a loading placeholder
- [x] 4.4 In `onMount`, call `appVersion()` and set `version` from the result; on error, fall back to the base version from `tauri.conf.json`
- [x] 4.5 Update the version display to use the reactive `version` state variable
- [x] 4.6 Update `RELEASE_URL` to derive the tag from the base version only (strip pre-release/build metadata before constructing the GitHub releases URL)

## 5. CI Workflow

- [x] 5.1 In `.github/workflows/build.yml`, add `fetch-depth: 0` to the Linux `actions/checkout@v4` step
- [x] 5.2 Add `fetch-depth: 0` to the macOS `actions/checkout@v4` step
- [x] 5.3 Add `fetch-depth: 0` to the Windows `actions/checkout@v4` step

## 6. Verification

- [x] 6.1 Run `cargo check` in `src-tauri/` — no errors
- [x] 6.2 Run `npm run check` — no type errors
- [ ] 6.3 Run `npm run tauri dev`, open Settings → About, confirm version string shows git-derived value (not `1.0.0`)
- [ ] 6.4 Confirm the log file on startup contains a line with the full SHA
