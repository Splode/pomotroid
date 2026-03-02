#!/usr/bin/env bash
# bump-version.sh — Bump version, update CHANGELOG, commit, and tag.
#
# Usage:
#   ./scripts/bump-version.sh <version>
#
# Example:
#   ./scripts/bump-version.sh 1.1.0
#
# What it does:
#   1. Validates the version argument (semver X.Y.Z)
#   2. Updates version in tauri.conf.json, Cargo.toml, and package.json
#   3. Renames [Unreleased] in CHANGELOG.md to [vX.Y.Z] - YYYY-MM-DD
#   4. Commits all changes and creates an annotated git tag
#
# After running, push with:
#   git push origin main --follow-tags

set -euo pipefail

# ── Validate input ────────────────────────────────────────────────────────────

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <version>"
  echo "  e.g. $0 1.1.0"
  exit 1
fi

VERSION="$1"

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: version must be in X.Y.Z format (got: $VERSION)"
  exit 1
fi

# Resolve repo root (script can be called from anywhere)
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# ── Check working tree ────────────────────────────────────────────────────────

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Error: working tree has uncommitted changes. Commit or stash first."
  exit 1
fi

# ── Check [Unreleased] exists ─────────────────────────────────────────────────

if ! grep -q '^\[Unreleased\]' CHANGELOG.md; then
  echo "Error: CHANGELOG.md has no [Unreleased] section."
  echo "Add one with the changes for this release before bumping."
  exit 1
fi

DATE="$(date +%Y-%m-%d)"

echo "Bumping to v${VERSION} (${DATE})..."

# ── Update tauri.conf.json ────────────────────────────────────────────────────

TAURI_CONF="src-tauri/tauri.conf.json"
sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"${VERSION}\"/" "$TAURI_CONF"
echo "  ✓ ${TAURI_CONF}"

# ── Update Cargo.toml (first occurrence — the package version) ────────────────

CARGO_TOML="src-tauri/Cargo.toml"
# Only update the first `version = "..."` line (the [package] entry)
sed -i "0,/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/s//version = \"${VERSION}\"/" "$CARGO_TOML"
echo "  ✓ ${CARGO_TOML}"

# ── Update package.json ───────────────────────────────────────────────────────

PACKAGE_JSON="package.json"
sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"${VERSION}\"/" "$PACKAGE_JSON"
echo "  ✓ ${PACKAGE_JSON}"

# ── Update Cargo.lock (regenerate after Cargo.toml change) ───────────────────

cargo generate-lockfile --manifest-path src-tauri/Cargo.toml --quiet 2>/dev/null || true
echo "  ✓ src-tauri/Cargo.lock (refreshed)"

# ── Update CHANGELOG.md ───────────────────────────────────────────────────────

CHANGELOG="CHANGELOG.md"
# Replace the [Unreleased] header with [vX.Y.Z] - YYYY-MM-DD
sed -i "s/^\[Unreleased\]/[v${VERSION}] - ${DATE}/" "$CHANGELOG"
echo "  ✓ ${CHANGELOG} ([Unreleased] → [v${VERSION}] - ${DATE})"

# ── Git commit + tag ──────────────────────────────────────────────────────────

git add \
  "$TAURI_CONF" \
  "$CARGO_TOML" \
  "src-tauri/Cargo.lock" \
  "$PACKAGE_JSON" \
  "$CHANGELOG"

git commit -m "chore: release v${VERSION}"

git tag -a "v${VERSION}" -m "v${VERSION}"

echo ""
echo "Done! Committed and tagged v${VERSION}."
echo ""
echo "Push to trigger the release workflow:"
echo "  git push origin main --follow-tags"
