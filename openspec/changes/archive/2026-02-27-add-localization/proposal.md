## Why

Pomotroid currently has all user-visible strings hardcoded in English, limiting its reach to non-English-speaking users. Adding i18n support broadens the audience and makes the app accessible to users whose system locale is not English, with a clear path for community contributions of additional translations.

## What Changes

- Install and configure **Paraglide JS v2** (`@inlang/paraglide-js`) as the i18n library — compile-time, type-safe, tree-shakable message functions
- Extract all ~50 user-visible strings from the frontend into Paraglide message files (`messages/en.json` as base, plus machine-translated `es`, `fr`, `de`, `ja`)
- Add a `language` setting (`'auto'` default) stored in SQLite; a dropdown in the **System** settings section lets users override the detected locale
- Startup logic reads the `language` setting and calls `setLocale()` accordingly; `'auto'` maps `navigator.language` to the closest supported locale
- Refactor notification dispatch: frontend constructs translated notification strings (using Paraglide) and calls a new lean `notification_show(title, body)` Rust command — removes hardcoded English strings from `notifications/mod.rs`
- Fix `svelte.config.js` adapter-static compatibility: add `paths: { relative: false }`
- Configure Paraglide strategy as `['baseLocale']` (no URL routing — correct for Tauri SPA)

## Capabilities

### New Capabilities

- `localization`: Paraglide-based i18n system — message files, locale detection, language setting, language override UI

### Modified Capabilities

- `settings`: New `language` DB key and `language` field on `Settings` struct; language selector in System section

## Impact

- **Frontend**: `vite.config.ts` (Paraglide Vite plugin), `svelte.config.js` (paths fix), all 6 settings section components, `Timer.svelte`, `TimerFooter.svelte`, `+page.svelte`, `settings/+page.svelte` (locale init + notification dispatch)
- **Backend**: `settings/defaults.rs` (new key), `settings/mod.rs` (new field + load), `db/migrations.rs` (MIGRATION_3), `commands.rs` (new `notification_show` command), `notifications/mod.rs` (simplified to take title+body)
- **New files**: `messages/en.json`, `messages/es.json`, `messages/fr.json`, `messages/de.json`, `messages/ja.json`
- **Dependencies**: `@inlang/paraglide-js` (npm)
