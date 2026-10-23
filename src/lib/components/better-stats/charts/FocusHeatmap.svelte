<script lang="ts">
  /**
   * Focus activity heatmap with three scopes:
   *
   * - **Recent** — the trailing 120 days, as a rolling contribution grid.
   * - **Month** — one calendar month, laid out as a real calendar with weekday
   *   columns and week rows.
   * - **Year** — one calendar year, laid out like the recent view (weeks as
   *   columns) so a whole year fits horizontally.
   *
   * All three read from `daily` (every recorded day) so navigation is instant.
   */
  import type { TrendPoint } from '$lib/types';
  import { fmtMins, fmtDay } from '$lib/utils/statsFormat';
  import * as m from '$paraglide/messages.js';

  interface Props {
    /** Trailing 120 days, zero-filled. */
    recent: TrendPoint[];
    /** Every recorded day, oldest → newest. */
    daily: TrendPoint[];
    /** The user's "today", "YYYY-MM-DD". */
    today: string;
  }

  let { recent, daily, today }: Props = $props();

  type Scope = 'recent' | 'month' | 'year';
  let scope = $state<Scope>('recent');

  const CELL = 11;
  const GAP = 3;
  const MONTHS = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ];
  const WEEKDAY_LABELS = ['Mon', '', 'Wed', '', 'Fri', '', 'Sun'];

  // --- Date helpers (all date-only, no timezone drift) ----------------------

  function parts(date: string): [number, number, number] {
    const [y, mo, d] = date.split('-').map(Number);
    return [y, mo, d];
  }

  /** Monday-based weekday index (0 = Mon) for a "YYYY-MM-DD" string. */
  function weekdayOf(date: string): number {
    const [y, mo, d] = parts(date);
    return (new Date(Date.UTC(y, mo - 1, d)).getUTCDay() + 6) % 7;
  }

  // `today` is a stable prop for the lifetime of this window, so the cursors can
  // be seeded from it directly rather than mirroring it reactively.
  let [initYear, initMonth] = parts(today);

  /** The last day of a month, handling leap years. */
  function daysInMonth(year: number, month: number): number {
    return new Date(Date.UTC(year, month, 0)).getUTCDate();
  }

  function iso(year: number, month: number, day: number): string {
    return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
  }

  // --- Value lookup ---------------------------------------------------------

  let byDate = $derived.by(() => {
    const map = new Map<string, TrendPoint>();
    for (const p of daily) map.set(p.date, p);
    return map;
  });

  /** Look a day up, defaulting to an empty entry so calendars stay aligned. */
  function entryFor(date: string): TrendPoint {
    return byDate.get(date) ?? { date, rounds: 0, focus_mins: 0 };
  }

  // --- Navigation state -----------------------------------------------------

  let [todayY, todayM] = $derived(parts(today));

  /** Cursors for the month and year views. */
  let monthCursor = $state({ year: initYear, month: initMonth });
  let yearCursor = $state(initYear);

  // --- Column layouts -------------------------------------------------------

  interface Column {
    lead: number;
    days: TrendPoint[];
  }

  /** Pack a flat, chronological day list into Monday-start columns. */
  function toColumns(points: TrendPoint[], lead = 0): Column[] {
    if (points.length === 0) return [];
    const cols: Column[] = [];
    let current: Column = { lead, days: [] };
    for (const p of points) {
      if (current.days.length + current.lead === 7) {
        cols.push(current);
        current = { lead: 0, days: [] };
      }
      current.days.push(p);
    }
    if (current.days.length || current.lead) cols.push(current);
    return cols;
  }

  /** Days for the month view: every calendar day, zero-filled. */
  let monthDays = $derived.by<TrendPoint[]>(() => {
    const { year, month } = monthCursor;
    const total = daysInMonth(year, month);
    const out: TrendPoint[] = [];
    for (let d = 1; d <= total; d += 1) {
      const date = iso(year, month, d);
      out.push(entryFor(date));
    }
    return out;
  });

  /** Days for the year view: every calendar day of the year. */
  let yearDays = $derived.by<TrendPoint[]>(() => {
    const out: TrendPoint[] = [];
    for (let mo = 1; mo <= 12; mo += 1) {
      const total = daysInMonth(yearCursor, mo);
      for (let d = 1; d <= total; d += 1) {
        out.push(entryFor(iso(yearCursor, mo, d)));
      }
    }
    return out;
  });

  /** Recent-view days, already zero-filled by the backend. */
  let recentDays = $derived(recent);

  /** The active day list, used for the summary and the intensity scale. */
  let activePoints = $derived<TrendPoint[]>(
    scope === 'recent' ? recentDays : scope === 'month' ? monthDays : yearDays
  );  /** Columns for the rolling views (recent / year). */
  let scrollColumns = $derived.by<Column[]>(() => {
    if (scope === 'recent') {
      return recentDays.length ? toColumns(recentDays, weekdayOf(recentDays[0].date)) : [];
    }
    if (scope === 'year') {
      return yearDays.length ? toColumns(yearDays, weekdayOf(yearDays[0].date)) : [];
    }
    return [];
  });

  // --- Summary + intensity --------------------------------------------------

  let totalRounds = $derived(activePoints.reduce((s, p) => s + p.rounds, 0));
  let totalFocus = $derived(activePoints.reduce((s, p) => s + p.focus_mins, 0));
  let activeDays = $derived(activePoints.filter((p) => p.rounds > 0).length);
  let maxRounds = $derived(Math.max(1, ...activePoints.map((p) => p.rounds)));

  /** Intensity bucket 0–4 for a round count. */
  function level(rounds: number): number {
    if (rounds <= 0) return 0;
    const ratio = rounds / maxRounds;
    if (ratio <= 0.25) return 1;
    if (ratio <= 0.5) return 2;
    if (ratio <= 0.75) return 3;
    return 4;
  }

  /** Is a date in the future relative to the user's today? */
  function isFuture(date: string): boolean {
    return date > today;
  }

  // --- Month markers for the rolling views ---------------------------------

  function monthMarkers(columns: Column[]): { col: number; label: string }[] {
    const out: { col: number; label: string }[] = [];
    let last = '';
    columns.forEach((c, i) => {
      if (c.days.length === 0) return;
      const mo = Number(c.days[0].date.slice(5, 7));
      const label = MONTHS[mo - 1].slice(0, 3);
      if (label !== last) {
        out.push({ col: i, label });
        last = label;
      }
    });
    return out;
  }

  let markers = $derived(monthMarkers(scrollColumns));

  // --- Navigation -----------------------------------------------------------

  let canGoForward = $derived(
    scope === 'month'
      ? monthCursor.year < todayY || (monthCursor.year === todayY && monthCursor.month < todayM)
      : scope === 'year'
        ? yearCursor < todayY
        : false
  );

  function shiftMonth(delta: number) {
    let { year, month } = monthCursor;
    month += delta;
    if (month < 1) {
      month = 12;
      year -= 1;
    } else if (month > 12) {
      month = 1;
      year += 1;
    }
    monthCursor = { year, month };
  }

  let hovered = $state<TrendPoint | null>(null);

  let hasAnyHistory = $derived(daily.length > 0);
