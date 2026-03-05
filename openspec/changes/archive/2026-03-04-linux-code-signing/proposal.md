## Why

Linux release artifacts (`.deb`, `.rpm`, `.AppImage`) are currently published unsigned, giving users no way to verify that packages haven't been tampered with between build and download. A unified GPG signing step in CI produces a detached ASCII-armored signature (`.asc`) for each artifact, providing consistent verifiability across all three formats with a single key and a single verification command.

## What Changes

- Add a post-build signing step to the `build-linux` CI job that imports a GPG private key from a repository secret and produces `*.deb.asc`, `*.rpm.asc`, and `*.AppImage.asc` for each artifact
- Upload the three `.asc` files alongside the packages in `upload-artifact` and in the draft GitHub Release
- Commit the public key (`public.asc`) to the repository root
- Add `SECURITY.md` documenting the key fingerprint, per-format verification commands, a note on future repo-channel signing, and maintainer key-setup/rotation instructions

## Capabilities

### New Capabilities

- `linux-package-signing`: GPG signing of all three Linux release artifacts in CI — key management, post-build signing, signature distribution on GitHub releases, and user verification workflow

### Modified Capabilities

_(none)_

## Impact

- `.github/workflows/release.yml` — key import step and `gpg --detach-sign` signing added to `build-linux`; artifact upload paths updated to include `.asc` files
- Two new GitHub Actions repository secrets required: `GPG_PRIVATE_KEY` (armored private key) and `GPG_PASSPHRASE`
- New files committed to repo: `public.asc`, `SECURITY.md`
- No changes to Tauri config, Rust code, or frontend
- Note: APT repo, RPM repo, Flatpak, Snap, and AUR distribution each have their own repo-level signing mechanisms — those are separate from this change
