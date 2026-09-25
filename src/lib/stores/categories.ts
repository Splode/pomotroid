// Categories store for the current window.
// Each window loads the list itself and follows `categories:changed`, which
// Rust emits after every create, update or delete from any window.

import { writable } from 'svelte/store';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { categoriesList, onCategoriesChanged } from '$lib/ipc';
import type { Category } from '$lib/types';

export const categories = writable<Category[]>([]);

/** Load the categories and keep the store in sync. Returns the unlisten function. */
export async function syncCategories(): Promise<UnlistenFn> {
  // Subscribe first so a change made while the list loads is not missed.
  const unlisten = await onCategoriesChanged((list) => categories.set(list));
  categories.set(await categoriesList());
  return unlisten;
}
