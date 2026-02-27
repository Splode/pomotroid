## 1. Setup — Paraglide and build config

- [x] 1.1 Install `@inlang/paraglide-js` npm package
- [x] 1.2 Run `npx @inlang/paraglide-js init` to scaffold `project.inlang/` and `messages/en.json`; set supported locales to `en, es, fr, de, ja` in `project.inlang/settings.json`
- [x] 1.3 Add `paraglide` Vite plugin to `vite.config.js` with `strategy: ['baseLocale']` and `outdir: './src/paraglide'`
- [x] 1.4 Add `paths: { relative: false }` to the adapter config in `svelte.config.js`

## 2. Backend — language setting

- [x] 2.1 Add `("language", "auto")` entry to `src-tauri/src/settings/defaults.rs`
- [x] 2.2 Add `pub language: String` to the `Settings` struct in `settings/mod.rs`; update `Default`, `load()`, and the `defaults_round_trip` test
- [x] 2.3 Add `MIGRATION_3` to `db/migrations.rs` that inserts `language = 'auto'` with `INSERT OR IGNORE`; bump schema version to 3; update `migration_is_idempotent` test

## 3. Frontend types

- [x] 3.1 Add `language: string` field to the `Settings` interface in `src/lib/types.ts`
- [x] 3.2 Add `language: 'auto'` to the default settings object in `src/lib/stores/settings.ts`

## 4. Notification refactor — Rust side

- [x] 4.1 Remove `round_notification_text()` from `notifications/mod.rs`; change `notify_round_change` signature to `notify(app, title, body, enabled)` that calls the platform dispatch directly; update the test
- [x] 4.2 Add `notification_show(title: String, body: String, app: AppHandle)` Tauri command to `commands.rs` that calls `notifications::dispatch` (extract `dispatch` as `pub` if needed)
- [x] 4.3 Register `notification_show` in `lib.rs` invoke_handler and import
- [x] 4.4 Update the `notify_round_change` call in `timer/mod.rs` to pass hardcoded English strings as a temporary measure (they will be replaced by the frontend in task 9.2); or remove the call entirely and rely solely on the frontend

## 5. Message catalog — English base

- [x] 5.1 Populate `messages/en.json` with all user-visible strings: round labels (Focus, Short Break, Long Break), settings nav labels (Timer, Appearance, Notifications, Shortcuts, System, About), settings group headings, toggle labels and descriptions, notification title/body strings for each round type, and any remaining UI strings

## 6. Message catalog — translations

- [x] 6.1 Create machine-translated `messages/es.json` (Spanish) with all keys from `en.json`
- [x] 6.2 Create machine-translated `messages/fr.json` (French) with all keys from `en.json`
- [x] 6.3 Create machine-translated `messages/de.json` (German) with all keys from `en.json`
- [x] 6.4 Create machine-translated `messages/ja.json` (Japanese) with all keys from `en.json`

## 7. Locale initialization

- [x] 7.1 Add `ipc/index.ts` wrapper: `notificationShow(title: string, body: string)`
- [x] 7.2 Add locale resolve helper `resolveLocale(language: string): string` to `src/lib/utils/locale.ts` — maps `'auto'` to `navigator.language` prefix-matched against supported locales, falls back to `'en'`
- [x] 7.3 Add locale init to `src/routes/+page.svelte` on mount: read `settings.language`, call `setLocale(resolveLocale(language))`
- [x] 7.4 Add `language` change handler in the `onSettingsChanged` callback in `+page.svelte`: call `setLocale()` when `language` changes
- [x] 7.5 Add locale init to `src/routes/settings/+page.svelte` on mount (same pattern)
- [x] 7.6 Add `language` change handler in `onSettingsChanged` in `settings/+page.svelte`

## 8. String extraction — settings sections

- [x] 8.1 Replace hardcoded strings in `TimerSection.svelte` with `m.*()` calls (group headings, slider labels, toggle labels/descriptions)
- [x] 8.2 Replace hardcoded strings in `AppearanceSection.svelte` with `m.*()` calls (mode labels, group headings)
- [x] 8.3 Replace hardcoded strings in `NotificationsSection.svelte` with `m.*()` calls (group headings, toggle labels/descriptions)
- [x] 8.4 Replace hardcoded strings in `SystemSection.svelte` with `m.*()` calls (group headings, toggle labels/descriptions)
- [x] 8.5 Replace hardcoded strings in `ShortcutsSection.svelte` with `m.*()` calls
- [x] 8.6 Replace hardcoded strings in `AboutSection.svelte` with `m.*()` calls

## 9. String extraction — core components and navigation

- [x] 9.1 Replace round labels in `Timer.svelte` `roundLabel()` function with `m.*()` calls (Focus, Short Break, Long Break)
- [x] 9.2 Replace `"Settings"` title in `SettingsTitlebar.svelte` with `m.settings_title()`
- [x] 9.3 Replace settings nav label strings in `settings/+page.svelte` with `m.*()` calls

## 10. Notification dispatch — frontend

- [x] 10.1 Remove notification dispatch from the Rust timer (if not done in 4.4): confirm `notify_round_change` is no longer called from `timer/mod.rs`
- [x] 10.2 Add notification dispatch to `+page.svelte` `onRoundChange` handler: if `$settings.notifications_enabled`, construct translated title/body using `m.*()` and call `notificationShow(title, body)`

## 11. Language dropdown in System settings

- [x] 11.1 Add a language selector control to `SystemSection.svelte` under a new "Language" group heading; options are Auto + the 5 supported locales with their native names; persist via `setSetting('language', value)` and call `setLocale()` on change

## 12. Validation

- [x] 12.1 `npm run check` — 0 type errors, 0 warnings
- [x] 12.2 `cargo test` — all tests pass
