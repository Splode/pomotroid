## Requirements

### Requirement: Update manifest hosted in repository

The system SHALL provide a `latest.json` file at the root of the `main` branch following the Tauri updater manifest format. The manifest SHALL contain the latest release version, per-platform download URLs, and Ed25519 signatures for each platform bundle. CI SHALL generate and commit this file after all platform bundles are successfully uploaded.

#### Scenario: Manifest is present after a release build

- **WHEN** a release CI run completes successfully
- **THEN** `latest.json` SHALL exist at the repo root on `main` and SHALL contain valid entries for Windows (NSIS), macOS (DMG), and Linux (AppImage)

#### Scenario: Manifest version matches release tag

- **WHEN** the CI run is triggered by a version tag (e.g. `v1.2.0`)
- **THEN** the `version` field in `latest.json` SHALL equal `1.2.0`

---

### Requirement: Bundles are Ed25519-signed during CI

The system SHALL sign release bundles with an Ed25519 private key stored in GitHub Secrets (`TAURI_SIGNING_PRIVATE_KEY`) during every release CI run. The corresponding public key SHALL be embedded in `tauri.conf.json` under the updater plugin configuration.

#### Scenario: Updater rejects tampered bundle

- **WHEN** the downloaded bundle's signature does not match the public key
- **THEN** `tauri-plugin-updater` SHALL abort the installation and report an error

#### Scenario: Updater accepts correctly signed bundle

- **WHEN** the downloaded bundle's signature matches the public key
- **THEN** installation proceeds normally

---

### Requirement: `check_update` Tauri command

The system SHALL expose a `check_update` Tauri command that queries `latest.json` and returns either a serialized update descriptor (version, body, date) or `null` if the current version is already the latest.

#### Scenario: Update available

- **WHEN** `check_update` is called and `latest.json` contains a version higher than the running app version
- **THEN** the command SHALL return an object with at least `{ version: string, body: string | null, date: string | null }`

#### Scenario: Already up to date

- **WHEN** `check_update` is called and `latest.json` version equals the running app version
- **THEN** the command SHALL return `null`

#### Scenario: Network unreachable

- **WHEN** `check_update` is called and the manifest URL is unreachable
- **THEN** the command SHALL return an error that the frontend can catch and display as a non-blocking message

---

### Requirement: `install_update` Tauri command

The system SHALL expose an `install_update` Tauri command that downloads, verifies, and installs the pending update, then relaunches the application.

#### Scenario: Install completes and app relaunches

- **WHEN** `install_update` is called after `check_update` returned an available update
- **THEN** the bundle is downloaded, signature verified, installed, and the app relaunches to the new version

---

### Requirement: Update check UI in Settings → About

The system SHALL display update status in the Settings → About section. On mount, if `check_for_updates` is enabled, the section SHALL silently call `check_update`. The status SHALL transition through states: idle → checking → up-to-date / update-available. When an update is available, an "Install vX.Y.Z" button SHALL appear. Clicking it SHALL call `install_update`.

#### Scenario: No update available

- **WHEN** the About section mounts and `check_update` returns `null`
- **THEN** the update row SHALL display "Up to date" (or equivalent) with the current version

#### Scenario: Update available — install button shown

- **WHEN** `check_update` returns an available update version
- **THEN** the About section SHALL show an "Install vX.Y.Z" button where X.Y.Z is the available version

#### Scenario: Check disabled — no check performed

- **WHEN** `check_for_updates` is `false` and the About section mounts
- **THEN** `check_update` SHALL NOT be called and the update row SHALL display a static version with no checking indicator

#### Scenario: Check fails gracefully

- **WHEN** `check_update` returns an error (e.g. network failure)
- **THEN** the update row SHALL display a non-blocking error message and SHALL NOT throw or crash the settings window

---

### Requirement: Updater capability registered

The system SHALL declare the `updater` permission in `src-tauri/capabilities/default.json` so that the settings window can invoke `check_update` and `install_update`.

#### Scenario: Commands callable from settings window

- **WHEN** the settings window invokes `check_update` or `install_update`
- **THEN** Tauri SHALL not reject the call due to missing capability permissions
