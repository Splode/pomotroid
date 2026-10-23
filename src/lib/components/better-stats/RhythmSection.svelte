<script lang="ts">
  /**
   * Rhythm block: the weekday × hour grid plus ranked focus-by-weekday and
   * focus-by-hour breakdowns.
   */
  import type { Insights } from '$lib/types';
  import { fmtMins, hourLabel, weekdayLabel } from '$lib/utils/statsFormat';
  import RhythmGrid from './charts/RhythmGrid.svelte';
  import HBarList from './charts/HBarList.svelte';
  import SectionBlock from './SectionBlock.svelte';
  import * as m from '$paraglide/messages.js';

  let { insights }: { insights: Insights } = $props();

  /** All-time focus minutes per weekday, strongest first. */
  let weekdayRows = $derived(
    [...insights.weekday_profile]
      .sort((a, b) => b.focus_mins - a.focus_mins)
      .map((w) => ({
        label: weekdayLabel(w.weekday),
        value: w.focus_mins,
        display: fmtMins(w.focus_mins),
        sub: `${w.rounds} ${m.better_rounds().toLowerCase()}`,
        color: 'var(--color-focus-round)',
      }))
  );

  /**
   * Focus minutes per hour. Only hours with activity are listed, strongest
   * first, so the panel stays readable instead of listing 24 near-empty rows.
   */
  let hourRows = $derived(
    insights.hour_profile
      .filter((h) => h.focus_mins > 0)
      .sort((a, b) => b.focus_mins - a.focus_mins)
      .slice(0, 8)
      .map((h) => ({
        label: hourLabel(h.hour),
        value: h.focus_mins,
        display: fmtMins(h.focus_mins),
        sub: `${h.rounds} ${m.better_rounds().toLowerCase()}`,
        color: 'var(--color-long-round)',
      }))
  );

  let hasData = $derived(insights.records.total_rounds > 0);
</script>

<SectionBlock title={m.better_section_rhythm()}>
  {#if hasData}
    <RhythmGrid cells={insights.rhythm_grid} />

    <div class="split">
      <div class="panel">
        <span class="panel-title">{m.better_weekday_title()}</span>
        <HBarList rows={weekdayRows} />
      </div>

      <div class="panel">
        <span class="panel-title">{m.better_hour_title()}</span>
        {#if hourRows.length}
          <HBarList rows={hourRows} />
        {:else}
          <span class="empty">{m.better_no_data()}</span>
        {/if}
      </div>
    </div>
  {:else}
    <span class="empty">{m.better_no_data()}</span>
  {/if}
</SectionBlock>

<style>
  .split {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 10px;
  }

  .panel {
    display: flex;
    flex-direction: column;
    gap: 9px;
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

  .empty {
    font-size: 0.68rem;
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
    padding: 8px 0;
  }
</style>
