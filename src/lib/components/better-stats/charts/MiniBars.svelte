<script lang="ts">
  /**
   * Compact vertical bar chart for a labelled series.
   *
   * Used for the weekly rollup and the hour-of-day profile, where the bar count
   * is known and the labels are short.
   */
  interface Props {
    /** Bar values, index-aligned with `labels`. */
    values: number[];
    labels: string[];
    /** Tooltip/axis suffix for each bar, e.g. "rounds". */
    unit?: string;
    /** Optional per-bar secondary value shown in the tooltip. */
    secondary?: (number | null)[];
    secondaryUnit?: string;
    height?: number;
    color?: string;
    /** Highlight the bar with the largest value. */
    highlightMax?: boolean;
    /** Show every nth label to avoid crowding. */
    labelEvery?: number;
  }

  let {
    values,
    labels,
    unit = '',
    secondary = [],
    secondaryUnit = '',
    height = 96,
    color = 'var(--color-focus-round)',
    highlightMax = false,
    labelEvery = 1,
  }: Props = $props();

  let max = $derived(Math.max(1, ...values));
  let maxIndex = $derived(values.indexOf(Math.max(...values)));
  let hovered = $state<number | null>(null);
</script>

<div
  class="bars"
  role="group"
  aria-label={unit ? `Bars in ${unit}` : 'Bars'}
  style="height: {height + 16}px"
  onmouseleave={() => (hovered = null)}
>
  {#each values as v, i (i)}
    {@const pct = (v / max) * 100}
    <button
      type="button"
      class="col"
      aria-label="{labels[i] ?? i}: {v} {unit}"
      onmouseenter={() => (hovered = i)}
      onfocus={() => (hovered = i)}
      onblur={() => (hovered = null)}
    >
      {#if hovered === i}
        <div class="tip">
          <span class="tip-label">{labels[i] ?? i}</span>
          <span class="tip-value"><b>{v}</b> {unit}</span>
          {#if secondary[i] != null}
            <span class="tip-value">{secondary[i]} {secondaryUnit}</span>
          {/if}
        </div>
      {/if}
      <div
        class="bar"
        class:empty={v === 0}
        class:peak={highlightMax && i === maxIndex && v > 0}
        class:active={hovered === i}
        style="height: {Math.max(2, pct)}%; --bar-color: {color}"
      ></div>
      {#if i % labelEvery === 0}
        <span class="tick">{labels[i] ?? ''}</span>
      {/if}
    </button>
  {/each}
</div>

<style>
  .bars {
    display: flex;
    align-items: flex-end;
    gap: 3px;
    padding-bottom: 14px;
  }

  .col {
    position: relative;
    flex: 1 1 0;
    min-width: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-end;
    appearance: none;
    border: none;
    background: none;
    font: inherit;
    color: inherit;
    padding: 0;
    cursor: default;
  }

  .col:focus-visible {
    outline: 2px solid color-mix(in oklch, var(--color-focus-round) 70%, transparent);
    outline-offset: -1px;
    border-radius: 3px;
  }

  .bar {
    width: 100%;
    max-width: 24px;
    border-radius: 2px 2px 0 0;
    background: color-mix(in oklch, var(--bar-color) 70%, transparent);
    transform-origin: bottom;
    animation: bar-grow 0.4s cubic-bezier(0.22, 1, 0.36, 1) both;
    transition: background 0.15s;
  }

  @keyframes bar-grow {
    from {
      transform: scaleY(0.05);
      opacity: 0;
    }
    to {
      transform: scaleY(1);
      opacity: 1;
    }
  }

  .bar.empty {
    background: color-mix(in oklch, var(--color-foreground) 8%, transparent);
    animation: none;
  }

  .bar.peak {
    background: var(--bar-color);
  }

  .bar.active {
    background: var(--bar-color);
  }

  .tick {
    position: absolute;
    bottom: -13px;
    font-size: 8px;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground-darker);
    white-space: nowrap;
  }

  .tip {
    position: absolute;
    bottom: calc(100% - 8px);
    left: 50%;
    transform: translateX(-50%);
    z-index: 3;
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding: 5px 8px;
    border-radius: 4px;
    background: color-mix(in oklch, var(--color-foreground) 93%, var(--color-background));
    color: var(--color-background);
    font-size: 0.6rem;
    line-height: 1.3;
    white-space: nowrap;
    pointer-events: none;
    box-shadow: 0 4px 14px rgb(0 0 0 / 0.28);
  }

  .tip-label {
    font-weight: 700;
  }

  .tip-value b {
    font-variant-numeric: tabular-nums;
  }
</style>
