## ADDED Requirements

### Requirement: All three Linux release artifacts are GPG-signed in CI

The release CI pipeline SHALL produce a detached ASCII-armored GPG signature file (`.asc`) for each Linux artifact (`.deb`, `.rpm`, `.AppImage`) using a private key stored as a repository secret.

#### Scenario: Signing step produces three signature files

- **WHEN** a release tag matching `v[0-9]+.[0-9]+.[0-9]+` is pushed
- **THEN** the `build-linux` CI job SHALL produce `*.deb.asc`, `*.rpm.asc`, and `*.AppImage.asc` alongside the corresponding package files

#### Scenario: Private key is not persisted

- **WHEN** the signing step completes
- **THEN** the imported GPG key SHALL exist only in the ephemeral CI runner's keyring for the duration of the job and SHALL NOT appear in any artifact, log, or persistent storage

### Requirement: Signature files are distributed with release artifacts

The `.asc` signature files SHALL be uploaded to the GitHub Release alongside the corresponding packages so users can download both from the same release page.

#### Scenario: Release page includes all six files

- **WHEN** a draft GitHub Release is created by the release workflow
- **THEN** the release assets SHALL include `*.deb`, `*.deb.asc`, `*.rpm`, `*.rpm.asc`, `*.AppImage`, and `*.AppImage.asc`

### Requirement: Public key is available in the repository

The GPG public key used for signing SHALL be committed to the repository root as `public.asc` so users can import it without relying on a key server.

#### Scenario: Public key file is present

- **WHEN** a user clones or browses the repository
- **THEN** `public.asc` SHALL be present at the repository root containing the ASCII-armored public key

### Requirement: Verification and setup instructions are documented

A `SECURITY.md` file SHALL document the public key fingerprint, how to verify each artifact format, a note that repo-channel signing (APT, RPM, Flatpak, Snap, AUR) is handled separately, and maintainer instructions for key generation and rotation.

#### Scenario: User can verify a downloaded artifact

- **WHEN** a user opens `SECURITY.md`
- **THEN** the file SHALL contain the GPG key fingerprint and the `gpg --verify <file>.asc <file>` command for each of the three artifact formats

#### Scenario: Maintainer can set up or rotate the signing key

- **WHEN** a maintainer reads `SECURITY.md`
- **THEN** the file SHALL contain step-by-step commands for generating the keypair, exporting it, and configuring `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` as repository secrets
