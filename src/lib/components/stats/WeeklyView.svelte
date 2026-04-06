<script lang="ts">
  import type { DayStat, StreakInfo, LabelStat, DayLabelStat } from '$lib/types';
  import * as m from '$paraglide/messages.js';
  import { getLocale } from '$paraglide/runtime.js';
  import LabelBreakdown from './LabelBreakdown.svelte';

  let { week, streak, labelBreakdown = [], weeklyLabels = [], onrename }: {
    week: DayStat[] | null;
    streak: StreakInfo | null;
    labelBreakdown?: LabelStat[];
    weeklyLabels?: DayLabelStat[];
    onrename?: (from: string, to: string) => void;
  } = $props();

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

  // Same palette as LabelBreakdown — must stay in sync.
  const SLICE_COLORS = [
    'var(--color-focus-round)',
    'var(--color-short-round)',
    'var(--color-long-round)',
    'color-mix(in oklch, var(--color-focus-round) 60%, var(--color-long-round))',
    'color-mix(in oklch, var(--color-short-round) 60%, var(--color-focus-round))',
  ];
  const UNLABELED_COLOR = 'color-mix(in oklch, var(--color-foreground-darker) 35%, transparent)';

  // Map label → color using the same index ordering as LabelBreakdown (sorted by weekly duration).
  const labelColorMap = $derived.by(() => {
    const map = new Map<string | null, string>();
    const sorted = [...labelBreakdown]
      .filter((e) => e.label !== null)
      .sort((a, b) => b.duration_mins - a.duration_mins);
    sorted.forEach((e, i) => map.set(e.label, SLICE_COLORS[i % SLICE_COLORS.length]));
    map.set(null, UNLABELED_COLOR);
    return map;
  });

  // Group weeklyLabels by date for O(1) tooltip lookup.
  const labelsByDate = $derived.by(() => {
    const map = new Map<string, DayLabelStat[]>();
    for (const entry of weeklyLabels) {
      const list = map.get(entry.date) ?? [];
      list.push(entry);
      map.set(entry.date, list);
    }
    return map;
  });

  // Tooltip — viewport-fixed so it escapes SVG/overflow clipping.
  let tooltip = $state<{ x: number; y: number; date: string; rounds: number; entries: DayLabelStat[] } | null>(null);
  let tooltipEl = $state<HTMLDivElement | undefined>(undefined);

  function showTooltip(event: MouseEvent, date: string, rounds: number) {
    const entries = labelsByDate.get(date) ?? [];
    const rect = (event.currentTarget as SVGRectElement).getBoundingClientRect();
    tooltip = { x: rect.left + rect.width / 2, y: rect.top - 8, date, rounds, entries };
  }

  function fmtMins(mins: number): string {
    if (mins < 60) return `${mins}m`;
    const h = Math.floor(mins / 60);
    const m = mins % 60;
    return m === 0 ? `${h}h` : `${h}h ${m}m`;
  }
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
          onmouseleave={() => { tooltip = null; }}
          role="img"
          aria-label="Weekly activity chart"
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

            <!-- Invisible full-column hit area for tooltip (covers bar + empty space above) -->
            {#if day.rounds > 0}
              <rect
                {x}
                y={0}
                width={BAR_W}
                height={CHART_H}
                fill="transparent"
                class="hit-area"
                role="img"
                aria-label="{day.date} label breakdown"
                onmouseenter={(e) => showTooltip(e, day.date, day.rounds)}
                onmouseleave={() => { tooltip = null; }}
              />
            {/if}
          {/each}

          <!-- Baseline -->
          <line
            x1="0" y1={CHART_H}
            x2={CHART_W} y2={CHART_H}
            class="baseline"
          />
        </svg>

        <!-- Bar tooltip (viewport-fixed to escape SVG clipping) -->
        {#if tooltip}
          {@const pad = 8}
          {@const w = tooltipEl?.offsetWidth ?? 0}
          {@const rawLeft = tooltip.x - w / 2}
          {@const left = Math.max(pad, Math.min(rawLeft, window.innerWidth - w - pad))}
          {@const arrowLeft = tooltip.x - left}
          <div
            bind:this={tooltipEl}
            class="bar-tooltip"
            style="top:{tooltip.y}px;left:{left}px;--arrow-left:{arrowLeft}px"
          >
            <div class="bar-tooltip-date">{tooltip.date}</div>
            {#if tooltip.entries.length === 0 || tooltip.entries.every((e) => e.label === null)}
              <div class="bar-tooltip-row bar-tooltip-no-labels">
                {tooltip.rounds} {tooltip.rounds === 1 ? 'round' : 'rounds'}
              </div>
            {:else}
              {#each tooltip.entries as entry}
                <div class="bar-tooltip-row">
                  <span class="bar-tooltip-dot" style="background: {labelColorMap.get(entry.label) ?? UNLABELED_COLOR}"></span>
                  <span class="bar-tooltip-label" class:bar-tooltip-unlabeled={entry.label === null}>{entry.label ?? '(unlabeled)'}</span>
                  <span class="bar-tooltip-mins">{fmtMins(entry.duration_mins)}</span>
                </div>
              {/each}
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <LabelBreakdown entries={labelBreakdown} variant="list" {onrename} />
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
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

  .chart-wrap { overflow: visible; position: relative; }
  .chart { display: block; overflow: visible; }

  .hit-area { cursor: default; }

  /* ── Bar tooltip ─────────────────────────────────────────── */
  .bar-tooltip {
    --tooltip-bg: var(--color-background-light, color-mix(in oklch, var(--color-foreground) 10%, var(--color-background)));
    position: fixed;
    transform: translateY(-100%);
    background: var(--tooltip-bg);
    color: var(--color-foreground);
    font-size: 0.72rem;
    line-height: 1.4;
    padding: 7px 10px;
    border-radius: 4px;
    width: max-content;
    max-width: 200px;
    pointer-events: none;
    z-index: 9999;
    box-shadow: 0 2px 8px color-mix(in oklch, black 30%, transparent);
    border: 1px solid color-mix(in oklch, var(--color-foreground) 12%, transparent);
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .bar-tooltip::after {
    content: '';
    position: absolute;
    top: 100%;
    left: var(--arrow-left, 50%);
    transform: translateX(-50%);
    border: 5px solid transparent;
    border-top-color: var(--tooltip-bg);
  }

  .bar-tooltip-date {
    font-size: 0.67rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--color-foreground-darker);
    margin-bottom: 2px;
  }

  .bar-tooltip-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .bar-tooltip-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .bar-tooltip-label {
    flex: 1;
    color: var(--color-foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }


  .bar-tooltip-no-labels {
    color: var(--color-foreground-darker);
  }

  .bar-tooltip-unlabeled {
    color: var(--color-foreground-darker);
    font-style: italic;
  }

  .bar-tooltip-mins {
    color: var(--color-foreground-darker);
    font-variant-numeric: tabular-nums;
    flex-shrink: 0;
  }

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
    cursor: default;
  }

  .day-label {
    fill: var(--color-foreground-darker);
    font-size: 10px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    cursor: default;
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
