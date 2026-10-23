<script lang="ts">
  /**
   * SVG donut for a small set of proportions.
   *
   * Each slice is drawn as a stroked arc segment; the caller supplies already
   * formatted legend labels.
   */

  export interface Slice {
    label: string;
    value: number;
    color: string;
  }

  interface Props {
    slices: Slice[];
    /** Text shown in the middle of the donut. */
    centerValue: string;
    centerCaption?: string | null;
    size?: number;
    thickness?: number;
  }

  let { slices, centerValue, centerCaption = null, size = 132, thickness = 16 }: Props = $props();

  let total = $derived(slices.reduce((s, x) => s + x.value, 0));
  let radius = $derived((size - thickness) / 2);
  let circumference = $derived(2 * Math.PI * radius);

  /** Arc segments with cumulative dash offsets, skipping empty slices. */
  let segments = $derived.by(() => {
    if (total <= 0) return [];
    let acc = 0;
    return slices
      .filter((s) => s.value > 0)
      .map((s) => {
        const frac = s.value / total;
        const seg = {
          ...s,
          frac,
          // A tiny gap between segments keeps the boundaries readable.
          dash: Math.max(0, frac * circumference - 2),
          offset: -acc * circumference,
        };
        acc += frac;
        return seg;
      });
  });
</script>

<div class="donut-wrap">
  <div class="donut" style="width: {size}px; height: {size}px">
    <svg width={size} height={size} viewBox="0 0 {size} {size}" role="img" aria-label={centerValue}>
      <circle
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke="color-mix(in oklch, var(--color-foreground) 10%, transparent)"
        stroke-width={thickness}
      />
      {#each segments as seg (seg.label)}
        <circle
          class="seg"
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke={seg.color}
          stroke-width={thickness}
          stroke-dasharray="{seg.dash} {circumference}"
          stroke-dashoffset={seg.offset}
          transform="rotate(-90 {size / 2} {size / 2})"
        >
          <title>{seg.label}: {Math.round(seg.frac * 100)}%</title>
        </circle>
      {/each}
    </svg>
    <div class="center">
      <span class="center-value">{centerValue}</span>
      {#if centerCaption}
        <span class="center-caption">{centerCaption}</span>
      {/if}
    </div>
  </div>

  <ul class="legend">
    {#each slices as s (s.label)}
      <li class="legend-row">
        <span class="swatch" style="background: {s.color}"></span>
        <span class="legend-label">{s.label}</span>
        <span class="legend-value"
          >{total > 0 ? Math.round((s.value / total) * 100) : 0}%</span
        >
      </li>
    {/each}
  </ul>
</div>

<style>
  .donut-wrap {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .donut {
    position: relative;
    flex-shrink: 0;
  }

  .seg {
    transition: stroke-dasharray 0.5s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .center {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    pointer-events: none;
  }

  .center-value {
    font-size: 1.2rem;
    font-weight: 700;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    color: var(--color-foreground);
  }

  .center-caption {
    font-size: 0.55rem;
    font-weight: 600;
    letter-spacing: 0.07em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .legend {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 5px;
    min-width: 150px;
    flex: 1 1 auto;
  }

  .legend-row {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 0.7rem;
  }

  .swatch {
    width: 9px;
    height: 9px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .legend-label {
    color: var(--color-foreground-darker);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .legend-value {
    margin-left: auto;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground);
  }
</style>
