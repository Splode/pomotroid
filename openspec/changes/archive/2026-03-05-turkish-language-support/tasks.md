## 1. Create the Turkish message file

- [x] 1.1 Create `src/messages/tr.json` with Turkish translations for all 109 keys from `src/messages/en.json`

## 2. Register the locale with Paraglide

- [x] 2.1 Add `"tr"` to the `locales` array in `project.inlang/settings.json`

## 3. Add Turkish to the language picker

- [x] 3.1 Add `{ value: 'tr', label: 'Türkçe' }` to the `LANGUAGES` array in `src/lib/components/settings/sections/SystemSection.svelte`

## 4. Verify

- [x] 4.1 Run `npm run check` to confirm no type errors from the new locale
- [x] 4.2 Run `npm run tauri dev`, switch language to Turkish in Settings → System, and verify UI strings display in Turkish
