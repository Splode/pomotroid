<script lang="ts">
  import type { DayStat, StreakInfo } from '$lib/types';
  import * as m from '$paraglide/messages.js';
  import { getLocale } from '$paraglide/runtime.js';

  let { week, streak }: { week: DayStat[] | null; streak: StreakInfo | null } = $props();

  const CHART_H = 140;  // px, max bar height
  const BAR_W   = 52;   // px per bar
  const BAR_GAP = 16;   // px between bars
  const CHART_W = 7 * (BAR_W + BAR_GAP) - BAR_GAP; // 412px

  // Reactive to app language setting — updates when user changes language in Settings.
  const shortFmt  = $derived(new Intl.DateTimeFormat(getLocale(), { weekday: 'short' }));
  const narrowFmt = $derived(new Intl.DateTimeFormat(getLocale(), { weekday: 'narrow' }));

  // Build a 7-day array (today and the previous 6 days), oldest first.
  const days = $derived.by(() => {
    const countByDate = new Map((week ?? []).map((d) => [d.date, d.rounds]));

    const today = new Date();
    today.setHours(0, 0, 0, 0);

    return Array.from({ length: 7 }, (_, i) => {
      const d = new Date(today);
      d.setDate(today.getDate() - (6 - i));
      const dateStr = [
        d.getFullYear(),
        String(d.getMonth() + 1).padStart(2, '0'),
        String(d.getDate()).padStart(2, '0'),
      ].join('-');
      return {
        date: dateStr,
        label: shortFmt.format(d),
        short: narrowFmt.format(d),
        rounds: countByDate.get(dateStr) ?? 0,
        isToday: i === 6,
      };
    });
  });

  const maxRounds = $derived(Math.max(1, ...days.map((d) => d.rounds)));
  const totalWeek  = $derived(days.reduce((s, d) => s + d.rounds, 0));
  const hasData    = $derived(totalWeek > 0);
</script>

<div class="view">
  <!-- Summary row -->
  <div class="summary">
    <div class="summary-item">
      <span class="summary-label">{m.stats_this_week()}</span>
      <span class="summary-value">{totalWeek} {m.stats_rounds().toLowerCase()}</span>
    </div>
    {#if streak}
      <div class="summary-item streak">
        <span class="summary-label">{m.stats_current_streak()}</span>
        <span class="summary-value">
          {#if streak.current > 0}
            <span class="flame" aria-hidden="true">🔥</span>{streak.current}
            {streak.current === 1 ? m.stats_day() : m.stats_days()}
          {:else}
            <span class="streak-none">{m.stats_no_active_streak()}</span>
          {/if}
        </span>
      </div>
    {/if}
  </div>

  <!-- Bar chart -->
  <div class="chart-section">
    {#if !hasData}
      <div class="empty">
        <span>{m.stats_no_sessions_week()}</span>
      </div>
    {:else}
      <div class="chart-wrap">
        <svg
          width={CHART_W}
          height={CHART_H + 36}
          viewBox="0 0 {CHART_W} {CHART_H + 36}"
          class="chart"
        >
          {#each days as day, i}
            {@const barH = Math.max(day.rounds > 0 ? 4 : 0, Math.round((day.rounds / maxRounds) * CHART_H))}
            {@const x = i * (BAR_W + BAR_GAP)}
            {@const y = CHART_H - barH}

            <!-- Bar -->
            <rect
              {x}
              {y}
              width={BAR_W}
              height={barH}
              rx="3"
              class="bar"
              class:bar-today={day.isToday}
              class:bar-empty={day.rounds === 0}
              style="--bar-delay: {i * 40}ms"
            />

            <!-- Round count label above bar -->
            {#if day.rounds > 0}
              <text
                x={x + BAR_W / 2}
                y={y - 5}
                text-anchor="middle"
                class="count-label"
              >{day.rounds}</text>
            {/if}

            <!-- Day label -->
            <text
              x={x + BAR_W / 2}
              y={CHART_H + 20}
              text-anchor="middle"
              class="day-label"
              class:day-label-today={day.isToday}
            >{day.short}</text>
          {/each}

          <!-- Baseline -->
          <line
            x1="0" y1={CHART_H}
            x2={CHART_W} y2={CHART_H}
            class="baseline"
          />
        </svg>
      </div>
    {/if}
  </div>
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100%;
    animation: app-fade-in 0.2s ease;
  }

  /* ── Summary row ─────────────────────────────────────────── */
  .summary {
    display: flex;
    gap: 32px;
    padding: 20px 32px 16px;
    border-bottom: 1px solid var(--color-separator);
    flex-shrink: 0;
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .summary-label {
    font-size: 0.67rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .summary-value {
    font-size: 1.1rem;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground);
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .streak-none {
    font-size: 0.9rem;
    font-weight: 400;
    color: var(--color-foreground-darker);
  }

  .flame {
    font-size: 1rem;
  }

  /* ── Bar chart ───────────────────────────────────────────── */
  .chart-section {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px 32px;
  }

  .empty {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-foreground-darker);
    font-size: 0.85rem;
    font-style: italic;
    opacity: 0.7;
  }

  .chart-wrap { overflow: visible; }
  .chart { display: block; overflow: visible; }

  .bar {
    fill: color-mix(in oklch, var(--color-focus-round) 55%, var(--color-background-light));
    transform-origin: bottom;
    transform-box: fill-box;
    animation: bar-rise 0.45s cubic-bezier(0.22, 1, 0.36, 1) both;
    animation-delay: var(--bar-delay, 0ms);
  }

  @keyframes bar-rise {
    from { transform: scaleY(0); opacity: 0; }
    to   { transform: scaleY(1); opacity: 1; }
  }

  .bar-today {
    fill: var(--color-focus-round);
  }

  .bar-empty {
    fill: color-mix(in oklch, var(--color-foreground) 6%, transparent);
    animation: none;
  }

  .count-label {
    fill: var(--color-foreground-darker);
    font-size: 10px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    animation: app-fade-in 0.3s ease both;
    animation-delay: 0.35s;
  }

  .day-label {
    fill: var(--color-foreground-darker);
    font-size: 10px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .day-label-today {
    fill: var(--color-focus-round);
    font-weight: 700;
  }

  .baseline {
    stroke: var(--color-separator);
    stroke-width: 1;
  }
</style>
