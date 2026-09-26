<script lang="ts">
  // Titlebar dropdown for picking the category that new focus rounds are
  // recorded under. Only rendered while categories are enabled.
  import { onMount } from 'svelte';
  import { settings } from '$lib/stores/settings';
  import { categories, syncCategories } from '$lib/stores/categories';
  import { setSetting } from '$lib/ipc';
  import { categoryLabel, resolveActiveCategory } from '$lib/utils/categories';
  import Tooltip from './Tooltip.svelte';
  import * as m from '$paraglide/messages.js';

  interface Props {
    /** Which edge of the trigger the menu lines up with. */
    align?: 'left' | 'right';
  }

  let { align = 'left' }: Props = $props();

  let open = $state(false);
  let rootEl: HTMLElement | undefined;

  const active = $derived(resolveActiveCategory($categories, $settings.active_category_id));

  // The compact window hides the name next to the dot, so the tooltip (also
  // the button's accessible name) names it too.
  const tooltip = $derived(
    `${active ? categoryLabel(active) : m.categories_none()} · ${m.tooltip_category_switcher()}`
  );

  onMount(() => {
    const unlisten = syncCategories();
    return () => {
      unlisten.then((fn) => fn());
    };
  });

  $effect(() => {
    if (!open) return;
    function onOutside(e: MouseEvent) {
      if (rootEl && !rootEl.contains(e.target as Node)) open = false;
    }
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') open = false;
    }
    window.addEventListener('mousedown', onOutside);
    window.addEventListener('keydown', onKey);
    return () => {
      window.removeEventListener('mousedown', onOutside);
      window.removeEventListener('keydown', onKey);
    };
  });

  function toggle() {
    // With every category deleted there is nothing to pick from.
    if (!open && $categories.length === 0) return;
    open = !open;
  }

  async function select(id: number) {
    open = false;
    if (id === active?.id) return;
    settings.set(await setSetting('active_category_id', String(id)));
  }
</script>

<div class="switcher" class:open bind:this={rootEl}>
  <Tooltip text={tooltip}>
    <button
      class="trigger"
      class:open
      onclick={toggle}
      aria-label={tooltip}
      aria-haspopup="listbox"
      aria-expanded={open}
    >
      {#if active}
        <span class="dot" style="background: {active.color}"></span>
        <span class="name">{categoryLabel(active)}</span>
      {:else}
        <span class="dot none"></span>
        <span class="name">{m.categories_none()}</span>
      {/if}
    </button>
  </Tooltip>

  {#if open}
    <ul class="menu" class:right={align === 'right'} role="listbox">
      {#each $categories as category (category.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <li
          class="option"
          class:selected={category.id === active?.id}
          role="option"
          aria-selected={category.id === active?.id}
          onmousedown={() => select(category.id)}
        >
          <span class="dot" style="background: {category.color}"></span>
          <span class="option-name">{categoryLabel(category)}</span>
          {#if category.id === active?.id}
            <svg class="check" width="10" height="8" viewBox="0 0 10 8" aria-hidden="true">
              <polyline
                points="1,4 4,7 9,1"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .switcher {
    position: relative;
    min-width: 0;
    margin: 0 4px;
  }

  /* Keep the tooltip from covering the open menu. */
  .switcher.open :global(.tooltip) {
    display: none;
  }

  .trigger {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 24px;
    max-width: 128px;
    padding: 0 8px;
    background: none;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 16%, transparent);
    border-radius: 12px;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.72rem;
    letter-spacing: 0.03em;
    cursor: pointer;
    transition:
      color 0.15s,
      background 0.15s,
      border-color 0.15s;
  }

  .trigger:focus {
    outline: none;
  }

  .trigger:focus-visible {
    outline: 2px solid color-mix(in oklch, var(--color-foreground) 45%, transparent);
    outline-offset: 2px;
  }

  .trigger:hover,
  .trigger.open {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  /* No category left to pick: a hollow dot, like "Uncategorized" in the stats. */
  .dot.none {
    background: transparent;
    border: 1.5px dashed currentColor;
    box-sizing: border-box;
  }

  .name,
  .option-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    min-width: 150px;
    max-width: 200px;
    /* Ten categories are taller than the compact window. */
    max-height: calc(100vh - 48px);
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: color-mix(in oklch, var(--color-foreground) 25%, transparent) transparent;
    background: var(--color-background-light);
    border: 1px solid color-mix(in oklch, var(--color-foreground) 15%, transparent);
    border-radius: 6px;
    box-shadow: 0 4px 16px color-mix(in oklch, black 25%, transparent);
    z-index: 300;
    list-style: none;
    margin: 0;
    padding: 4px 0;
  }

  .menu.right {
    left: auto;
    right: 0;
  }

  .option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    font-size: 0.78rem;
    color: var(--color-foreground);
    cursor: pointer;
    transition: background 0.1s;
  }

  .option:hover {
    background: var(--color-hover);
  }

  .option.selected .option-name {
    font-weight: 600;
  }

  .check {
    margin-left: auto;
    flex-shrink: 0;
    color: var(--color-accent);
  }

  /* Compact window: keep only the color dot, in a round button, so the titlebar still fits. */
  @media (max-width: 299px) {
    .trigger {
      justify-content: center;
      width: 24px;
      padding: 0;
    }

    .trigger .name {
      display: none;
    }
  }
</style>
