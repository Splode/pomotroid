## Context

The About section of the settings window currently hardcodes `const VERSION = '1.0.0'` in `AboutSection.svelte`. This string is entirely disconnected from the actual build — there is no way to determine which commit a running binary was compiled from. Two binaries built from different commits show the same version string.

`build.rs` already exists (`src-tauri/build.rs`) and contains only `tauri_build::build()`. It is the correct and idiomatic place to perform git introspection at compile time.

The CI workflow uses `actions/checkout@v4` with default depth 1 (shallow clone), which prevents `git describe` from finding ancestor tags and computing commit distances. Full history is needed for commit count.

## Goals / Non-Goals

**Goals:**

- Every compiled binary carries a unique, traceable version string baked in at compile time.
- The string conforms to semver: `1.0.0-dev.{n}+{short-sha}` for dev builds, `1.0.0+{short-sha}` for release builds.
- Short SHA (7 chars) is displayed in Settings → About for readability.
- Full SHA is logged to the log file on startup for grep/traceability.
- `tauri.conf.json` remains the single source of truth for the base version number.
- Zero runtime overhead — the string is a compile-time constant.

**Non-Goals:**

- Automatic version bumping or tag management.
- Exposing branch name in the version string (too noisy, changes often).
- A separate build number counter outside of git commit distance.
- Changing the `tauri.conf.json` version value at build time.

## Decisions

### D1: `build.rs` as the injection point

**Decision**: Extend `src-tauri/build.rs` to run `git describe`, parse its output, and emit `cargo:rustc-env=APP_BUILD_VERSION=<string>`. The string is accessed in Rust via the compile-time macro `env!("APP_BUILD_VERSION")`.

**Alternatives considered**:

- _Vite `define` plugin_: Would work for the frontend, but the string would not be available in Rust (e.g., for startup logging). Split injection in two places introduces drift risk.
- _Runtime environment variable_: Requires the launching environment to set the variable; doesn't work for distributed binaries.
- _Patching `tauri.conf.json` before build_: Modifies a tracked file; requires cleanup; pollutes git diff.

`build.rs` is the correct Rust idiom. `env!()` is zero-cost. One source, accessible everywhere.

### D2: `git describe --tags --long --always --dirty` as the data source

**Decision**: Use `git describe --tags --long --always --dirty`.

- `--tags`: Match any tag (not just annotated).
- `--long`: Always output `{tag}-{count}-g{sha}` format, even when count is 0 (on the exact tag). Uniform parsing regardless of release/dev state.
- `--always`: Fall back to bare SHA if no tags exist at all.
- `--dirty`: Append `-dirty` if working tree has uncommitted changes.

Output format: `v1.0.0-80-g20b2d87[-dirty]`

Parsed into semver:
| count | dirty | result |
|-------|-------|--------|
| 0 | no | `1.0.0+20b2d87` |
| 0 | yes | `1.0.0+20b2d87.dirty` |
| N > 0 | no | `1.0.0-dev.N+20b2d87` |
| N > 0 | yes | `1.0.0-dev.N+20b2d87.dirty` |

The `g` prefix from git describe is stripped; build metadata contains a clean 7-char hex SHA.

**Fallback**: If `git describe` fails (no git binary, detached non-tagged repo), `build.rs` falls back to `{base_version}+unknown`. Base version is read from `tauri.conf.json` at build time.

### D3: `cargo:rerun-if-changed` triggers

**Decision**: Emit two rerun triggers:

```
cargo:rerun-if-changed=.git/HEAD
cargo:rerun-if-changed=.git/refs/
```

Without these, Cargo caches `build.rs` output and the version string becomes stale after commits. `.git/HEAD` changes on every commit and checkout. `.git/refs/` changes when tags are created or moved.

### D4: Short SHA in UI, full SHA in logs

**Decision**: The IPC command `app_version()` returns the version string with 7-char SHA (e.g., `1.0.0-dev.80+20b2d87`). A separate startup log line in `lib.rs` records the full 40-char SHA.

The full SHA is captured via `git rev-parse HEAD` in `build.rs` and emitted as `APP_BUILD_SHA` alongside `APP_BUILD_VERSION`.

### D5: New `app_version` Tauri command (no settings involvement)

**Decision**: Expose the build version via a new read-only command `app_version() -> &'static str`. It is not a setting; it does not go through the settings system. The frontend calls it once on About section mount.

### D6: CI checkout depth

**Decision**: Add `fetch-depth: 0` to all three `actions/checkout@v4` steps in `build.yml`. This gives the CI full tag history, enabling commit count in CI-produced artifacts. Without it, `git describe` falls back to the SHA-only path and the commit count is absent.

## Risks / Trade-offs

- **`-dirty` in release binaries**: If someone builds a release from a dirty working tree, the binary is labeled `dirty`. This is accurate and intentional — it's a signal, not an error.
- **CI build time**: `fetch-depth: 0` fetches full history. On a project with few commits this is negligible. Worth monitoring if the repo grows very large.
- **No git binary at build time**: Unlikely in any normal dev or CI environment, but handled gracefully by the fallback to `+unknown`.
- **Shallow clone outside CI**: If someone clones with `--depth 1` locally and builds, they get `+unknown` or a bare SHA. Acceptable.

## Migration Plan

No runtime migration required. The change is entirely at compile time and in the UI display layer. No database changes, no settings changes, no breaking IPC changes. The new `app_version` command is additive.

Rollout: merge to master, next build picks up the new version string automatically.