</script>

<div class="heatmap">
  <!-- Header: scope toggle + period navigation + summary -->
  <div class="head">
    <div class="head-left">
      <div class="toggle">
        <button
          class="toggle-btn"
          class:active={scope === 'recent'}
          onclick={() => (scope = 'recent')}>{m.better_scope_recent()}</button
        >
        <button
          class="toggle-btn"
          class:active={scope === 'month'}
          onclick={() => (scope = 'month')}>{m.better_scope_month()}</button
        >
        <button
          class="toggle-btn"
          class:active={scope === 'year'}
          onclick={() => (scope = 'year')}>{m.better_scope_year()}</button
        >
      </div>

      {#if scope !== 'recent'}
        <div class="nav">
          <button
            class="nav-btn"
            aria-label={scope === 'month' ? m.better_prev_month() : m.better_prev_year()}
            onclick={() => (scope === 'month' ? shiftMonth(-1) : (yearCursor -= 1))}
          >
            ‹
          </button>
          <span class="nav-label">
            {scope === 'month' ? `${MONTHS[monthCursor.month - 1]} ${monthCursor.year}` : yearCursor}
          </span>
          <button
            class="nav-btn"
            aria-label={scope === 'month' ? m.better_next_month() : m.better_next_year()}
            onclick={() => (scope === 'month' ? shiftMonth(1) : (yearCursor += 1))}
            disabled={!canGoForward}
          >
            ›
          </button>
        </div>
      {/if}
    </div>

    <div class="legend">
      <span>{m.stats_legend_less()}</span>
      {#each [0, 1, 2, 3, 4] as lvl}
        <span class="legend-cell" data-level={lvl}></span>
      {/each}
      <span>{m.stats_legend_more()}</span>
    </div>
  </div>

  <div class="summary">
    <span><b>{totalRounds}</b> {m.better_rounds().toLowerCase()}</span>
    <span class="sep">·</span>
    <span><b>{fmtMins(totalFocus)}</b> {m.better_focus_time().toLowerCase()}</span>
    <span class="sep">·</span>
    <span><b>{activeDays}</b>/{activePoints.length} {m.better_active_days().toLowerCase()}</span>
    {#if !hasAnyHistory}
      <span class="sep">·</span>
      <span class="muted">{m.better_no_data()}</span>
    {/if}
  </div>

  <!-- Calendar layout for a single month -->
  {#if scope === 'month'}
    <div class="calendar">
      <div class="cal-row cal-head">
        {#each ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'] as w}
          <span class="cal-head-cell">{w}</span>
        {/each}
      </div>
      {#each Array(Math.ceil((weekdayOf(monthDays[0]?.date ?? today) + monthDays.length) / 7)) as _, week}
        <div class="cal-row">
          {#each Array(7) as _, wd}
            {@const idx = week * 7 + wd - weekdayOf(monthDays[0]?.date ?? today)}
            {#if idx >= 0 && idx < monthDays.length}
              {@const p = monthDays[idx]}
              <button
                type="button"
                class="cal-cell"
                class:future={isFuture(p.date)}
                class:today={p.date === today}
                data-level={isFuture(p.date) ? 0 : level(p.rounds)}
                aria-label="{fmtDay(p.date)}: {p.rounds} {m.better_rounds().toLowerCase()}, {fmtMins(p.focus_mins)}"
                onmouseenter={() => (hovered = p)}
                onfocus={() => (hovered = p)}
                onblur={() => (hovered = null)}
              >
                <span class="cal-day">{Number(p.date.slice(8, 10))}</span>
                {#if p.rounds > 0}
                  <span class="cal-count">{p.rounds}</span>
                {/if}
              </button>
            {:else}
              <span class="cal-cell cal-cell--void"></span>
            {/if}
          {/each}
        </div>
      {/each}
    </div>

    <!-- Rolling contribution grid for recent / year -->
  {:else}
    <div class="grid-wrap">
      <div class="weekday-col">
        {#each WEEKDAY_LABELS as w}
          <span class="weekday-label">{w}</span>
        {/each}
      </div>

      <div class="cols" role="group" aria-label="Focus activity" onmouseleave={() => (hovered = null)}>
        {#each scrollColumns as col, ci (ci)}
          <div class="col">
            {#each Array(col.lead) as _}
              <span class="cell cell--void"></span>
            {/each}
            {#each col.days as p (p.date)}
              <button
                type="button"
                class="cell"
                class:future={isFuture(p.date)}
                data-level={isFuture(p.date) ? 0 : level(p.rounds)}
                aria-label="{fmtDay(p.date)}: {p.rounds} {m.better_rounds().toLowerCase()}, {fmtMins(p.focus_mins)}"
                onmouseenter={() => (hovered = p)}
                onfocus={() => (hovered = p)}
                onblur={() => (hovered = null)}
              ></button>
            {/each}
          </div>
        {/each}

        <div class="months">
          {#each markers as { col, label } (col)}
            <span class="month" style="left: {col * (CELL + GAP)}px">{label}</span>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <div class="readout" class:empty={hovered === null}>
    {#if hovered}
      <span class="readout-date">{fmtDay(hovered.date)}</span>
      <span class="readout-value"><b>{hovered.rounds}</b> {m.better_rounds().toLowerCase()}</span>
      <span class="readout-value">{fmtMins(hovered.focus_mins)}</span>
    {:else}
      <span class="readout-hint">{m.better_heatmap_hint()}</span>
    {/if}
  </div>
</div>

<style>
  .heatmap {
    display: flex;
    flex-direction: column;
    gap: 9px;
    padding: 12px 14px;
    border-radius: 6px;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 10%, transparent);
    background: color-mix(in oklch, var(--color-foreground) 4%, var(--color-background));
  }

  /* ── Header ────────────────────────────────────────────── */
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }

  .head-left {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }

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
    font-size: 0.62rem;
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

  .nav {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .nav-btn {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 14%, transparent);
    border-radius: 3px;
    background: none;
    color: var(--color-foreground);
    font-size: 0.8rem;
    line-height: 1;
    cursor: pointer;
    padding: 0;
  }

  .nav-btn:hover:not(:disabled) {
    background: var(--color-hover);
    border-color: var(--color-focus-round);
    color: var(--color-focus-round);
  }

  .nav-btn:disabled {
    opacity: 0.35;
    cursor: default;
  }

  .nav-label {
    min-width: 92px;
    text-align: center;
    font-size: 0.68rem;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground);
  }

  .legend {
    display: flex;
    align-items: center;
    gap: 3px;
    font-size: 0.56rem;
    color: var(--color-foreground-darker);
  }

  .legend-cell {
    width: 9px;
    height: 9px;
    border-radius: 2px;
  }

  .summary {
    display: flex;
    align-items: baseline;
    flex-wrap: wrap;
    gap: 6px;
    font-size: 0.68rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 88%, transparent);
  }

  .summary b {
    color: var(--color-foreground);
    font-variant-numeric: tabular-nums;
  }

  .sep {
    color: color-mix(in oklch, var(--color-foreground) 25%, transparent);
  }

  .muted {
    font-style: italic;
  }

  /* ── Rolling grid ──────────────────────────────────────── */
  .grid-wrap {
    display: flex;
    gap: 5px;
    overflow-x: auto;
    padding-bottom: 16px;
  }

  .weekday-col {
    display: flex;
    flex-direction: column;
    gap: 3px;
    flex-shrink: 0;
  }

  .weekday-label {
    height: 11px;
    line-height: 11px;
    font-size: 8px;
    color: var(--color-foreground-darker);
    white-space: nowrap;
  }

  .cols {
    position: relative;
    display: flex;
    gap: 3px;
  }

  .col {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .cell {
    width: 11px;
    height: 11px;
    border-radius: 2px;
    padding: 0;
    appearance: none;
    border: none;
    cursor: default;
  }

  .cell:focus-visible,
  .cal-cell:focus-visible {
    outline: 2px solid var(--color-focus-round);
    outline-offset: 1px;
  }

  .cell--void {
    background: transparent;
  }

  .cell.future {
    background: transparent;
    border: 1px dashed color-mix(in oklch, var(--color-foreground) 12%, transparent);
  }

  .months {
    position: absolute;
    left: 0;
    bottom: -14px;
    height: 12px;
    width: 100%;
  }

  .month {
    position: absolute;
    font-size: 8px;
    color: var(--color-foreground-darker);
    white-space: nowrap;
  }

  /* ── Calendar (month view) ─────────────────────────────── */
  .calendar {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .cal-row {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    gap: 3px;
  }

  .cal-head {
    margin-bottom: 1px;
  }

  .cal-head-cell {
    font-size: 0.56rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    text-align: center;
    padding-bottom: 2px;
  }

  .cal-cell {
    position: relative;
    aspect-ratio: 1 / 1;
    max-height: 44px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1px;
    border-radius: 3px;
    padding: 0;
    appearance: none;
    border: none;
    cursor: default;
    font: inherit;
    color: inherit;
  }

  .cal-day {
    font-size: 0.6rem;
    font-variant-numeric: tabular-nums;
    color: color-mix(in oklch, var(--color-foreground) 72%, transparent);
    line-height: 1;
  }

  .cal-count {
    font-size: 0.62rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--color-background);
    line-height: 1;
  }

  .cal-cell--void {
    background: transparent;
  }

  .cal-cell.future {
    background: transparent;
    border: 1px dashed color-mix(in oklch, var(--color-foreground) 10%, transparent);
  }

  .cal-cell.future .cal-day {
    opacity: 0.3;
  }

  .cal-cell.today {
    box-shadow: inset 0 0 0 1px var(--color-focus-round);
  }

  /* ── Intensity ramp ────────────────────────────────────── */
  .cell[data-level='0'],
  .cal-cell[data-level='0'],
  .legend-cell[data-level='0'] {
    background: color-mix(in oklch, var(--color-foreground) 8%, var(--color-background));
  }

  .cell[data-level='1'],
  .cal-cell[data-level='1'],
  .legend-cell[data-level='1'] {
    background: color-mix(in oklch, var(--color-focus-round) 25%, var(--color-background));
  }

  .cell[data-level='2'],
  .cal-cell[data-level='2'],
  .legend-cell[data-level='2'] {
    background: color-mix(in oklch, var(--color-focus-round) 48%, var(--color-background));
  }

  .cell[data-level='3'],
  .cal-cell[data-level='3'],
  .legend-cell[data-level='3'] {
    background: color-mix(in oklch, var(--color-focus-round) 74%, var(--color-background));
  }

  .cell[data-level='4'],
  .cal-cell[data-level='4'],
  .legend-cell[data-level='4'] {
    background: var(--color-focus-round);
  }

  /* Day numbers over the darkest cells need light text. */
  .cal-cell[data-level='3'] .cal-day,
  .cal-cell[data-level='4'] .cal-day {
    color: color-mix(in oklch, var(--color-background) 78%, transparent);
  }

  /* ── Readout ───────────────────────────────────────────── */
  .readout {
    display: flex;
    align-items: baseline;
    gap: 10px;
    min-height: 16px;
    font-size: 0.68rem;
    color: var(--color-foreground);
  }

  .readout-date {
    font-weight: 700;
    color: var(--color-foreground-darker);
  }

  .readout-value {
    font-variant-numeric: tabular-nums;
  }

  .readout-value b {
    color: var(--color-focus-round);
  }

  .readout-hint {
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
  }
</style>
