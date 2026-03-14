## 1. Rust — Settings

- [x] 1.1 In `src-tauri/src/settings/mod.rs`, add `pub global_shortcuts_enabled: bool` field to the `Settings` struct (after `check_for_updates`)
- [x] 1.2 In `src-tauri/src/settings/defaults.rs` (or wherever defaults are seeded), add `("global_shortcuts_enabled", "false")` to the common defaults list
- [x] 1.3 In `src-tauri/src/settings/mod.rs`, handle `"global_shortcuts_enabled"` in the DB→struct mapping (parse `"true"`→`true`, anything else→`false`)

## 2. Rust — Database Migration

- [x] 2.1 In `src-tauri/src/db/migrations.rs`, add `MIGRATION_4` constant that inserts `('global_shortcuts_enabled', 'false')` with `INSERT OR IGNORE` and increments schema version to 4
- [x] 2.2 In the `run()` function in `migrations.rs`, add `if version < 4 { ... }` block that applies `MIGRATION_4` with logging consistent with existing migration blocks

## 3. Rust — Shortcuts Module

- [x] 3.1 In `src-tauri/src/shortcuts/mod.rs`, update `register_all` to accept `&Settings` (already does) and early-return after calling `unregister_all` when `settings.global_shortcuts_enabled` is `false`, logging `[shortcuts] global shortcuts disabled — skipping registration`

## 4. Rust — Commands

- [x] 4.1 In `src-tauri/src/commands.rs`, extend the `settings_set` shortcut-change trigger list to include `"global_shortcuts_enabled"` alongside `"shortcut_toggle" | "shortcut_reset" | "shortcut_skip" | "shortcut_restart"`, so toggling the setting immediately registers or unregisters shortcuts

## 5. Frontend — Types

- [x] 5.1 In `src/lib/types.ts`, add `global_shortcuts_enabled: boolean` to the `Settings` interface (after `check_for_updates`)

## 6. Frontend — ShortcutsSection UI

- [x] 6.1 In `src/lib/components/settings/sections/ShortcutsSection.svelte`, import `SettingsToggle` from `$lib/components/settings/SettingsToggle.svelte`
- [x] 6.2 Add a `toggle` helper (same pattern as `SystemSection`) that calls `setSetting(dbKey, current ? 'false' : 'true')` and updates the settings store
- [x] 6.3 Add a `SettingsToggle` for `global_shortcuts_enabled` at the top of the section template, above the accessibility notice and note, using i18n keys `shortcuts_toggle_enabled` and `shortcuts_toggle_enabled_desc`
- [x] 6.4 Wrap the four `ShortcutInput` rows (and the note paragraph) in a container that applies `opacity: 0.4` and `pointer-events: none` when `!$settings.global_shortcuts_enabled`, to visually indicate they are inactive

## 7. Localisation

- [x] 7.1 Add `"shortcuts_toggle_enabled"` and `"shortcuts_toggle_enabled_desc"` to all 8 locale files in `src/messages/` with appropriate translations:
  - `en`: `"Enable Global Shortcuts"` / `"Register system-wide keyboard shortcuts for controlling the timer."`
  - `de`: `"Globale Tastenkürzel aktivieren"` / `"Systemweite Tastenkürzel zur Steuerung des Timers registrieren."`
  - `es`: `"Activar atajos globales"` / `"Registrar atajos de teclado en todo el sistema para controlar el temporizador."`
  - `fr`: `"Activer les raccourcis globaux"` / `"Enregistrer des raccourcis clavier système pour contrôler le minuteur."`
  - `ja`: `"グローバルショートカットを有効にする"` / `"タイマーを操作するためのシステム全体のキーボードショートカットを登録します。"`
  - `zh`: `"启用全局快捷键"` / `"注册系统级键盘快捷键以控制计时器。"`
  - `pt`: `"Ativar Atalhos Globais"` / `"Registrar atalhos de teclado em todo o sistema para controlar o temporizador."`
  - `tr`: `"Global Kısayolları Etkinleştir"` / `"Zamanlayıcıyı kontrol etmek için sistem genelinde klavye kısayollarını kaydet."`

## 8. Verify

- [x] 8.1 On first launch (fresh DB), confirm no global shortcuts are registered and the toggle is off
- [x] 8.2 Toggle on — confirm all four shortcuts become active immediately
- [x] 8.3 Toggle off — confirm shortcuts stop responding immediately
- [x] 8.4 Shortcut key fields are non-interactive (pointer-events blocked) and visually dimmed while disabled — spec updated to match this behaviour
- [x] 8.5 Restart the app with shortcuts disabled — confirm they remain disabled
- [x] 8.8 Enable global shortcuts, then Reset All Settings (Settings → System → Data) — confirm the toggle returns to off and shortcuts are unregistered
- [x] 8.6 Run `npm run check` — no type errors
- [x] 8.7 Run `cargo test` in `src-tauri/` — all tests pass
