## Context

The existing `build-linux` CI job in `.github/workflows/release.yml` produces `.deb`, `.rpm`, and `.AppImage` bundles via `npm run tauri build -- --bundles deb,appimage,rpm` and uploads them to a draft GitHub Release. No signing step exists.

GPG is available on GitHub Actions' Ubuntu runners without any `apt-get install`. All three artifact types can be signed with the same `gpg --detach-sign --armor` command, producing a `.asc` sidecar file that users verify with `gpg --verify`.

## Goals / Non-Goals

**Goals:**
- Sign all three Linux artifacts with a single GPG key using detached ASCII-armored signatures
- Keep the private key out of the repository — stored only in GitHub Actions secrets
- Make `.asc` files available alongside packages on the GitHub Release page
- Document verification and key management for users and maintainers

**Non-Goals:**
- Tauri-native AppImage signing via `SIGN=1` / `appimagetool` — this is AppImage-only and doesn't compose with `.deb`/`.rpm` signing
- Format-native embedded signing (`dpkg-sig`, `rpmsign`) — higher CI complexity, different verification UX per format
- Repo-level signing for APT, RPM, Flatpak, Snap, or AUR distribution — those channels have their own signing mechanisms, handled separately when each channel is set up
- Automatic signature verification in the CI pipeline

## Decisions

### Unified detached `.asc` sidecars for all three formats

`gpg --detach-sign --armor <file>` produces a `<file>.asc` sidecar that works identically for `.deb`, `.rpm`, and `.AppImage`. One tool, one GPG key, one verification command across all formats:

```
gpg --verify pomotroid.deb.asc pomotroid.deb
gpg --verify pomotroid.rpm.asc pomotroid.rpm
gpg --verify pomotroid.AppImage.asc pomotroid.AppImage
```

**Alternative considered:** Tauri-native AppImage signing (`SIGN=1`) + format-native for the others (`dpkg-sig`, `rpmsign`). Rejected — three different tools, three different verification UXes, and higher CI complexity with no meaningful benefit for GitHub release users.

**Alternative considered:** Tauri-native AppImage signing only. Rejected — leaves `.deb` and `.rpm` unsigned with no clear path to adding them later without introducing a second mechanism.

### Signing as a post-build step, same job

The signing step runs after `npm run tauri build` and before `upload-artifact`, within the existing `build-linux` job. This avoids a separate job, keeps the key import co-located with its use, and means the key is discarded when the ephemeral runner terminates.

### GPG key stored as two repository secrets

- `GPG_PRIVATE_KEY`: ASCII-armored private key (`gpg --armor --export-secret-keys <fingerprint>`)
- `GPG_PASSPHRASE`: key passphrase, passed via `echo "$GPG_PASSPHRASE" | gpg --batch --yes --passphrase-fd 0 --detach-sign --armor <file>`

### Public key committed as `public.asc`

A committed file is always reachable from the repo regardless of key server availability. The full fingerprint is recorded in `SECURITY.md`.

### Repo-channel signing is out of scope

When APT, RPM, Flatpak, Snap, or AUR distribution is set up, each channel requires its own signing approach (APT Release file signing, `rpmsign` for RPM repos, OSTree for Flatpak, Snap Store infrastructure, AUR PKGBUILD). This is independent of individual file signing for GitHub releases and is deferred to those distribution efforts.

## Risks / Trade-offs

- **Sidecar files can be separated from packages:** Users must know to download both the package and its `.asc`. Mitigation: document clearly in `SECURITY.md` and the release description.
- **Key compromise:** Rotation requires new keypair, updated secrets, and re-published `public.asc`. Mitigation: restrict secret access to protected tags; document rotation in `SECURITY.md`.
- **Passphrase in CI:** Passed via stdin with `--passphrase-fd 0`, never echoed to logs. Masked by GitHub Actions automatically.

## Migration Plan

1. Generate GPG keypair locally (see `SECURITY.md`)
2. Add `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` as repository secrets
3. Commit `public.asc` and `SECURITY.md`
4. Update `.github/workflows/release.yml` with key import and signing steps
5. Push a test tag; verify signing step completes and `.asc` files appear in release assets
6. Download an artifact and its `.asc`; verify locally with `gpg --verify`
