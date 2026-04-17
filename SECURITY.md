# Security

## Release artifact signing

Linux release artifacts (`.deb`, `.rpm`, `.AppImage`) published to GitHub Releases are signed with GPG. Each package is accompanied by a detached ASCII-armored signature file (`.asc`) that you can use to verify authenticity before installing.

### Import the public key

```sh
gpg --import public.asc
```

Or by fingerprint from a key server:

```sh
gpg --keyserver keys.openpgp.org --recv-keys 2E6C4B1F9538A929690710C704A45C72F8B93EC2
```

**Key fingerprint:** `2E6C4B1F9538A929690710C704A45C72F8B93EC2`

### Verify a downloaded package

Download both the package and its `.asc` signature file from the release page, then run the appropriate command:

```sh
# Debian package
gpg --verify pomotroid_<version>_amd64.deb.asc pomotroid_<version>_amd64.deb

# RPM package
gpg --verify pomotroid-<version>-1.x86_64.rpm.asc pomotroid-<version>-1.x86_64.rpm

# AppImage
gpg --verify pomotroid_<version>_amd64.AppImage.asc pomotroid_<version>_amd64.AppImage
```

A `Good signature` message confirms the file is genuine and unmodified.

### Note on repository-based distribution

Signing for package repository channels (APT, RPM repo, Flatpak, Snap, AUR) is handled separately at the repository level when those distribution channels are set up. The GPG signatures on GitHub Release artifacts are for direct-download verification only.

---

## Maintainer: key setup

### Generate the signing keypair (one-time)

```sh
# Generate the keypair (choose RSA 4096 or Ed25519)
gpg --full-gen-key

# List the new key and note the fingerprint
gpg --list-secret-keys --keyid-format LONG

# Export the public key and commit it to the repository
gpg --armor --export <FINGERPRINT> > public.asc

# Export the private key for CI (keep this secure — do not commit)
gpg --armor --export-secret-keys <FINGERPRINT>
```

### Configure repository secrets

Add the following secrets to the GitHub repository (Settings → Secrets and variables → Actions):

| Secret name       | Value                                                               |
| ----------------- | ------------------------------------------------------------------- |
| `GPG_PRIVATE_KEY` | The full output of `gpg --armor --export-secret-keys <FINGERPRINT>` |
| `GPG_PASSPHRASE`  | The passphrase chosen during key generation                         |

### Key rotation

1. Generate a new keypair following the steps above
2. Update `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` in repository secrets
3. Replace `public.asc` with the new public key and commit
4. Update the fingerprint in this file
5. Optionally revoke the old key and upload the revocation certificate to key servers
