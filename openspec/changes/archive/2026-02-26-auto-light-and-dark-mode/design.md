## Context

Pomotroid uses a flat key/value SQLite settings table. The active theme is stored as a single `theme` string (theme name). The frontend loads this on startup, looks up the theme JSON by name, and applies its CSS custom properties to `:root`. Eighteen bundled themes exist; some are light (GitHub, Solarized Light, Rosé Pine Dawn), most are dark.

There is currently no OS color-scheme detection in the codebase. The `applyTheme()` store function applies a theme object's colors directly to the document root — it is the single apply point shared by both the main window and the settings window.

## Goals / Non-Goals

**Goals:**

- Replace the single `theme` setting with `theme_mode` + `theme_light` + `theme_dark`
- Resolve the active theme at startup and on live OS change using a shared utility
- Migrate existing users' `theme` value into both new picker fields automatically
- Update the Appearance section UI to expose mode selector + two independent pickers
- Remove the `theme` field from settings entirely (no redundant derived state)

**Non-Goals:**

- Filtering or categorizing themes as "light" or "dark" in the UI or JSON
- Adding new Pomotroid Light/Dark theme variants (separate future work)
- Per-round-type theme overrides
- Syncing the resolved theme name back to the DB on every OS change

## Decisions

### 1. Frontend-only OS signal detection

**Decision**: Use `window.matchMedia('(prefers-color-scheme: dark)')` in the frontend. No Rust involvement.

**Rationale**: `applyTheme()` is purely frontend. The Rust side never needs to know the resolved theme — it only stores user preferences. Adding a Tauri theme event would require new commands and cross-window event plumbing for no gain.

**Alternative considered**: Tauri `on_system_theme_changed` window event → emit custom IPC event → frontend listener. Adds ~3 extra layers for the same result.

### 2. Remove `theme` field entirely (Option B)

**Decision**: Delete `theme` from `DEFAULTS`, the `Settings` struct, and `types.ts`. Startup derives the active theme from `theme_mode` + `theme_light`/`theme_dark` + OS query every time.

**Rationale**: Keeping a cached `theme` field creates a second source of truth. It would need to be updated on every OS change and every picker change, and could drift. Deriving it fresh on startup is cheap (one `matchMedia.matches` call) and eliminates the drift problem.

**Alternative considered**: Keep `theme` as a write-through cache. Rejected because it needs to be kept in sync across two windows and OS changes, which adds complexity without benefit.

### 3. Shared `resolveThemeName()` utility

**Decision**: Extract a single `resolveThemeName(settings, osDark): string` function, imported by both `+page.svelte` and `settings/+page.svelte`.

**Rationale**: Both windows need identical resolution logic. A shared utility ensures consistency and avoids drift between the two implementations.

### 4. Deferred preview for inactive picker

**Decision**: Clicking a theme in the non-active picker saves the selection (via `setSetting`) but does not call `applyTheme()`.

**Rationale**: Applying a theme that contradicts the current OS/mode would be confusing. The user is configuring a future state. The active theme only changes when the picker is currently the active one (mode=Light selects in light picker, mode=Dark selects in dark picker, mode=Auto uses the picker that matches current OS).

**Alternative considered**: Always apply as preview regardless of active state. Rejected — too surprising when auto-mode is active and OS is dark but user is configuring light picker.

### 5. DB migration for existing users

**Decision**: Add a one-time migration in `db/migrations.rs` that reads the current `theme` value and writes it to `theme_light` and `theme_dark` if those keys are absent.

**Rationale**: The seed/defaults mechanism only inserts "Pomotroid" for missing keys. Existing users with custom themes (e.g. Nord) would silently revert to Pomotroid without a migration. The migration runs once during startup, before the settings are loaded.

## Risks / Trade-offs

- **Startup flash on slow OS query**: `matchMedia` is synchronous, so there is no async gap. No risk.
- **Settings window out of sync**: If the user changes the OS theme while the settings window is open and focused on Appearance, the active picker highlight needs to update. The `matchMedia` listener in the settings page must handle this.
- **`theme` removal is breaking**: Any external tooling reading the settings DB directly will lose the `theme` key. Acceptable — this is an internal implementation detail.

## Migration Plan

1. New migration added to `db/migrations.rs` runs on startup
2. If `theme_light` key is absent: read `theme`, write to `theme_light` and `theme_dark`, set `theme_mode = "auto"`
3. Delete `theme` key from settings table (or leave as orphan — no functional impact either way)
4. Seed inserts `theme_mode/light/dark` defaults for brand-new installs
5. No rollback path needed — migration is additive and non-destructive to user data

## Open Questions

- None. All decisions resolved during exploration.
