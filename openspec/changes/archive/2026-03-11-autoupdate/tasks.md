## 1. Rust dependencies and plugin registration

- [x] 1.1 Add `tauri-plugin-updater` to `src-tauri/Cargo.toml` dependencies
- [x] 1.2 Register the updater plugin in `src-tauri/src/lib.rs` (`.plugin(tauri_plugin_updater::Builder::new().build())`)

## 2. Signing key setup

- [x] 2.1 Generate an Ed25519 keypair with `npm run tauri -- signer generate` and save the output (public key for `tauri.conf.json`, private key for GitHub Secrets) — MANUAL STEP (requires interactive TTY)
- [x] 2.2 Add the public key to `tauri.conf.json` under `plugins.updater.pubkey` (placeholder — replace after running 2.1)
- [x] 2.3 Add the updater endpoint URL to `tauri.conf.json` under `plugins.updater.endpoints` pointing to the raw `latest.json` on `main`

## 3. Settings: DB migration and struct

- [x] 3.1 Add a new migration in `src-tauri/src/db/migrations.rs` that inserts `check_for_updates = 'true'` (INSERT OR IGNORE pattern)
- [x] 3.2 Add `check_for_updates: bool` field to the `Settings` struct in `src-tauri/src/settings/mod.rs`
- [x] 3.3 Add the DB key → struct field mapping in `settings/mod.rs` `load()` (DB key `check_for_updates`, default `true`)
- [x] 3.4 Add `check_for_updates: boolean` to the `Settings` interface in `src/lib/types.ts`

## 4. Tauri commands

- [x] 4.1 Add a `check_update` async Tauri command in `src-tauri/src/commands.rs` that uses `tauri_plugin_updater::UpdaterExt` to check the endpoint and returns `Option<UpdateInfo>` where `UpdateInfo` is a serializable struct with `version`, `body`, and `date` fields
- [x] 4.2 Add an `install_update` async Tauri command that downloads, verifies, and installs the pending update, then calls `app.restart()`
- [x] 4.3 Register both commands in the `.invoke_handler` in `src-tauri/src/lib.rs`

## 5. Capabilities

- [x] 5.1 Add `"updater:default"` (or the appropriate Tauri 2 permission identifier) to `src-tauri/capabilities/default.json`

## 6. Frontend IPC wrappers

- [x] 6.1 Add typed `checkUpdate(): Promise<UpdateInfo | null>` and `installUpdate(): Promise<void>` wrappers in `src/lib/ipc/index.ts`

## 7. Settings UI

- [x] 7.1 Add a "Check for Updates Automatically" `SettingsToggle` to `SystemSection.svelte` (DB key `check_for_updates`) in the System section
- [x] 7.2 Update the `About` section (`AboutSection.svelte`) to call `checkUpdate()` on mount when `$settings.check_for_updates` is true, manage update state (`idle | checking | up-to-date | available | error`) with Svelte 5 `$state`, and render the appropriate status row (checking spinner → "Up to date" or "Install vX.Y.Z" button)
- [x] 7.3 Wire the "Install vX.Y.Z — app will restart" button to call `installUpdate()` and show a brief "Installing…" state; the app relaunches immediately upon completion

## 8. CI: generate and commit `latest.json`

- [x] 8.1 Add a CI step in the release workflow (`.github/workflows/`) that generates `latest.json` from the signed build artifacts using the Tauri CLI or a manifest-generation script, including platform URLs and Ed25519 signatures
- [x] 8.2 Add a CI step that commits `latest.json` to `main` (same pattern as the Scoop manifest `pomotroid.json` commit step)

## 9. Verify

- [x] 9.1 Run `npm run check` to confirm no TypeScript type errors from the new settings field
- [x] 9.2 Run `cargo test` in `src-tauri/` to confirm migrations compile and run without errors
- [x] 9.3 Test manually in `npm run tauri dev`: confirm the update check toggle appears in System, the About section shows "Up to date" (or checking state) on open, and the toggle prevents the check when disabled
