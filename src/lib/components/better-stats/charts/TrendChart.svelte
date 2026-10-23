<script lang="ts">
  /**
   * Dual-series line chart: a filled area for the primary series plus an
   * optional dashed overlay (used for the 7-day moving average).
   *
   * Rendered with a fixed viewBox and `preserveAspectRatio="none"` so the chart
   * scales to whatever width the panel gives it.
   */
  import type { TrendPoint } from '$lib/types';
  import { fmtMins } from '$lib/utils/statsFormat';

  interface Props {
    points: TrendPoint[];
    /** Secondary series aligned index-for-index with `points`. */
    overlay?: number[];
    overlayLabel?: string;
    /** Format a point for the hover tooltip. */
    formatDate: (point: TrendPoint) => string;
    /** Chart height in CSS pixels. */
    height?: number;
    /** Accent colour for the primary series. */
    color?: string;
    /** Tick labels drawn under the x axis, keyed by point index. */
    tickLabel?: (point: TrendPoint, index: number) => string;
  }

  let {
    points,
    overlay = [],
    overlayLabel = '',
    formatDate,
    height = 150,
    color = 'var(--color-focus-round)',
    tickLabel = () => '',
  }: Props = $props();

  // Internal coordinate system; the SVG stretches to the container width.
  const VB_W = 1000;
  const VB_H = 300;
  const PAD_BOTTOM = 34;

  let maxRounds = $derived(Math.max(1, ...points.map((p) => p.rounds), ...overlay));

  /** Horizontal position of point `i`, spread edge to edge. */
  function xAt(i: number): number {
    if (points.length <= 1) return VB_W / 2;
    return (i / (points.length - 1)) * VB_W;
  }

  /** Vertical position for a rounds value (0 sits on the baseline). */
  function yAt(v: number): number {
    const plotH = VB_H - PAD_BOTTOM;
    return plotH - (v / maxRounds) * plotH;
  }

  let linePath = $derived(
    points.map((p, i) => `${i === 0 ? 'M' : 'L'} ${xAt(i).toFixed(2)} ${yAt(p.rounds).toFixed(2)}`).join(' ')
  );

  let areaPath = $derived(
    points.length
      ? `${linePath} L ${VB_W} ${yAt(0)} L 0 ${yAt(0)} Z`
      : ''
  );

  let overlayPath = $derived(
    overlay.length
      ? overlay
          .map((v, i) => `${i === 0 ? 'M' : 'L'} ${xAt(i).toFixed(2)} ${yAt(v).toFixed(2)}`)
          .join(' ')
      : ''
  );

  let hovered = $state<number | null>(null);
  let svgEl = $state<SVGSVGElement | null>(null);

  /** Map a pointer position to the nearest point index. */
  function indexFromEvent(e: MouseEvent): number | null {
    if (!svgEl || points.length === 0) return null;
    const rect = svgEl.getBoundingClientRect();
    if (rect.width === 0) return null;
    const frac = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    return Math.round(frac * (points.length - 1));
  }

  let hoveredPoint = $derived(hovered !== null ? points[hovered] : null);
  let hoveredAvg = $derived(hovered !== null ? overlay[hovered] : null);

  /** Fraction across the chart where the hover marker sits (for HTML overlay). */
  let hoverFrac = $derived(hovered !== null && points.length > 1 ? hovered / (points.length - 1) : 0);
</script>

