## 1. Key generation (one-time, local)

- [ ] 1.1 Generate a dedicated GPG signing keypair: `gpg --full-gen-key` (RSA 4096 or Ed25519, with passphrase)
- [ ] 1.2 Note the key fingerprint: `gpg --list-secret-keys --keyid-format LONG`
- [ ] 1.3 Export ASCII-armored public key: `gpg --armor --export <fingerprint> > public.asc`
- [ ] 1.4 Export ASCII-armored private key: `gpg --armor --export-secret-keys <fingerprint>`
- [ ] 1.5 Add `GPG_PRIVATE_KEY` repository secret (Settings → Secrets → Actions → New repository secret)
- [ ] 1.6 Add `GPG_PASSPHRASE` repository secret

## 2. Commit public key and documentation

- [ ] 2.1 Commit `public.asc` to the repository root
- [x] 2.2 Create `SECURITY.md` with:
  - Full GPG key fingerprint
  - How to import the public key: `gpg --import public.asc`
  - Verification commands for each format:
    - `gpg --verify pomotroid_*.deb.asc pomotroid_*.deb`
    - `gpg --verify pomotroid_*.rpm.asc pomotroid_*.rpm`
    - `gpg --verify pomotroid_*.AppImage.asc pomotroid_*.AppImage`
  - Note that APT/RPM repo, Flatpak, Snap, and AUR distribution use separate repo-level signing mechanisms
  - Maintainer key generation and rotation instructions

## 3. Update release workflow

- [x] 3.1 Add "Import GPG signing key" step to `build-linux` in `.github/workflows/release.yml`, before the build step
- [x] 3.2 Add "Sign Linux artifacts" step after the build step, signing all three artifact files
- [x] 3.3 Update the `upload-artifact` paths to include `*.asc` files alongside packages
- [x] 3.4 Update the `Create draft release` file globs to include `*.deb.asc`, `*.rpm.asc`, and `*.AppImage.asc`

## 4. Verification

- [ ] 4.1 Push a test tag and confirm the signing step completes without error
- [ ] 4.2 Download a signed artifact and its `.asc` from the test run; verify locally:
  - `gpg --import public.asc`
  - `gpg --verify pomotroid_*.deb.asc pomotroid_*.deb`
- [ ] 4.3 Confirm no private key material appears in CI logs
