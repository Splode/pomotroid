// Helpers shared by the category switcher, the Categories settings section
// and the stats filter.

import type { Category, Settings, StatsFilter } from '$lib/types';
import * as m from '$paraglide/messages.js';

/** Mirrors `queries::MAX_CATEGORIES` in Rust. */
export const MAX_CATEGORIES = 10;

/** Mirrors `queries::MAX_CATEGORY_NAME_CHARS` in Rust. */
export const MAX_CATEGORY_NAME_CHARS = 20;

/** Id used in `stats_hidden_categories` for rounds without a category. */
export const UNCATEGORIZED_ID = 0;

/** Colors offered for categories. Picked to stay readable on light and dark themes. */
export const CATEGORY_COLORS = [
  '#4A9FF5',
  '#F5A623',
  '#B57EDC',
  '#3DD68C',
  '#FF6B6B',
  '#2EC4B6',
  '#F06595',
  '#FFD43B',
  '#A0785A',
  '#94A3B8',
];

/** Display name: the user's name, or the localized name of a built-in category. */
export function categoryLabel(category: Category): string {
  if (category.name) return category.name;
  switch (category.builtin_key) {
    case 'work':
      return m.category_work();
    case 'study':
      return m.category_study();
    case 'leisure':
      return m.category_leisure();
    default:
      return '';
  }
}

/** The category new focus rounds are filed under: the stored choice if it
 *  still exists, otherwise the first category. Mirrors
 *  `queries::resolve_active_category` in Rust. */
export function resolveActiveCategory(categories: Category[], activeId: number): Category | null {
  return categories.find((c) => c.id === activeId) ?? categories[0] ?? null;
}

/** Parses the `stats_hidden_categories` setting, ignoring malformed values. */
export function parseHiddenCategories(json: string): number[] {
  try {
    const value: unknown = JSON.parse(json);
    return Array.isArray(value) ? value.filter((id) => Number.isInteger(id)) : [];
  } catch {
    return [];
  }
}

/** Stats filter for the current settings, or null (no filtering) when
 *  categories are off. */
export function statsFilterFor(settings: Settings): StatsFilter | null {
  if (!settings.categories_enabled) return null;
  const hidden = parseHiddenCategories(settings.stats_hidden_categories);
  return {
    hidden_ids: hidden.filter((id) => id !== UNCATEGORIZED_ID),
    hide_uncategorized: hidden.includes(UNCATEGORIZED_ID),
  };
}

/** First palette color not in use yet, so a new category is easy to tell apart. */
export function nextCategoryColor(categories: Category[]): string {
  const used = new Set(categories.map((c) => c.color.toUpperCase()));
  return (
    CATEGORY_COLORS.find((color) => !used.has(color)) ??
    CATEGORY_COLORS[categories.length % CATEGORY_COLORS.length]
  );
}
