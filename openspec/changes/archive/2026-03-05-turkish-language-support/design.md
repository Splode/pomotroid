## Context

Pomotroid uses Paraglide JS v2 for localization. Locales are defined by:

1. A JSON message file at `src/messages/<locale>.json` with one key per translatable string.
2. An entry in the `locales` array in `project.inlang/settings.json`.
3. An option in the language picker UI (Settings → System).

Paraglide generates type-safe message functions at build time from the message files. Adding a new locale requires no code changes beyond the three items above — the Paraglide plugin handles code generation automatically.

The English file (`src/messages/en.json`) is the source of truth. It currently contains 109 keys covering settings labels, timer controls, notifications, navigation, and system UI. All non-English files must contain every key present in `en.json`.

## Goals / Non-Goals

**Goals:**

- Ship a complete `tr.json` with all 109 keys translated to Turkish.
- Register `"tr"` in `project.inlang/settings.json`.
- Add Turkish to the language picker dropdown.

**Non-Goals:**

- Human review of translations (initial translations are machine-generated; community corrections can follow via the standard contribution workflow).
- Right-to-left layout changes (Turkish is a left-to-right language).
- Any changes to the Paraglide build pipeline.

## Decisions

### Decision 1: Machine-translate the initial file, consistent with existing locales

The existing non-English locales (Spanish, French, German, Japanese, Chinese, Portuguese) are machine-translated. Turkish will follow the same approach. The translations will be reviewed and refined by community contributors via pull requests — this is the established pattern for Pomotroid localization.

### Decision 2: Locale code `tr` (ISO 639-1)

Paraglide uses BCP 47 tags. `tr` is the correct tag for Turkish (no region qualifier needed for the base language).

## Risks / Trade-offs

- **Translation quality**: Machine translations for Turkish may have grammatical quirks due to Turkish agglutinative morphology. Community review is the mitigation path, consistent with existing locales.
- **Key completeness**: If `en.json` gains new keys after `tr.json` is written, Paraglide will fall back to English for those keys. This is the same risk that applies to all other locales and is acceptable.
