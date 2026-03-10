# Auto-Update Feature — Test Flight Checklist

## Overview

This document covers end-to-end testing of the auto-update feature on the
`feat/autoupdate` branch, and the revert steps required before merging to
`main`.

---

## Temporary changes in effect during testing

The following three changes were made intentionally to scope the update
channel to this branch. **They must be reverted before merging.**

| # | File | Current (test) value | Revert to |
|---|------|----------------------|-----------|
| 1 | `src-tauri/tauri.conf.json` | `feat/autoupdate/latest.json` | `main/latest.json` |
| 2 | `.github/workflows/release.yml` — checkout `ref:` | `feat/autoupdate` | `main` |
| 3 | `.github/workflows/release.yml` — `git push` target | `HEAD:feat/autoupdate` | `HEAD:main` |

---

## Pre-conditions

- [ ] `feat/autoupdate` branch is pushed to GitHub
- [ ] A version tag has been pushed (e.g. `v1.1.1-test`) to trigger the
      release CI workflow
- [ ] CI run completes successfully — confirm that `latest.json` has been
      committed to the `feat/autoupdate` branch
- [ ] `latest.json` on the branch contains valid content: correct version,
      platform URLs pointing at the draft release assets, and non-empty
      Ed25519 signatures

---

## Step 1 — Build a downgraded test binary

The installed binary must report a version lower than the one in `latest.json`
for the updater to detect an available update.

- [ ] In `src-tauri/Cargo.toml`, temporarily set `version = "0.0.1"`
- [ ] In `package.json`, temporarily set `"version": "0.0.1"`
- [ ] Run `npm run tauri build` (or `npm run tauri dev` if testing the check
      dialog only without the actual install step)
- [ ] Install the resulting binary as you would a normal release

---

## Step 2 — Verify update detection

- [ ] Launch the installed `0.0.1` binary
- [ ] Open Settings → About
- [ ] Click **Check for Updates**
- [ ] Confirm the UI transitions: `idle` → `checking` → `available`
- [ ] Confirm the detected version matches the tag used in Pre-conditions
- [ ] Confirm release notes / version string display correctly

---

## Step 3 — Verify update installation

- [ ] Click **Install Update**
- [ ] Confirm the UI transitions to `installing` and shows download progress
- [ ] Confirm the app relaunches automatically after install completes
- [ ] After relaunch, open Settings → About
- [ ] Confirm the displayed version matches the updated release

---

## Step 4 — Verify error handling

- [ ] Temporarily break the endpoint (e.g. rename `latest.json` on the branch
      or point `tauri.conf.json` at a non-existent path), rebuild, and confirm
      the UI shows the `error` state with a readable message
- [ ] Restore the endpoint after verifying

---

## Step 5 — Verify opt-out setting

- [ ] Disable **Check for updates** in Settings → System
- [ ] Confirm that clicking Check for Updates in About does nothing / is hidden
- [ ] Re-enable the setting and confirm it works again

---

## Pre-merge revert checklist

Complete all three before opening the PR against `main`:

- [ ] **`src-tauri/tauri.conf.json`** — restore endpoint:
  ```
  "https://raw.githubusercontent.com/Splode/pomotroid/main/latest.json"
  ```
- [ ] **`.github/workflows/release.yml`** — restore checkout ref:
  ```yaml
  ref: main
  ```
- [ ] **`.github/workflows/release.yml`** — restore push target:
  ```
  git push origin HEAD:main
  ```
- [ ] **`src-tauri/Cargo.toml`** and **`package.json`** — confirm version is
      restored to the correct release version (not `0.0.1`)
- [ ] **`latest.json`** — confirm the file is **not** staged for the PR. It
      lives only on this branch for testing and should not be merged to `main`
      (CI will generate it correctly on the first stable release tag)
- [ ] Final `npm run check` and `cargo check` pass clean
