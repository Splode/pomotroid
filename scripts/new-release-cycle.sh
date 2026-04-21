#!/usr/bin/env bash
# new-release-cycle.sh — Re-open CHANGELOG.md for the next release.
#
# Usage:
#   ./scripts/new-release-cycle.sh
#
# What it does:
#   1. Verifies there is no existing [Unreleased] section (guards against
#      running this twice or before bump-version.sh has been run)
#   2. Inserts a fresh [Unreleased] header at the top of CHANGELOG.md
#   3. Commits the change on the current branch
#
# Run this after publishing the GitHub Release draft, while still on main.

set -euo pipefail

# Resolve repo root (script can be called from anywhere)
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

CHANGELOG="CHANGELOG.md"

# ── Guard: [Unreleased] must not already exist ────────────────────────────────

if grep -q '^## \[Unreleased\]' "$CHANGELOG"; then
  echo "Error: CHANGELOG.md already has an [Unreleased] section."
  echo "This script is intended to be run after bump-version.sh has renamed it."
  exit 1
fi

# ── Guard: working tree must be clean ────────────────────────────────────────

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Error: working tree has uncommitted changes. Commit or stash first."
  exit 1
fi

# ── Prepend [Unreleased] header ───────────────────────────────────────────────

# Write the new header to a temp file, then append the existing content.
TMP="$(mktemp)"
printf '## [Unreleased]\n\n' > "$TMP"
cat "$CHANGELOG" >> "$TMP"
mv "$TMP" "$CHANGELOG"

echo "  ✓ ${CHANGELOG} — [Unreleased] section added"

# ── Commit ────────────────────────────────────────────────────────────────────

git add "$CHANGELOG"
git commit -m "chore: open changelog for next release"

echo ""
echo "Done. Push when ready:"
echo "  git push origin main"
