import type { Settings } from '$lib/types';

/**
 * Resolve the active theme name from settings and the current OS color scheme.
 *
 * - `auto`  → use `theme_dark` when OS is dark, `theme_light` otherwise
 * - `light` → always `theme_light`
 * - `dark`  → always `theme_dark`
 */
export function resolveThemeName(settings: Settings, osDark: boolean): string {
  switch (settings.theme_mode) {
    case 'light':
      return settings.theme_light;
    case 'dark':
      return settings.theme_dark;
    default: // 'auto'
      return osDark ? settings.theme_dark : settings.theme_light;
  }
}
