<script lang="ts">
  import { onMount } from 'svelte';
  import { error as logError } from '@tauri-apps/plugin-log';
  import { settings } from '$lib/stores/settings';
  import { categories, syncCategories } from '$lib/stores/categories';
  import {
    setSetting,
    categoriesCreate,
    categoriesUpdate,
    categoriesDelete,
    categoriesRoundCount,
  } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';
  import {
    CATEGORY_COLORS,
    MAX_CATEGORIES,
    MAX_CATEGORY_NAME_CHARS,
    categoryLabel,
    nextCategoryColor,
  } from '$lib/utils/categories';
  import type { Category } from '$lib/types';
  import * as m from '$paraglide/messages.js';

  onMount(() => {
    const unlisten = syncCategories();
    return () => {
      unlisten.then((fn) => fn());
    };
  });

  async function toggleEnabled() {
    const value = $settings.categories_enabled ? 'false' : 'true';
    settings.set(await setSetting('categories_enabled', value));
  }

  /** Run a category mutation and adopt the updated list it returns. */
  async function mutate(action: () => Promise<Category[]>) {
    try {
      categories.set(await action());
    } catch (e) {
      await logError(`[categories] ${e}`);
    }
  }

  // --- Rename ---

  /** The name shown for a built-in category when the user has not renamed it. */
  function defaultLabel(category: Category): string | undefined {
    return category.builtin_key ? categoryLabel({ ...category, name: null }) : undefined;
  }

  async function rename(category: Category, input: HTMLInputElement) {
    const current = categoryLabel(category);
    const value = input.value.trim();
    // Clearing a built-in category's name restores its localized default.
    const name = value && value !== defaultLabel(category) ? value : null;
    // Nothing to save, or a user-created category left without a name:
    // show the current name again.
    if (value === current || name === category.name || (!value && !category.builtin_key)) {
      input.value = current;
      return;
    }
    await mutate(() => categoriesUpdate(category.id, name, category.color));
  }

  function onNameKeydown(e: KeyboardEvent, category: Category) {
    const input = e.currentTarget as HTMLInputElement;
    if (e.key === 'Enter') input.blur();
    if (e.key === 'Escape') {
      input.value = categoryLabel(category);
      input.blur();
    }
  }

  // --- Color ---

  let paletteFor = $state<number | null>(null);

  $effect(() => {
    if (paletteFor === null) return;
    function onOutside(e: MouseEvent) {
      // Clicks on the open swatch or its palette are handled by their own buttons.
      if (!(e.target as Element).closest?.('.color-wrap.open')) paletteFor = null;
    }
    window.addEventListener('mousedown', onOutside);
    return () => window.removeEventListener('mousedown', onOutside);
  });

  async function setColor(category: Category, color: string) {
    paletteFor = null;
    if (color.toUpperCase() === category.color.toUpperCase()) return;
    await mutate(() => categoriesUpdate(category.id, category.name, color));
  }

  // --- Delete (with confirmation) ---

  let confirming = $state<{ id: number; rounds: number } | null>(null);

  async function askDelete(category: Category) {
    try {
      confirming = { id: category.id, rounds: await categoriesRoundCount(category.id) };
    } catch (e) {
      await logError(`[categories] ${e}`);
    }
  }

  async function confirmDelete() {
    if (!confirming) return;
    const { id } = confirming;
    confirming = null;
    await mutate(() => categoriesDelete(id));
  }

  // --- Add ---

  let newName = $state('');
  const atLimit = $derived($categories.length >= MAX_CATEGORIES);

  async function add() {
    const name = newName.trim();
    if (!name || atLimit) return;
    await mutate(() => categoriesCreate(name, nextCategoryColor($categories)));
    newName = '';
  }
</script>

