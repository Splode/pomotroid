## Why

Users have no way to know when a new Pomotroid version is available short of manually checking GitHub releases. Adding automatic update checking removes that friction and ensures users stay current with bug fixes and new features without needing to reinstall manually.

## What Changes

- Add `tauri-plugin-updater` dependency (Rust + JS) for in-app update checking and installation
- Add a new `check_for_updates` boolean setting (default: `true`) persisted in SQLite
- Add a DB migration to seed the new setting
- Add a `check_update` Tauri command that queries `latest.json` hosted in the repo and returns available update info or null
- Expose a `check_for_updates` toggle in Settings → System
- Add update status UI in Settings → About: silent background check on settings window open (if enabled), "Install vX.Y.Z" button appears when an update is found
- Add CI step: after `tauri build`, generate `latest.json` manifest (platform URLs + Ed25519 signatures) and commit it to `main`
- Ed25519 signing keypair: private key stored in GitHub Secrets (`TAURI_SIGNING_PRIVATE_KEY`), public key embedded in `tauri.conf.json`

## Capabilities

### New Capabilities

- `autoupdate`: In-app update detection and installation via Tauri's updater plugin. Covers the check_update command, latest.json manifest format, CI signing and publishing flow, and the opt-out setting.

### Modified Capabilities

- `settings`: New `check_for_updates` boolean setting added to the settings schema.

## Impact

- **Rust**: `src-tauri/Cargo.toml` (add `tauri-plugin-updater`), `src-tauri/src/lib.rs` (register plugin), `src-tauri/src/commands.rs` (new `check_update` command), `src-tauri/src/settings/mod.rs` (new field), `src-tauri/src/db/migrations.rs` (new migration)
- **Frontend**: `src/lib/ipc/index.ts` (typed wrapper), `src/lib/types.ts` (Settings type), `src/lib/components/settings/sections/AboutSection.svelte` (update UI), `src/lib/components/settings/sections/SystemSection.svelte` (opt-out toggle)
- **Config**: `src-tauri/tauri.conf.json` (updater plugin config, public key, endpoints)
- **CI**: `.github/workflows/` (sign bundles, generate and commit `latest.json`)
- **New file**: `latest.json` at repo root (committed by CI, read by updater)
- **Platform support**: Windows (NSIS), macOS (DMG), Linux (AppImage) — `.deb` and `.rpm` are not supported by Tauri's updater
