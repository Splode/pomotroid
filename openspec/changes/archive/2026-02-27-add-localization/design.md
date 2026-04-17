## Context

Pomotroid has all user-visible strings hardcoded in English across 8+ Svelte components and in the Rust notification module. There is no existing i18n infrastructure. The app runs as a Tauri SPA (single-page application with `adapter-static`) in two windows — `main` and `settings` — which share the same JS bundle. The settings system already supports per-key persistence and live propagation via `settings:changed` events, which we can leverage for the language preference.

## Goals / Non-Goals

**Goals:**

- Translate all user-visible strings in the frontend via Paraglide message functions
- Detect system locale automatically; allow user override via a language dropdown in System settings
- Ship 5 locales at launch: `en` (base), `es`, `fr`, `de`, `ja`
- Refactor notification strings to be constructed on the frontend (translated), then passed to Rust as plain title + body

**Non-Goals:**

- RTL language support (layout changes required — future work)
- Pluralization beyond what Paraglide's ICU message format handles naturally
- Community translation tooling / Inlang editor integration (the message format is compatible, but no workflow is set up now)
- Translating theme names or custom user data

## Decisions

### D1 — Paraglide JS v2 over svelte-i18n or custom solution

**Decision**: Use `@inlang/paraglide-js`.

**Rationale**: Paraglide generates type-safe message functions at build time. Each message is a named function (e.g., `m.settings_title()`) — dead-code elimination removes unused messages, and TypeScript catches missing/renamed keys. `svelte-i18n` uses a runtime lookup store (`$_('key')`) which is less type-safe and can't tree-shake. A custom solution adds maintenance burden.

**Alternatives considered**: `svelte-i18n` (runtime, less type-safe), `i18next` (heavyweight, browser-focused), custom JSON loader (no type safety).

### D2 — Strategy: `['baseLocale']` (no URL-based routing)

**Decision**: Configure Paraglide with `strategy: ['baseLocale']` and call `setLocale()` imperatively at startup.

**Rationale**: Paraglide's default strategies route locale via URL path (e.g., `/en/`, `/fr/`). Tauri SPA apps use a flat file structure; URL-based routing would break navigation and the `adapter-static` fallback. With `['baseLocale']`, Paraglide only changes locale via `setLocale()` — matching exactly how settings-driven locale switching works.

**Alternatives considered**: URL-based routing (incompatible with Tauri SPA), storing locale in `localStorage` (bypasses the settings system).

### D3 — Language setting stored in SQLite (`language` key)

**Decision**: Add `language` to the settings DB with default `'auto'`. The `Settings` struct gains a `language: String` field; a MIGRATION_3 inserts the default row.

**Rationale**: Consistent with how all other user preferences are stored. `settings:changed` propagates the new value to both windows automatically — no extra IPC needed for the settings window to pick up a language change made in the main window (or vice versa).

**Locale codes**: The stored value is either `'auto'` or an IETF BCP 47 tag (`'es'`, `'fr'`, `'de'`, `'ja'`). `'auto'` at runtime reads `navigator.language`, matches to the closest supported locale (prefix match: `'fr-CA'` → `'fr'`), falling back to `'en'`.

### D4 — Locale init in both page files

**Decision**: Both `+page.svelte` and `settings/+page.svelte` call `setLocale()` on mount. They also call it again inside `onSettingsChanged` when `language` changes.

**Rationale**: Each Tauri window is an independent browser context. A locale change in one window won't automatically affect the other unless each window listens to `settings:changed` and re-calls `setLocale()`. This is consistent with how theme changes are handled across windows.

### D5 — Notification strings constructed on the frontend (Option B)

**Decision**: Remove hardcoded English strings from `notifications/mod.rs`. Add a `notification_show(title: String, body: String)` Tauri command. In `+page.svelte`'s `onRoundChange` handler, construct translated title/body strings with Paraglide and call `notification_show`.

**Rationale**: Rust has no concept of the user's chosen locale — the settings value is stored as a string and Rust has no message catalog. Constructing notification strings on the frontend means translations are consistent with the rest of the UI. The lean `notification_show` command is simpler and more testable than passing locale codes to Rust.

**Alternatives considered**: Passing a locale code to Rust and building a Rust-side message catalog (high complexity, duplicated strings), keeping English-only notifications (inconsistent UX).

### D6 — `paths: { relative: false }` in svelte.config.js

**Decision**: Add `paths: { relative: false }` to the adapter-static config.

**Rationale**: Paraglide's Vite plugin emits imports using absolute base-path URLs. `adapter-static` by default rewrites asset URLs to relative paths, which corrupts the Paraglide runtime imports. `relative: false` preserves absolute paths and makes the build work correctly.

### D7 — Machine-translated launch locales

**Decision**: Ship `es`, `fr`, `de`, `ja` as machine-translated starter files. All strings are short, contextual, and easy to verify.

**Rationale**: Getting 4 locales out at launch demonstrates the system works and seeds community corrections. The short strings (labels, button text, section headings) are low-risk for machine translation — errors are obvious and easy to fix.

## Risks / Trade-offs

- **[Risk] Paraglide Vite plugin conflicts with SvelteKit** → Mitigation: add `paraglideVitePlugin()` to `vite.config.ts` plugins array (not `svelte.config.js`); this is the documented path for SvelteKit integration.
- **[Risk] Both windows calling `setLocale()` on mount may cause a brief flash** → Mitigation: locale loading is synchronous; there is no async gap between mount and the `setLocale()` call, so no flash in practice.
- **[Risk] Machine translations contain errors** → Mitigation: strings are short and visible; wrong translations are easy for native speakers to spot and report. All message keys are typed, so corrections are a simple JSON edit.
- **[Trade-off] Paraglide generates a messages module** → The generated `src/paraglide/` directory must be committed or gitignored. Decision: commit it (small, stable after initial generation; makes builds reproducible without re-running codegen).

## Migration Plan

1. Install `@inlang/paraglide-js`; run init to scaffold `project.inlang/` and `messages/en.json`
2. Add MIGRATION_3 to insert `language = 'auto'` default row
3. Update Settings struct and frontend types
4. Configure Vite plugin and svelte.config.js fix
5. Extract all strings to `messages/en.json`; generate machine translations for `es`, `fr`, `de`, `ja`
6. Replace hardcoded strings in all components with `m.*()` calls
7. Add locale init logic to both page files
8. Refactor notification dispatch (new command + frontend handler)
9. Add language dropdown to SystemSection.svelte

**Rollback**: Language feature is additive — removing the `language` setting row and reverting message function calls restores the English-only state. No data loss.

## Open Questions

- None — all major decisions are resolved from the explore session.
