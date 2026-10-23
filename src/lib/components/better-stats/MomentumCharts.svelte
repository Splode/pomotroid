<script lang="ts">
  /**
   * Momentum charts: the daily/weekly progression with a moving average, plus
   * focus minutes per period.
   */
  import type { Insights, TrendPoint } from '$lib/types';
  import { fmtMins, fmtDay, fmtWeek, fmtAvg } from '$lib/utils/statsFormat';
  import TrendChart from './charts/TrendChart.svelte';
  import MiniBars from './charts/MiniBars.svelte';
  import SectionBlock from './SectionBlock.svelte';
  import * as m from '$paraglide/messages.js';

  let { insights }: { insights: Insights } = $props();

  type Mode = 'days' | 'weeks';
  let mode = $state<Mode>('days');

  let points = $derived<TrendPoint[]>(mode === 'days' ? insights.trend : insights.weeks);
  let overlay = $derived<number[]>(mode === 'days' ? insights.moving_avg_7 : []);

  let totalRounds = $derived(points.reduce((s, p) => s + p.rounds, 0));
  let totalFocus = $derived(points.reduce((s, p) => s + p.focus_mins, 0));
  let activeCount = $derived(points.filter((p) => p.rounds > 0).length);
  let best = $derived(
    points.reduce<TrendPoint | null>((b, p) => (!b || p.rounds > b.rounds ? p : b), null)
  );

  /** Label a point for the axis / tooltip depending on the active mode. */
  function label(p: TrendPoint): string {
    return mode === 'days' ? fmtDay(p.date) : `Week of ${fmtWeek(p.date)}`;
  }

  /** Show a tick every 4th day / 2nd week, plus the final point. */
  function tick(p: TrendPoint, i: number): string {
    const step = mode === 'days' ? 4 : 2;
    if (i % step !== 0 && i !== points.length - 1) return '';
    const short = label(p).replace('Week of ', '');
    return short.split(' ')[1] ?? short;
  }

  /** Weekly totals for the focus-minutes bar chart. */
  let weekFocus = $derived(insights.weeks.map((w) => w.focus_mins));
  let weekLabels = $derived(insights.weeks.map((w) => fmtWeek(w.date)));
</script>

<SectionBlock title={m.better_progress_title()} hint={mode === 'days' ? m.better_trend_hint() : m.better_weeks_hint()}>
  {#snippet headerExtra()}
    <div class="toggle">
      <button class="toggle-btn" class:active={mode === 'days'} onclick={() => (mode = 'days')}
        >{m.better_view_days()}</button
      >
      <button class="toggle-btn" class:active={mode === 'weeks'} onclick={() => (mode = 'weeks')}
        >{m.better_view_weeks()}</button
      >
    </div>
  {/snippet}

  <div class="summary">
    <span class="summary-item"><b>{totalRounds}</b> {m.better_rounds().toLowerCase()}</span>
    <span class="summary-dot">·</span>
    <span class="summary-item"><b>{fmtMins(totalFocus)}</b> {m.better_focus_time().toLowerCase()}</span>
    <span class="summary-dot">·</span>
    <span class="summary-item"
      ><b>{activeCount}</b>/{points.length} {m.better_active_days().toLowerCase()}</span
    >
    <span class="summary-dot">·</span>
    <span class="summary-item"
      >{m.better_consistency_avg()}: <b>{fmtAvg(totalRounds / Math.max(1, points.length))}</b></span
    >
    {#if best && best.rounds > 0}
      <span class="summary-best">
        {m.better_best_day()}: <b>{label(best)}</b> ({best.rounds})
      </span>
    {/if}
  </div>

  <TrendChart
    {points}
    {overlay}
    overlayLabel={m.better_moving_avg()}
    formatDate={label}
    tickLabel={tick}
    height={168}
  />

  <div class="split">
    <div class="panel">
      <span class="panel-title">{m.better_focus_per_week()}</span>
      <MiniBars
        values={weekFocus}
        labels={weekLabels}
        unit="min"
        secondary={insights.weeks.map((w) => w.rounds)}
        secondaryUnit={m.better_rounds().toLowerCase()}
        color="var(--color-long-round)"
        highlightMax
        labelEvery={2}
        height={90}
      />
    </div>

    <div class="panel">
      <span class="panel-title">{m.better_rounds_per_week()}</span>
      <MiniBars
        values={insights.weeks.map((w) => w.rounds)}
        labels={weekLabels}
        unit={m.better_rounds().toLowerCase()}
        secondary={insights.weeks.map((w) => w.focus_mins)}
        secondaryUnit="min"
        color="var(--color-focus-round)"
        highlightMax
        labelEvery={2}
        height={90}
      />
    </div>
  </div>
</SectionBlock>

<style>
  .toggle {
    display: flex;
    gap: 2px;
    padding: 2px;
    border-radius: 4px;
    background: var(--color-hover);
  }

  .toggle-btn {
    border: none;
    background: none;
    cursor: pointer;
    padding: 3px 10px;
    border-radius: 3px;
    font-size: 0.64rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    transition:
      background 0.15s,
      color 0.15s;
  }

  .toggle-btn:hover {
    color: var(--color-foreground);
  }

  .toggle-btn.active {
    background: color-mix(in oklch, var(--color-focus-round) 22%, var(--color-background));
    color: var(--color-focus-round);
  }

  .summary {
    display: flex;
    align-items: baseline;
    flex-wrap: wrap;
    gap: 6px;
    font-size: 0.7rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 88%, transparent);
  }

  .summary-item b {
    color: var(--color-foreground);
    font-variant-numeric: tabular-nums;
  }

  .summary-dot {
    color: color-mix(in oklch, var(--color-foreground) 25%, transparent);
  }

  .summary-best {
    margin-left: auto;
    font-size: 0.66rem;
    color: var(--color-foreground-darker);
  }

  .summary-best b {
    color: var(--color-focus-round);
  }

  .split {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 10px;
  }

  .panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 14px;
    border-radius: 6px;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 10%, transparent);
    background: color-mix(in oklch, var(--color-foreground) 4%, var(--color-background));
    min-width: 0;
  }

  .panel-title {
    font-size: 0.6rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }
</style>
