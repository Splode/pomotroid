## 1. Rust — Settings & DB

- [x] 1.1 Add `theme_mode`, `theme_light`, `theme_dark` defaults to `settings/defaults.rs`; remove `theme` default
- [x] 1.2 Replace `theme: String` with `theme_mode: String`, `theme_light: String`, `theme_dark: String` in `settings/mod.rs`
- [x] 1.3 Add DB migration in `db/migrations.rs`: if `theme_light` absent, copy `theme` → `theme_light` + `theme_dark`, set `theme_mode = "auto"`, then delete `theme` key
- [x] 1.4 Run `cargo test` — fix any breakage from removing the `theme` field

## 2. Frontend — Types & Store

- [x] 2.1 Remove `theme: string` and add `theme_mode: string`, `theme_light: string`, `theme_dark: string` to `Settings` interface in `types.ts`
- [x] 2.2 Update default object in `stores/settings.ts` to match new fields

## 3. Frontend — Shared Resolution Utility

- [x] 3.1 Create `src/lib/utils/theme.ts` exporting `resolveThemeName(settings: Settings, osDark: boolean): string`

## 4. Frontend — Window Integration

- [x] 4.1 Update `src/routes/+page.svelte` startup theme load to use `resolveThemeName()` instead of `s.theme`
- [x] 4.2 Add `matchMedia('(prefers-color-scheme: dark)')` change listener in `+page.svelte`; on change, if `theme_mode === 'auto'` resolve and apply theme
- [x] 4.3 Update `src/routes/settings/+page.svelte` startup theme load to use `resolveThemeName()`
- [x] 4.4 Add matching `matchMedia` listener in `settings/+page.svelte`
- [x] 4.5 Update both `onSettingsChanged` handlers — remove `theme` field reference, re-resolve on any of `theme_mode`, `theme_light`, or `theme_dark` change

## 5. Frontend — Appearance Section UI

- [x] 5.1 Replace the single theme list in `AppearanceSection.svelte` with a mode selector (Auto / Light / Dark) — three-button segmented control style
- [x] 5.2 Add a Light theme picker (same card layout as current picker) wired to `theme_light`
- [x] 5.3 Add a Dark theme picker wired to `theme_dark`
- [x] 5.4 Implement active-card highlight logic: light picker shows active card when mode=light, or mode=auto and OS is light; dark picker shows active card when mode=dark, or mode=auto and OS is dark
- [x] 5.5 Implement deferred preview: clicking a card in the non-active picker saves via `setSetting` only; clicking in the active picker saves and calls `applyTheme()`
- [x] 5.6 Update mode selector `onclick` handlers to resolve and apply theme immediately on mode change

## 6. Cleanup & Verification

- [x] 6.1 Search codebase for any remaining references to `settings.theme` (the old single field) and update or remove them
- [x] 6.2 Run `npm run check` — confirm zero type errors
- [x] 6.3 Smoke test: new install defaults (auto mode, both pickers show Pomotroid active)
- [x] 6.4 Smoke test: switching modes and pickers — correct theme applies or defers
- [x] 6.5 Smoke test: live OS switch while in Auto mode — theme changes in both windows
