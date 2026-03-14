## Context

Pomotroid currently has no mechanism to notify users of new releases. Users must manually monitor GitHub or the releases page. Tauri 2 ships a first-party updater plugin (`tauri-plugin-updater`) that handles Ed25519-signed update manifests, bundle downloading, and installation. The project already has a CI pipeline (GitHub Actions) that builds release bundles and commits a Scoop manifest (`pomotroid.json`) back to `main` — the autoupdate manifest will follow the same pattern.

## Goals / Non-Goals

**Goals:**
- Detect available updates automatically when the user opens Settings
- Let the user install with a single button click (no silent forced updates)
- Provide an opt-out setting for users who prefer manual updates
- Use Tauri's built-in updater to avoid reinventing download/verify/install logic
- Keep the update manifest (`latest.json`) in the same repo, committed by CI alongside the Scoop manifest

**Non-Goals:**
- Silent/forced background updates without user interaction
- Update support for `.deb` or `.rpm` packages (Tauri updater does not support them)
- Delta/partial updates — full bundle replacement only
- Pre-release / beta channel support
- Update checking from the main timer window

## Decisions

### D1: Use `tauri-plugin-updater` rather than a custom HTTP check
**Decision**: Use the official Tauri 2 updater plugin.
**Rationale**: It handles Ed25519 signature verification, cross-platform bundle download and install, and the Tauri app relaunch flow out of the box. Writing equivalent logic from scratch would be higher risk and higher maintenance. The plugin is actively maintained by the Tauri team and is the documented approach for Tauri 2 apps.
**Alternative considered**: A custom `reqwest`-based check that just reads `latest.json` and opens a browser link. Rejected because it provides no install-in-app path and requires duplicating version comparison logic.

### D2: Host `latest.json` committed to `main` (not a GitHub Release asset)
**Decision**: CI generates and commits `latest.json` to the repo root on every release build.
**Rationale**: Pomotroid already does this for `pomotroid.json` (Scoop manifest). Same CI pattern, same atomic commit, no extra GitHub Releases API calls needed from the updater. The raw GitHub URL for a file on `main` is stable and version-independent.
**Alternative considered**: Attach `latest.json` as a GitHub Release asset. More semantically correct but requires the updater endpoint URL to change per release or use a permanent redirect, adding complexity.
**Alternative considered**: Use `releases.tauri.app` (Tauri's hosted manifest service). Adds an external service dependency; Option A is simpler and self-contained.

### D3: Check on settings window open, not at app startup
**Decision**: Trigger the update check when the settings window opens, specifically in `AboutSection.svelte`'s `onMount`.
**Rationale**: The main timer window is the primary UX and should not be interrupted. Users who open Settings are already in a reflective/administrative mindset. Avoids startup latency for the common case (running a Pomodoro session with no intention to update).
**Alternative considered**: Check at app startup (main window). Rejected — adds startup latency and distracts from the primary workflow.

### D4: Hybrid UX — silent check, explicit install action
**Decision**: The check runs silently in the background. If an update is found, the About section shows an "Install vX.Y.Z" button. No toast, no blocking dialog.
**Rationale**: Non-intrusive. The user is already in Settings → About when the result appears. They can act or ignore. Forced prompts are annoying in a focus app.

### D5: `check_for_updates` setting defaults to `true`
**Decision**: Auto-checking is on by default, with an opt-out toggle in Settings → System.
**Rationale**: Most users benefit from updates and won't configure this. Power users who prefer manual control can disable it. Defaulting to `false` would mean most users never see updates.

### D6: Update check and install are frontend-initiated via Tauri command
**Decision**: A single `check_update` Tauri command returns a serialized update result (or null). The install step is a separate `install_update` command (or handled via a JS method on the returned update object).
**Rationale**: The frontend owns the UX state machine (idle → checking → available/none → installing → relaunching). Keeping this logic on the frontend avoids adding background Rust threads and makes the state transitions easy to track with Svelte runes.

### D7: Ed25519 keypair — one-time generation, private key in GitHub Secrets
**Decision**: Generate an Ed25519 keypair once using `tauri signer generate`. Store the private key as `TAURI_SIGNING_PRIVATE_KEY` in GitHub Secrets. Embed the public key in `tauri.conf.json`.
**Rationale**: Standard Tauri signing workflow. Private key never leaves GitHub Secrets. Public key in config allows the runtime to verify bundle integrity before installing.

## Risks / Trade-offs

- **[Risk] CI key compromise** → The private signing key in GitHub Secrets could be extracted by a malicious workflow. Mitigation: restrict secret access to protected branches only; rotate key if compromise is suspected (requires a `latest.json` update with new public key in a new release).
- **[Risk] `latest.json` stale after failed CI** → If CI fails mid-release, `latest.json` may point to partially uploaded bundles. Mitigation: CI step order — commit `latest.json` only after all platform bundles are uploaded successfully.
- **[Risk] Linux AppImage only** → Users who installed via `.deb`/`.rpm` will not receive in-app updates. Mitigation: document this limitation in the About section or release notes. Those users rely on their package manager.
- **[Risk] GitHub raw content CDN caching** → `raw.githubusercontent.com` has up to 5-minute cache. Mitigation: acceptable latency for a non-critical update check. Could add `?nocache=<timestamp>` query param if needed.
- **[Trade-off] Relaunch required** → Tauri's updater installs on next launch (or relaunches immediately). Users mid-session will lose their current round context. Mitigation: the "Install" button can note "App will restart". Session state is ephemeral by design.

## Migration Plan

1. Generate Ed25519 keypair locally; add public key to `tauri.conf.json`; store private key in GitHub Secrets
2. Add `tauri-plugin-updater` to `Cargo.toml` and register in `lib.rs`
3. Add DB migration for `check_for_updates` setting
4. Add `check_update` and `install_update` Tauri commands
5. Wire frontend: `AboutSection.svelte` update UI + `SystemSection.svelte` opt-out toggle
6. Update CI to sign bundles and generate/commit `latest.json`
7. Add `updater` permission to `capabilities/default.json`
8. First release after this ships will populate `latest.json`; subsequent releases will trigger in-app notifications

**Rollback**: Remove the `check_for_updates` DB key via a new migration. The UI change is purely additive — no rollback needed there. `latest.json` commits to main are benign if the plugin is disabled.

## Open Questions

- Key rotation procedure: if the signing private key needs to be rotated, what is the documented process for publishing a new public key in a release that old clients can trust?

## Resolved Decisions

- **Relaunch behavior**: Clicking "Install" triggers an immediate app relaunch (not a deferred install-on-next-launch). The button label should make this clear (e.g., "Install vX.Y.Z — app will restart").
