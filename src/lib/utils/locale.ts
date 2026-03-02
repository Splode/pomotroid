// Locale resolution utility.
// Maps the stored `language` setting to a Paraglide locale tag.

import { locales, baseLocale } from '$paraglide/runtime.js';

/** Supported locale tags (matches project.inlang locales). */
export type SupportedLocale = typeof locales[number];

/**
 * Resolve the active locale from the stored `language` setting.
 *
 * - `'auto'` → detect from `navigator.language`, prefix-matching supported locales.
 * - Any known locale tag → return as-is.
 * - Unknown tag or no match → fallback to `baseLocale` ('en').
 */
export function resolveLocale(language: string): SupportedLocale {
  const tag = language === 'auto' ? navigator.language : language;
  return matchLocale(tag);
}

function matchLocale(tag: string): SupportedLocale {
  // Exact match first.
  if ((locales as readonly string[]).includes(tag)) {
    return tag as SupportedLocale;
  }
  // Prefix match: 'de-AT' → 'de'.
  const prefix = tag.split('-')[0];
  if ((locales as readonly string[]).includes(prefix)) {
    return prefix as SupportedLocale;
  }
  return baseLocale as SupportedLocale;
}
