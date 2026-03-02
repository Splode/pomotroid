# Contributing to Pomotroid

Thank you for taking the time to contribute. Below you'll find everything needed to get set up, make changes, and cut a release.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Filing a Bug or Feature](#filing-a-bug-or-feature)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Releasing](#releasing)

---

## Code of Conduct

Be kind and respectful to the members of the community. Take time to educate others who are seeking help. Harassment of any kind will not be tolerated.

---

## Filing a Bug or Feature

1. Check existing issues before opening a new one.
2. **Bug reports** — include steps to reproduce, Pomotroid version, OS, and what you expected vs. what happened.
3. **Feature requests** — open an issue with a clear title and description of the feature and why it would be useful.

---

## Development Setup

### Prerequisites

| Tool | Version |
|------|---------|
| Node.js | 22+ |
| Rust | stable (via [rustup](https://rustup.rs)) |
| npm | bundled with Node.js |

**Linux** — install system dependencies:

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  patchelf libasound2-dev
```

**macOS / Windows** — no extra system dependencies required.

### Install and run

```bash
git clone https://github.com/Splode/pomotroid
cd pomotroid
npm install
npm run tauri dev
```

### Checks and tests

```bash
# TypeScript + Svelte type checking
npm run check

# Rust unit tests
cargo test --manifest-path src-tauri/Cargo.toml

# Rust linting (must pass with zero warnings)
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

All three must pass before submitting a pull request. The CI pipeline runs them automatically.

### Seeding test data

To populate the statistics views with realistic historical data, use the seed script (requires Python 3, no extra dependencies):

```bash
# Preview what would be inserted — no database changes
python3 scripts/seed-db.py --dry-run

# Seed ~2 years of session history (default)
python3 scripts/seed-db.py

# Seed 1 year and wipe any existing sessions first
python3 scripts/seed-db.py --days 365 --clear
```

The database must exist before seeding — launch Pomotroid at least once to create it. The script resolves the platform-specific app-data path automatically; pass `--db PATH` to override.

---

## Project Structure

```
pomotroid/
├── src/                    # SvelteKit frontend (Svelte 5 runes)
│   ├── routes/             # Page components (main timer, settings, stats)
│   └── lib/
│       ├── components/     # Reusable UI components
│       ├── ipc/            # Frontend wrappers for Tauri commands
│       └── stores/         # Svelte stores (settings, theme)
├── src-tauri/              # Rust backend (Tauri 2)
│   └── src/
│       ├── commands.rs     # IPC command handlers
│       ├── timer/          # Timer engine
│       ├── audio/          # Audio playback (rodio)
│       ├── db/             # SQLite persistence (rusqlite)
│       └── settings/       # Settings load/save
├── static/themes/          # Built-in JSON theme files
└── scripts/                # Maintainer scripts
```

---

## Making Changes

1. Fork the repository and create a feature branch from `main`.
2. Make your changes. Keep commits focused and use conventional commit prefixes (`feat:`, `fix:`, `chore:`, etc.).
3. Ensure all checks pass (`npm run check`, `cargo test`, `cargo clippy`).
4. Open a pull request against `main` with a clear description of what changed and why.

### Adding a built-in theme

Built-in themes are embedded into the binary at compile time, so adding one requires a small code change in addition to the JSON file.

1. **Create the theme JSON** in `static/themes/your-theme-name.json`. See `THEMES.md` for the required format and color keys.

2. **Register it** in `src-tauri/src/themes/mod.rs` — add an `include_str!()` entry to the `BUNDLED_JSON` array:
   ```rust
   include_str!("../../../static/themes/your-theme-name.json"),
   ```

3. **Update the bundled theme count** in the test assertion in the same file (e.g. `assert_eq!(themes.len(), 38, ...)`).

Custom user themes (placed in the app data `themes/` folder) are discovered at runtime and require no code changes.

### Adding a localization

The app uses [Inlang + Paraglide](https://inlang.com) for translations. Adding a language requires two files to be touched, then a rebuild.

1. **Create the message file** at `src/messages/{locale}.json`. Copy `src/messages/en.json` as a starting point and translate all values — do not change any keys.

2. **Register the locale** by adding it to the `locales` array in `project.inlang/settings.json`:
   ```json
   "locales": ["en", "zh", "pt", "your-locale"]
   ```

3. **Rebuild** (`npm run build`) — Paraglide auto-generates the typed JS modules in `src/paraglide/messages/`. No further code changes are needed; the language will appear in Settings automatically.

### Modifying settings

Settings are stored as key/value strings in SQLite. Adding a new setting involves:

1. Adding the DB key and default value in `src-tauri/src/settings/defaults.rs`
2. Adding the typed field to the `Settings` struct in `src-tauri/src/settings/mod.rs`
3. Updating the frontend `Settings` type in `src/lib/types.ts`
4. Exposing a toggle or input in the relevant settings section under `src/lib/components/settings/sections/`

---

## Releasing

Releases are managed by maintainers. The process is intentionally minimal: one script, one push.

### Prerequisites

- You are on the `main` branch with a clean working tree.
- `[Unreleased]` in `CHANGELOG.md` contains the changes for this release.

### Steps

**1. Fill in the changelog**

Open `CHANGELOG.md` and make sure `[Unreleased]` accurately describes everything going into the release. The release workflow uses this section as the GitHub Release body.

**2. Run the bump script**

```bash
./scripts/bump-version.sh <version>
# e.g.
./scripts/bump-version.sh 1.1.0
```

This will:
- Update the version in `tauri.conf.json`, `Cargo.toml`, and `package.json`
- Rename `[Unreleased]` in `CHANGELOG.md` to `[v1.1.0] - YYYY-MM-DD`
- Commit all changes and create an annotated `v1.1.0` tag

**3. Push**

```bash
git push origin main --follow-tags
```

Pushing the tag triggers the [release workflow](.github/workflows/release.yml), which:
- Builds Linux (`.deb`, `.AppImage`), macOS (universal `.dmg`), and Windows (`.exe` installer) in parallel
- Creates a **draft** GitHub Release with all artifacts attached and the changelog section as the release body

**4. Publish the draft**

Go to the [Releases page](https://github.com/Splode/pomotroid/releases), review the draft, and click **Publish release**.

**5. Add a new `[Unreleased]` section**

After publishing, add a fresh `[Unreleased]` block at the top of `CHANGELOG.md` to start collecting the next release's changes.

### Version numbering

Pomotroid follows [Semantic Versioning](https://semver.org):

| Change | Version bump |
|--------|-------------|
| Breaking changes or major rewrites | `X.0.0` |
| New features, backward-compatible | `X.Y.0` |
| Bug fixes only | `X.Y.Z` |
