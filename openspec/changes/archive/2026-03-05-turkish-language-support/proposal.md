## Why

Turkish is a widely spoken language with a large desktop-software user base, and several community members have expressed interest in a Turkish localization. Adding Turkish (`tr`) extends the app's reach without requiring any architectural changes — the Paraglide JS infrastructure already supports adding locales by dropping in a message file.

## What Changes

- A new `src/messages/tr.json` file is added containing Turkish translations for all 109 keys defined in `src/messages/en.json`.
- `"tr"` is added to the `locales` array in `project.inlang/settings.json` so Paraglide picks it up at build time.
- The language dropdown in Settings → System gains a "Turkish" option.

## Capabilities

### New Capabilities

*(none — this change adds a locale within the existing localization system)*

### Modified Capabilities

- `localization`: The supported locale set gains `tr`. The requirement "Supported locales at launch" is updated to reflect 8 locales: English, Spanish, French, German, Japanese, Chinese (Simplified), Portuguese, and Turkish.

## Impact

- **`src/messages/tr.json`** — new file, 109 translated keys
- **`project.inlang/settings.json`** — add `"tr"` to `locales` array
- **`src/lib/components/settings/sections/SystemSection.svelte`** (or equivalent language picker) — add Turkish option to the dropdown
- No Rust changes, no migrations, no new dependencies
