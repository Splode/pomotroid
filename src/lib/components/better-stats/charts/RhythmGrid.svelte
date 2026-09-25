<script lang="ts">
  /**
   * Weekday × hour activity grid — the "when do I actually focus" view.
   *
   * Rows are weekdays (Monday first), columns are hours of the day. Cell
   * intensity maps to rounds logged in that slot across all history.
   */
  import type { RhythmCell } from '$lib/types';
  import { fmtMins, hourLabel, weekdayLabel } from '$lib/utils/statsFormat';
  import * as m from '$paraglide/messages.js';

  interface Props {
    cells: RhythmCell[];
    /** Hours to exclude at each end of the day (quiet hours). */
    trimStart?: number;
    trimEnd?: number;
  }

  let { cells, trimStart = 5, trimEnd = 23 }: Props = $props();

  let hours = $derived(
    Array.from({ length: Math.max(1, trimEnd - trimStart + 1) }, (_, i) => i + trimStart)
  );

  let maxRounds = $derived(Math.max(1, ...cells.map((c) => c.rounds)));

  function cellFor(weekday: number, hour: number): RhythmCell | undefined {
    return cells.find((c) => c.weekday === weekday && c.hour === hour);
  }

  /** Intensity bucket 0–4, matching the legend. */
  function level(rounds: number): number {
    if (rounds <= 0) return 0;
    const ratio = rounds / maxRounds;
    if (ratio <= 0.2) return 1;
    if (ratio <= 0.45) return 2;
    if (ratio <= 0.7) return 3;
    return 4;
  }

  let hovered = $state<RhythmCell | null>(null);

  /** The single busiest slot, called out under the grid. */
  let peak = $derived(
    cells.reduce<RhythmCell | null>((best, c) => (!best || c.rounds > best.rounds ? c : best), null)
  );
</script>

<div class="rhythm">
  <div class="head">
    <span class="head-title">{m.better_grid_title()}</span>
    {#if peak && peak.rounds > 0}
      <span class="head-peak">
        {m.better_grid_peak()}:
        <b>{weekdayLabel(peak.weekday)} {hourLabel(peak.hour)}</b>
      </span>
    {/if}
  </div>

  <div class="grid-scroll">
    <table class="grid">
      <caption class="sr-only">{m.better_grid_title()}</caption>
      <thead>
        <tr>
          <th class="corner"></th>
          {#each hours as h (h)}
            <th class="hour-head">{h % 3 === 0 ? hourLabel(h) : ''}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each [0, 1, 2, 3, 4, 5, 6] as wd (wd)}
          <tr>
            <th class="row-head">{weekdayLabel(wd)}</th>
            {#each hours as h (h)}
              {@const cell = cellFor(wd, h)}
              {@const rounds = cell?.rounds ?? 0}
              <td>
                <button
                  type="button"
                  class="cell"
                  data-level={level(rounds)}
                  aria-label="{weekdayLabel(wd)} {hourLabel(h)}: {rounds} {m.better_rounds().toLowerCase()}"
                  onmouseenter={() => (hovered = cell ?? null)}
                  onfocus={() => (hovered = cell ?? null)}
                  onblur={() => (hovered = null)}
                ></button>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="foot">
    {#if hovered}
      <span class="foot-slot">{weekdayLabel(hovered.weekday)} {hourLabel(hovered.hour)}</span>
      <span class="foot-value"><b>{hovered.rounds}</b> {m.better_rounds().toLowerCase()}</span>
      <span class="foot-value">{fmtMins(hovered.focus_mins)}</span>
    {:else}
      <span class="foot-hint">{m.better_grid_hint()}</span>
    {/if}
  </div>
</div>

<style>
  .rhythm {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 14px;
    border-radius: 6px;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 10%, transparent);
    background: color-mix(in oklch, var(--color-foreground) 4%, var(--color-background));
  }

  .head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 10px;
  }

  .head-title {
    font-size: 0.6rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .head-peak {
    font-size: 0.62rem;
    color: color-mix(in oklch, var(--color-foreground-darker) 80%, transparent);
  }

  .head-peak b {
    color: var(--color-focus-round);
  }

  .grid-scroll {
    overflow-x: auto;
  }

  .grid {
    border-collapse: separate;
    border-spacing: 2px;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .corner {
    width: 24px;
  }

  .hour-head {
    font-size: 8px;
    font-weight: 500;
    color: var(--color-foreground-darker);
    text-align: center;
    padding: 0;
    white-space: nowrap;
    min-width: 13px;
  }

  .row-head {
    font-size: 8px;
    font-weight: 500;
    color: var(--color-foreground-darker);
    text-align: right;
    padding-right: 3px;
    white-space: nowrap;
  }

  .cell {
    display: block;
    width: 13px;
    height: 13px;
    border-radius: 2px;
    padding: 0;
    appearance: none;
    border: none;
    cursor: default;
  }

  .cell:focus-visible {
    outline: 2px solid var(--color-focus-round);
    outline-offset: 1px;
  }

  .cell[data-level='0'] {
    background: color-mix(in oklch, var(--color-foreground) 7%, var(--color-background));
  }
  .cell[data-level='1'] {
    background: color-mix(in oklch, var(--color-focus-round) 22%, var(--color-background));
  }
  .cell[data-level='2'] {
    background: color-mix(in oklch, var(--color-focus-round) 45%, var(--color-background));
  }
  .cell[data-level='3'] {
    background: color-mix(in oklch, var(--color-focus-round) 72%, var(--color-background));
  }
  .cell[data-level='4'] {
    background: var(--color-focus-round);
  }

  .foot {
    display: flex;
    align-items: baseline;
    gap: 10px;
    min-height: 15px;
    font-size: 0.66rem;
  }

  .foot-slot {
    font-weight: 700;
    color: var(--color-foreground-darker);
  }

  .foot-value {
    font-variant-numeric: tabular-nums;
    color: var(--color-foreground);
  }

  .foot-value b {
    color: var(--color-focus-round);
  }

  .foot-hint {
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
  }
</style>