<div class="chart" style="height: {height}px">
  <svg
    bind:this={svgEl}
    class="svg"
    viewBox="0 0 {VB_W} {VB_H}"
    preserveAspectRatio="none"
    role="img"
    aria-label="Focus trend"
    onmousemove={indexFromEvent}
    onmouseleave={() => (hovered = null)}
    onfocus={() => (hovered = points.length - 1)}
    onblur={() => (hovered = null)}
    tabindex="-1"
  >
    <defs>
      <linearGradient id="area-fill" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color={color} stop-opacity="0.34" />
        <stop offset="100%" stop-color={color} stop-opacity="0.02" />
      </linearGradient>
    </defs>

    <!-- Horizontal guides -->
    {#each [0.25, 0.5, 0.75] as g}
      <line
        x1="0"
        x2={VB_W}
        y1={yAt(maxRounds * g)}
        y2={yAt(maxRounds * g)}
        class="grid"
        vector-effect="non-scaling-stroke"
      />
    {/each}

    {#if areaPath}
      <path d={areaPath} fill="url(#area-fill)" />
    {/if}

    <!-- Moving average overlay -->
    {#if overlayPath}
      <path
        d={overlayPath}
        class="overlay"
        stroke="var(--color-long-round)"
        vector-effect="non-scaling-stroke"
      />
    {/if}

    <path d={linePath} class="line" stroke={color} vector-effect="non-scaling-stroke" />

    <!-- Baseline -->
    <line x1="0" x2={VB_W} y1={yAt(0)} y2={yAt(0)} class="baseline" vector-effect="non-scaling-stroke" />

    {#if hovered !== null}
      <line
        x1={xAt(hovered)}
        x2={xAt(hovered)}
        y1="0"
        y2={yAt(0)}
        class="cursor"
        vector-effect="non-scaling-stroke"
      />
      <circle cx={xAt(hovered)} cy={yAt(points[hovered].rounds)} r="5" fill={color} vector-effect="non-scaling-stroke" />
    {/if}
  </svg>

  <!-- X-axis ticks live in HTML so they keep a constant font size -->
  <div class="ticks">
    {#each points as p, i (p.date)}
      {#if tickLabel(p, i)}
        <span class="tick" style="left: {(i / Math.max(1, points.length - 1)) * 100}%">{tickLabel(p, i)}</span>
      {/if}
    {/each}
  </div>

  {#if hoveredPoint}
    <div class="tip" style="left: {hoverFrac * 100}%">
      <span class="tip-date">{formatDate(hoveredPoint)}</span>
      <span class="tip-row">
        <span class="dot" style="background: {color}"></span>
        <b>{hoveredPoint.rounds}</b> rounds
      </span>
      <span class="tip-row"><span class="dot dot--muted"></span>{fmtMins(hoveredPoint.focus_mins)} focus</span>
      {#if hoveredAvg !== null && overlayLabel}
        <span class="tip-row">
          <span class="dot" style="background: var(--color-long-round)"></span>
          {overlayLabel}: <b>{hoveredAvg.toFixed(1)}</b>
        </span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .chart {
    position: relative;
    width: 100%;
    border-radius: 6px;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 10%, transparent);
    background: color-mix(in oklch, var(--color-foreground) 4%, var(--color-background));
    overflow: hidden;
  }

  .svg {
    display: block;
    width: 100%;
    height: 100%;
    cursor: crosshair;
  }

  .svg:focus-visible {
    outline: 2px solid color-mix(in oklch, var(--color-focus-round) 70%, transparent);
    outline-offset: -2px;
  }

  .grid {
    stroke: color-mix(in oklch, var(--color-foreground) 9%, transparent);
    stroke-width: 1;
    stroke-dasharray: 3 5;
  }

  .baseline {
    stroke: var(--color-separator);
    stroke-width: 1;
  }

  .line {
    fill: none;
    stroke-width: 2;
    stroke-linejoin: round;
    stroke-linecap: round;
  }

  .overlay {
    fill: none;
    stroke-width: 2;
    stroke-dasharray: 5 4;
    stroke-linejoin: round;
    opacity: 0.9;
  }

  .cursor {
    stroke: color-mix(in oklch, var(--color-foreground) 32%, transparent);
    stroke-width: 1;
    stroke-dasharray: 2 3;
  }

  .ticks {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 6px;
    height: 12px;
    pointer-events: none;
  }

  .tick {
    position: absolute;
    transform: translateX(-50%);
    font-size: 8px;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground-darker);
    white-space: nowrap;
  }

  .tip {
    position: absolute;
    top: 8px;
    transform: translateX(-50%);
    z-index: 3;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 6px 9px;
    border-radius: 4px;
    background: color-mix(in oklch, var(--color-foreground) 93%, var(--color-background));
    color: var(--color-background);
    font-size: 0.62rem;
    line-height: 1.35;
    white-space: nowrap;
    pointer-events: none;
    box-shadow: 0 4px 16px rgb(0 0 0 / 0.3);
  }

  .tip-date {
    font-weight: 700;
  }

  .tip-row {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .tip-row b {
    font-variant-numeric: tabular-nums;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .dot--muted {
    background: color-mix(in oklch, currentColor 45%, transparent);
  }
</style>
