<script lang="ts">
  /**
   * Top of the dashboard: today's progress ring, momentum against the trailing
   * average, and a head-to-head comparison table for the week.
   */
  import type { Insights } from '$lib/types';
  import { fmtMins, fmtRate, fmtDelta, fmtAvg } from '$lib/utils/statsFormat';
  import Ring from './charts/Ring.svelte';
  import MetaTile from './MetaTile.svelte';
  import * as m from '$paraglide/messages.js';

  let { insights }: { insights: Insights } = $props();

  /** Trailing 7-day average; falls back to the 28-day window, then today. */
  let baseline = $derived(
    insights.week.avg_rounds_per_active_day ??
      insights.month.avg_rounds_per_active_day ??
      null
  );

  /** Progress toward the baseline, clamped for the ring. */
  let progress = $derived(
    baseline && baseline > 0 ? Math.min(1, insights.today_rounds / baseline) : 0
  );

  /** The comparison rows, resolved to display strings. */
  const LABEL_KEYS: Record<string, () => string> = {
    rounds_week: () => m.better_compare_rounds(),
    focus_week: () => m.better_compare_focus(),
    active_days_week: () => m.better_compare_active(),
    avg_per_active_day: () => m.better_compare_avg(),
  };

  /** Format a comparison value according to its metric. */
  function fmtComparison(label: string, value: number): string {
    if (label === 'focus_week') return fmtMins(value);
    if (label === 'avg_per_active_day') return fmtAvg(value);
    return `${Math.round(value)}`;
  }

  let comparisons = $derived(insights.comparisons);

  /** Rounds still needed to match the baseline (`null` when there is no baseline). */
  let roundsBehind = $derived(insights.rounds_to_beat_average);

  let streakLabel = $derived(
    insights.streak.current === 1 ? m.stats_day() : m.stats_days()
  );
</script>

