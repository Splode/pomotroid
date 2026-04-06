<script lang="ts">
  import type { DailyStats, LabelStat } from '$lib/types';
  import * as m from '$paraglide/messages.js';
  import LabelBreakdown from './LabelBreakdown.svelte';

  let { today, labelBreakdown = [], onrename }: {
    today: DailyStats | null;
    labelBreakdown?: LabelStat[];
    onrename?: (from: string, to: string) => void;
  } = $props();

  const CHART_H = 80;   // px, max bar height in the hourly chart
  const CHART_W = 744;  // px, total SVG width for 24 bars
  const BAR_W   = 22;   // px per bar
  const BAR_GAP = 9;    // px between bars

  function fmtTime(mins: number): string {
    if (mins < 60) return `${mins}m`;
    const h = Math.floor(mins / 60);
    const m = mins % 60;
    return m === 0 ? `${h}h` : `${h}h ${m}m`;
  }

  function fmtRate(rate: number | null): string {
    if (rate === null) return '—';
    return `${Math.round(rate * 100)}%`;
  }

  const byHour = $derived(today?.by_hour ?? Array(24).fill(0));
  const maxHour = $derived(Math.max(1, ...byHour));
  const hasData = $derived(today !== null && today.rounds > 0);

  // Hour labels: show every 6 hours
  const hourLabels = [0, 6, 12, 18];
</script>

<div class="view">
  <!-- Stat cards -->
  <div class="cards">
    <div class="card" style="--delay: 0ms">
      <span class="card-label">{m.stats_rounds()}</span>
      <span class="card-value">{today?.rounds ?? '—'}</span>
    </div>
    <div class="card-divider"></div>
    <div class="card" style="--delay: 60ms">
      <span class="card-label">{m.stats_focus_time()}</span>
      <span class="card-value">{today ? fmtTime(today.focus_mins) : '—'}</span>
    </div>
    <div class="card-divider"></div>
    <div class="card" style="--delay: 120ms">
      <span class="card-label">{m.stats_completion()}</span>
      <span class="card-value">{today ? fmtRate(today.completion_rate) : '—'}</span>
    </div>
  </div>

  <!-- Hourly breakdown -->
  <div class="section">
    <div class="section-header">
      <span class="section-title">{m.stats_sessions_by_hour()}</span>
      {#if !hasData}
        <span class="empty-hint">{m.stats_no_sessions_today()}</span>
      {/if}
    </div>

    <div class="chart-wrap">
      <svg
        width={CHART_W}
        height={CHART_H + 28}
        viewBox="0 0 {CHART_W} {CHART_H + 28}"
        class="chart"
      >
        {#each byHour as count, h}
          {@const barH = Math.max(2, Math.round((count / maxHour) * CHART_H))}
          {@const x = h * (BAR_W + BAR_GAP)}
          {@const y = CHART_H - barH}

          <!-- Bar -->
          <rect
            {x}
            {y}
            width={BAR_W}
            height={barH}
            rx="2"
            class="bar"
            class:bar-empty={count === 0}
            style="--bar-scale: {barH / CHART_H}; --bar-delay: {h * 18}ms"
          />

          <!-- Hour label (every 6 hours) -->
          {#if hourLabels.includes(h)}
            <text
              x={x + BAR_W / 2}
              y={CHART_H + 18}
              text-anchor="middle"
              class="hour-label"
            >{h === 0 ? '12a' : h === 12 ? '12p' : h < 12 ? `${h}a` : `${h - 12}p`}</text>
          {/if}
        {/each}

        <!-- Baseline -->
        <line
          x1="0" y1={CHART_H}
          x2={CHART_W} y2={CHART_H}
          class="baseline"
        />
      </svg>
    </div>
  </div>

  <LabelBreakdown entries={labelBreakdown} variant="pie" {onrename} />
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding: 0;
    animation: app-fade-in 0.2s ease;
  }

  /* ── Stat cards ──────────────────────────────────────────── */
  .cards {
    display: flex;
    align-items: stretch;
    border-bottom: 1px solid var(--color-separator);
  }

  .card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 28px 24px;
    animation: card-rise 0.35s cubic-bezier(0.22, 1, 0.36, 1) both;
    animation-delay: var(--delay, 0ms);
  }

  @keyframes card-rise {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .card-label {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .card-value {
    font-size: 2.4rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    color: var(--color-foreground);
    line-height: 1;
  }

  .card-divider {
    width: 1px;
    background: var(--color-separator);
    align-self: stretch;
    margin: 12px 0;
  }

  /* ── Section ─────────────────────────────────────────────── */
  .section {
    display: flex;
    flex-direction: column;
    padding: 20px 24px 16px;
    gap: 12px;
  }

  .section-header {
    display: flex;
    align-items: baseline;
    gap: 12px;
  }

  .section-title {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .empty-hint {
    font-size: 0.72rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 60%, transparent);
    font-style: italic;
  }

  /* ── Hourly chart ────────────────────────────────────────── */
  .chart-wrap {
    overflow-x: auto;
    overflow-y: hidden;
  }

  .chart { display: block; }

  .bar {
    fill: var(--color-focus-round);
    opacity: 0.85;
    transform-origin: bottom;
    transform-box: fill-box;
    animation: bar-rise 0.4s cubic-bezier(0.22, 1, 0.36, 1) both;
    animation-delay: var(--bar-delay, 0ms);
  }

  @keyframes bar-rise {
    from { transform: scaleY(0.05); opacity: 0; }
    to   { transform: scaleY(1);    opacity: 0.85; }
  }

  .bar-empty {
    fill: color-mix(in oklch, var(--color-foreground) 8%, transparent);
    animation: none;
  }

  .hour-label {
    fill: var(--color-foreground-darker);
    font-size: 9px;
    font-variant-numeric: tabular-nums;
    cursor: default;
  }

  .baseline {
    stroke: var(--color-separator);
    stroke-width: 1;
  }
</style>
