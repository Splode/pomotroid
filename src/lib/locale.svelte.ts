import { overwriteGetLocale, baseLocale, locales } from '$paraglide/runtime.js';
import { resolveLocale } from '$lib/utils/locale';

type SupportedLocale = (typeof locales)[number];

// Module-level reactive state (Svelte 5 rune).
// Reading this inside any Svelte template expression — even through a function
// call chain like m.nav_timer() → getLocale() → currentLocale — creates a
// tracked dependency. When currentLocale changes, all those expressions
// automatically re-evaluate without a page reload.
let currentLocale: SupportedLocale = $state(baseLocale as SupportedLocale);

// Override Paraglide's getLocale so that every m.*() call reads from our
// reactive $state instead of its internal plain-JS variable.
overwriteGetLocale(() => currentLocale);

/**
 * Change the active locale. Call this instead of Paraglide's setLocale.
 * Accepts the raw DB value ('auto' | BCP-47 tag) and resolves it.
 */
export function setLocale(language: string): void {
	currentLocale = resolveLocale(language) as SupportedLocale;
}