<div class="hero">
  <!-- Left: ring + today's headline numbers -->
  <div class="panel panel--today">
    <div class="panel-head">
      <span class="panel-title">{m.better_momentum_today()}</span>
      <span class="panel-date">{insights.today}</span>
    </div>

    <div class="today-body">
      <Ring
        value={progress}
        display={insights.today_rounds}
        caption={m.better_rounds().toLowerCase()}
        size={124}
        color={insights.ahead_of_average ? 'var(--color-short-round)' : 'var(--color-focus-round)'}
      />

      <div class="today-meta">
        <span class="today-focus">{fmtMins(insights.today_focus_mins)}</span>
        <span class="today-focus-label">{m.better_focus_time().toLowerCase()}</span>

        <span class="today-hint">
          {#if baseline === null || roundsBehind === null}
            {m.better_no_average()}
          {:else if roundsBehind > 1}
            {m.better_behind_average({ n: roundsBehind })}
          {:else if roundsBehind === 1}
            {m.better_behind_average_one({ n: 1 })}
          {:else if roundsBehind === 0}
            {m.better_average_matched()}
          {:else}
            {m.better_ahead_of_average()}
          {/if}
        </span>

        {#if baseline !== null}
          <span class="baseline-note">
            {m.better_avg_per_active_day()}: <b>{fmtAvg(baseline ?? 0)}</b>
          </span>
        {/if}
      </div>
    </div>
  </div>

  <!-- Right: streak + windows -->
  <div class="panel panel--windows">
    <div class="panel-head">
      <span class="panel-title">{m.better_section_momentum()}</span>
      <span class="panel-date">{m.better_delta_vs_prev()}</span>
    </div>

    <div class="streak-strip" class:at-risk={insights.streak.at_risk}>
      <div class="streak-value">
        {insights.streak.current}
        <span class="streak-unit">{streakLabel}</span>
      </div>
      <div class="streak-meta">
        <span class="streak-label">{m.better_records_streak()}</span>
        {#if insights.streak.at_risk}
          <span class="streak-warn">{m.better_streak_at_risk()}</span>
        {:else}
          <span class="streak-sub">
            {m.better_records_longest()}: <b>{insights.streak.longest}</b>
          </span>
        {/if}
      </div>
    </div>

    <ul class="compare">
      {#each comparisons as c (c.label)}
        {@const label = (LABEL_KEYS[c.label] ?? (() => c.label))()}
        {@const tone = c.delta_pct === null ? '' : c.delta_pct > 0 ? 'up' : c.delta_pct < 0 ? 'down' : ''}
        <li class="compare-row">
          <span class="compare-label">{label}</span>
          <span class="compare-values">
            <b>{fmtComparison(c.label, c.current)}</b>
            <span class="compare-prev">/ {fmtComparison(c.label, c.previous)}</span>
          </span>
          <span class="compare-delta" class:up={tone === 'up'} class:down={tone === 'down'}>
            {fmtDelta(c.delta_pct)}
          </span>
        </li>
      {/each}
    </ul>

    <div class="window-tiles">
      <MetaTile
        label={m.better_momentum_week()}
        value={`${insights.last_7_days.rounds}`}
        sub={`${fmtMins(insights.last_7_days.focus_mins)} · ${fmtRate(insights.last_7_days.completion_rate)}`}
      />
      <MetaTile
        label={m.better_momentum_month()}
        value={`${insights.month.rounds}`}
        sub={`${fmtMins(insights.month.focus_mins)} · ${insights.month.active_days}/${insights.month.days}d`}
      />
      <MetaTile
        label={m.better_last_90()}
        value={`${insights.last_90_days.rounds}`}
        sub={`${fmtMins(insights.last_90_days.focus_mins)} · ${fmtRate(insights.last_90_days.completion_rate)}`}
      />
    </div>
  </div>
</div>

<style>
  .hero {
    display: grid;
    grid-template-columns: minmax(240px, 0.85fr) minmax(280px, 1.15fr);
    gap: 10px;
    align-items: stretch;
  }

  @media (max-width: 660px) {
    .hero {
      grid-template-columns: 1fr;
    }
  }

  .panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 13px 15px;
    border-radius: 6px;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 10%, transparent);
    background: color-mix(in oklch, var(--color-foreground) 4%, var(--color-background));
    min-width: 0;
  }

  .panel-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 10px;
  }

  .panel-title {
    font-size: 0.6rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .panel-date {
    font-size: 0.6rem;
    font-variant-numeric: tabular-nums;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
  }

  /* ── Today ─────────────────────────────────────────────── */
  .today-body {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .today-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1 1 120px;
  }

  .today-focus {
    font-size: 1.35rem;
    font-weight: 700;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    color: var(--color-focus-round);
  }

  .today-focus-label {
    font-size: 0.56rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    margin-bottom: 6px;
  }

  .today-hint {
    font-size: 0.68rem;
    line-height: 1.35;
    color: color-mix(in oklch, var(--color-foreground-darker) 88%, transparent);
  }

  .baseline-note {
    margin-top: 4px;
    font-size: 0.62rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 75%, transparent);
  }

  .baseline-note b {
    color: var(--color-foreground);
    font-variant-numeric: tabular-nums;
  }

  /* ── Streak strip ──────────────────────────────────────── */
  .streak-strip {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 11px;
    border-radius: 5px;
    background: color-mix(in oklch, var(--color-focus-round) 12%, transparent);
    border-left: 3px solid var(--color-focus-round);
  }

  .streak-strip.at-risk {
    background: color-mix(in oklch, var(--color-long-round) 14%, transparent);
    border-left-color: var(--color-long-round);
  }

  .streak-value {
    display: flex;
    align-items: baseline;
    gap: 4px;
    font-size: 1.5rem;
    font-weight: 700;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.03em;
    color: var(--color-foreground);
  }

  .streak-unit {
    font-size: 0.6rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .streak-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .streak-label {
    font-size: 0.58rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .streak-sub {
    font-size: 0.64rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 85%, transparent);
  }

  .streak-sub b {
    color: var(--color-foreground);
  }

  .streak-warn {
    font-size: 0.64rem;
    color: var(--color-long-round);
  }

  /* ── Comparison table ──────────────────────────────────── */
  .compare {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .compare-row {
    display: flex;
    align-items: baseline;
    gap: 10px;
    font-size: 0.72rem;
  }

  .compare-label {
    color: var(--color-foreground-darker);
    min-width: 96px;
  }

  .compare-values {
    display: flex;
    align-items: baseline;
    gap: 5px;
    font-variant-numeric: tabular-nums;
  }

  .compare-values b {
    font-size: 0.88rem;
    color: var(--color-foreground);
  }

  .compare-prev {
    font-size: 0.68rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 72%, transparent);
  }

  .compare-delta {
    margin-left: auto;
    font-size: 0.68rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground-darker);
  }

  .compare-delta.up {
    color: var(--color-short-round);
  }

  .compare-delta.down {
    color: var(--color-long-round);
  }

  .window-tiles {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 7px;
    margin-top: auto;
  }
</style>
