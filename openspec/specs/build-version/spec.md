## ADDED Requirements

### Requirement: Build version string baked in at compile time

The system SHALL compute a semver-conformant build version string during `cargo build` by invoking `git describe --tags --long --always --dirty` in `build.rs` and emitting the result as the compile-time environment variable `APP_BUILD_VERSION`. The full commit SHA SHALL be emitted separately as `APP_BUILD_SHA` via `git rev-parse HEAD`.

#### Scenario: Dev build (commits since last tag)

- **WHEN** the binary is compiled from a commit that is N > 0 commits after the most recent tag
- **THEN** `APP_BUILD_VERSION` SHALL be `{base}-dev.{N}+{short-sha}` (e.g. `1.0.0-dev.80+20b2d87`)

#### Scenario: Release build (on exact tag)

- **WHEN** the binary is compiled from a commit that is exactly at a tag
- **THEN** `APP_BUILD_VERSION` SHALL be `{base}+{short-sha}` (e.g. `1.0.0+20b2d87`)

#### Scenario: Dirty working tree

- **WHEN** the binary is compiled with uncommitted changes present
- **THEN** `APP_BUILD_VERSION` SHALL include a `.dirty` suffix in the build metadata (e.g. `1.0.0-dev.80+20b2d87.dirty`)

#### Scenario: No git history or no tags

- **WHEN** `git describe` fails (no git binary, no tags, detached shallow clone)
- **THEN** `APP_BUILD_VERSION` SHALL fall back to `{base}+unknown` where `{base}` is read from `tauri.conf.json`

#### Scenario: Incremental rebuild after a new commit

- **WHEN** a new commit is made and `cargo build` is run again
- **THEN** the build script SHALL re-execute and produce an updated `APP_BUILD_VERSION` reflecting the new commit

---

### Requirement: Build version exposed via IPC command

The system SHALL provide a Tauri command `app_version` that returns `APP_BUILD_VERSION` as a `&'static str`. This command SHALL be callable by the frontend at any time and returns the compile-time-baked version string.

#### Scenario: Command returns baked version

- **WHEN** the frontend invokes `app_version`
- **THEN** the response SHALL be the `APP_BUILD_VERSION` string compiled into the binary

---

### Requirement: Build version displayed in Settings → About

The system SHALL display the build version string in Settings → About in place of the previously hardcoded version constant. The displayed string SHALL use the short-SHA form (7 characters).

#### Scenario: Version displayed on About mount

- **WHEN** the user opens Settings → About
- **THEN** the version line SHALL show the full semver build string (e.g. `1.0.0-dev.80+20b2d87`)

#### Scenario: Version string is never empty

- **WHEN** `app_version` returns successfully
- **THEN** the About section SHALL display the returned string; if the call fails, it SHALL fall back to displaying the base version from `tauri.conf.json`

---

### Requirement: Full commit SHA logged on startup

The system SHALL log the full 40-character commit SHA (from `APP_BUILD_SHA`) at INFO level during application startup, alongside the build version string.

#### Scenario: Startup log includes full SHA

- **WHEN** the application starts
- **THEN** the log file SHALL contain an INFO entry with both the build version string and the full commit SHA (e.g. `[app] version=1.0.0-dev.80+20b2d87 sha=20b2d87173870d939002efe84fddff2e944eabd6`)

---

### Requirement: CI workflow fetches full git history

The CI build workflow SHALL use `fetch-depth: 0` on all checkout steps so that `git describe` has access to full tag history and can compute commit distances in CI-produced artifacts.

#### Scenario: CI artifact carries commit count

- **WHEN** a binary is built in CI from a commit that is N commits after the last tag
- **THEN** the binary's `APP_BUILD_VERSION` SHALL include the commit count N (e.g. `1.0.0-dev.80+abc1234`)
