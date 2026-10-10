<script lang="ts">
  // Row of toggle chips that choose which categories the stats include.
  // Several can be active at once; the selection is stored in settings.
  import type { Category } from '$lib/types';
  import { UNCATEGORIZED_ID, categoryLabel } from '$lib/utils/categories';
  import * as m from '$paraglide/messages.js';

  interface Props {
    categories: Category[];
    /** Hidden category ids; UNCATEGORIZED_ID stands for rounds without a category. */
    hidden: number[];
    /** Offer an "Uncategorized" chip (only when such rounds exist). */
    showUncategorized: boolean;
    ontoggle: (id: number) => void;
  }

  let { categories, hidden, showUncategorized, ontoggle }: Props = $props();
</script>

<div class="filter" role="group" aria-label={m.stats_filter_label()}>
  {#each categories as category (category.id)}
    {@const on = !hidden.includes(category.id)}
    <button class="chip" class:on aria-pressed={on} onclick={() => ontoggle(category.id)}>
      <span class="dot" style="background: {category.color}"></span>
      <span class="label">{categoryLabel(category)}</span>
    </button>
  {/each}
  {#if showUncategorized}
    {@const on = !hidden.includes(UNCATEGORIZED_ID)}
    <button class="chip" class:on aria-pressed={on} onclick={() => ontoggle(UNCATEGORIZED_ID)}>
      <span class="dot uncategorized"></span>
      <span class="label">{m.category_uncategorized()}</span>
    </button>
  {/if}
</div>

<style>
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 10px 24px;
    border-bottom: 1px solid var(--color-separator);
    flex-shrink: 0;
  }

  .chip {
    display: flex;
    align-items: center;
    gap: 6px;
    max-width: 160px;
    height: 24px;
    padding: 0 10px;
    background: none;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 16%, transparent);
    border-radius: 12px;
    color: var(--color-foreground-darker);
    font-size: 0.72rem;
    letter-spacing: 0.03em;
    cursor: pointer;
    opacity: 0.55;
    transition:
      opacity 0.15s,
      background 0.15s,
      border-color 0.15s,
      color 0.15s;
  }

  .chip:hover {
    opacity: 0.85;
  }

  .chip.on {
    opacity: 1;
    color: var(--color-foreground);
    background: var(--color-hover);
    border-color: color-mix(in oklch, var(--color-foreground) 28%, transparent);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .chip:not(.on) .dot {
    filter: grayscale(1);
  }

  .dot.uncategorized {
    background: transparent;
    border: 1.5px dashed var(--color-foreground-darker);
    box-sizing: border-box;
  }

  .label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