<div class="section">
  <SettingsToggle
    label={m.categories_toggle()}
    description={m.categories_toggle_desc()}
    checked={$settings.categories_enabled}
    onclick={toggleEnabled}
  />
  <p class="note">{m.categories_off_note()}</p>

  {#if $settings.categories_enabled}
    <div class="group-heading">{m.nav_categories()}</div>

    <ul class="list">
      {#each $categories as category (category.id)}
        <li class="item">
          <div class="color-wrap" class:open={paletteFor === category.id}>
            <button
              class="swatch"
              style="background: {category.color}"
              aria-label={m.categories_color()}
              title={m.categories_color()}
              onclick={() => (paletteFor = paletteFor === category.id ? null : category.id)}
            ></button>
            {#if paletteFor === category.id}
              <div class="palette">
                {#each CATEGORY_COLORS as color (color)}
                  <button
                    class="palette-color"
                    class:selected={color === category.color.toUpperCase()}
                    style="background: {color}"
                    aria-label={color}
                    onclick={() => setColor(category, color)}
                  ></button>
                {/each}
              </div>
            {/if}
          </div>

          <input
            class="name-input"
            value={categoryLabel(category)}
            placeholder={defaultLabel(category)}
            maxlength={MAX_CATEGORY_NAME_CHARS}
            spellcheck="false"
            onblur={(e) => rename(category, e.currentTarget)}
            onkeydown={(e) => onNameKeydown(e, category)}
          />

          <button
            class="icon-btn"
            aria-label={m.categories_delete()}
            title={m.categories_delete()}
            onclick={() => askDelete(category)}
          >
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none" aria-hidden="true">
              <path
                d="M2.5 4h11M6.5 4V2.5h3V4M4 4l.7 9.5h6.6L12 4"
                stroke="currentColor"
                stroke-width="1.3"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </li>

        {#if confirming?.id === category.id}
          <li class="confirm-row">
            <span class="confirm-label">
              {confirming.rounds > 0
                ? m.categories_delete_confirm({
                    name: categoryLabel(category),
                    count: confirming.rounds,
                  })
                : m.categories_delete_confirm_empty({ name: categoryLabel(category) })}
            </span>
            <div class="confirm-actions">
              <button class="confirm-cancel" onclick={() => (confirming = null)}
                >{m.categories_cancel()}</button
              >
              <button class="confirm-destructive" onclick={confirmDelete}
                >{m.categories_delete_action()}</button
              >
            </div>
          </li>
        {/if}
      {/each}
    </ul>

    <div class="add-row">
      <input
        class="name-input"
        bind:value={newName}
        placeholder={m.categories_new_placeholder()}
        maxlength={MAX_CATEGORY_NAME_CHARS}
        spellcheck="false"
        disabled={atLimit}
        onkeydown={(e) => {
          if (e.key === 'Enter') add();
        }}
      />
      <button class="add-btn" onclick={add} disabled={atLimit || !newName.trim()}>
        {m.categories_add()}
      </button>
      <span class="count">{$categories.length} / {MAX_CATEGORIES}</span>
    </div>
    {#if atLimit}
      <p class="note">{m.categories_limit_reached({ max: MAX_CATEGORIES })}</p>
    {/if}
  {/if}
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    padding-bottom: 20px;
  }

  .group-heading {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    margin: 0;
    padding: 16px 20px 6px;
  }

  .note {
    font-size: 0.75rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.65;
    padding: 10px 20px 0;
    margin: 0;
    line-height: 1.6;
  }

  .list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 20px;
    border-bottom: 1px solid var(--color-separator);
  }

  .color-wrap {
    position: relative;
    display: flex;
  }

  .swatch {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid color-mix(in oklch, var(--color-foreground) 20%, transparent);
    padding: 0;
    cursor: pointer;
    flex-shrink: 0;
  }

  .swatch:hover {
    border-color: var(--color-foreground);
  }

  .palette {
    position: absolute;
    top: calc(100% + 6px);
    left: -6px;
    display: grid;
    grid-template-columns: repeat(5, 18px);
    gap: 6px;
    padding: 8px;
    background: var(--color-background-light);
    border: 1px solid color-mix(in oklch, var(--color-foreground) 15%, transparent);
    border-radius: 6px;
    box-shadow: 0 4px 16px color-mix(in oklch, black 20%, transparent);
    z-index: 200;
  }

  .palette-color {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid transparent;
    padding: 0;
    cursor: pointer;
  }

  .palette-color:hover,
  .palette-color.selected {
    border-color: var(--color-foreground);
  }

  .name-input {
    flex: 1;
    min-width: 0;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: var(--color-foreground);
    font-size: 0.85rem;
    letter-spacing: 0.02em;
    padding: 4px 8px;
    outline: none;
    transition:
      border-color 0.15s,
      background 0.15s;
  }

  .name-input:hover {
    background: var(--color-hover);
  }

  .name-input:focus {
    background: var(--color-hover);
    border-color: var(--color-accent);
  }

  .name-input::placeholder {
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
  }

  .name-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    background: none;
    border: none;
    border-radius: 4px;
    color: var(--color-foreground-darker, var(--color-foreground));
    cursor: pointer;
    flex-shrink: 0;
    transition:
      color 0.15s,
      background 0.15s;
  }

  .icon-btn:hover {
    background: var(--color-hover);
    color: color-mix(in oklch, var(--color-focus-round) 80%, var(--color-foreground));
  }

  .confirm-row {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 20px;
    border-bottom: 1px solid var(--color-separator);
    background: color-mix(in oklch, var(--color-foreground) 4%, transparent);
  }

  .confirm-label {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    line-height: 1.5;
  }

  .confirm-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .confirm-cancel,
  .confirm-destructive,
  .add-btn {
    background: none;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 18%, transparent);
    border-radius: 4px;
    font-size: 0.8rem;
    padding: 5px 14px;
    cursor: pointer;
    transition:
      border-color 0.15s,
      color 0.15s,
      background 0.15s;
  }

  .confirm-cancel {
    color: var(--color-foreground-darker, var(--color-foreground));
  }

  .confirm-cancel:hover {
    border-color: color-mix(in oklch, var(--color-foreground) 40%, transparent);
    color: var(--color-foreground);
  }

  .confirm-destructive,
  .add-btn {
    color: var(--color-accent);
    border-color: color-mix(in oklch, var(--color-accent) 40%, transparent);
  }

  .confirm-destructive:hover,
  .add-btn:hover:not(:disabled) {
    background: color-mix(in oklch, var(--color-accent) 10%, transparent);
    border-color: var(--color-accent);
  }

  .add-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .add-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 20px 0;
  }

  .add-row .name-input {
    border-color: color-mix(in oklch, var(--color-foreground) 18%, transparent);
  }

  .count {
    font-size: 0.72rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    min-width: 36px;
    text-align: right;
  }
</style>
