<script lang="ts">
  /**
   * Consistency block: the 120-day heatmap, habit-strength figures, the
   * all-time focus window and the round-length mix.
   */
  import type { Insights } from '$lib/types';
  import { fmtMins, fmtAvg, hourLabel } from '$lib/utils/statsFormat';
  import FocusHeatmap from './charts/FocusHeatmap.svelte';
  import Donut from './charts/Donut.svelte';
  import HBarList from './charts/HBarList.svelte';
  import MetaTile from './MetaTile.svelte';
  import SectionBlock from './SectionBlock.svelte';
  import * as m from '$paraglide/messages.js';

  let { insights }: { insights: Insights } = $props();

  let c = $derived(insights.consistency);

  /** Day-mix donut slices. */
  let mixSlices = $derived([
    {
      label: m.better_consistency_strong(),
      value: c.strong_days,
      color: 'var(--color-focus-round)',
    },
    {
      label: m.better_consistency_light(),
      value: c.light_days,
      color: 'color-mix(in oklch, var(--color-focus-round) 45%, var(--color-background))',
    },
    {
      label: m.better_consistency_rest(),
      value: c.rest_days,
      color: 'color-mix(in oklch, var(--color-foreground) 16%, var(--color-background))',
    },
  ]);

  /** Round-length histogram, formatted for the ranked bar list. */
  let bucketRows = $derived(
    insights.session_buckets.map((b) => ({
      label:
        b.max_mins === null
          ? m.better_length_over({ n: b.min_mins })
          : `${b.min_mins}–${b.max_mins}m`,
      value: b.rounds,
      display: `${b.rounds}`,
      sub: fmtMins(b.focus_mins),
      color: 'var(--color-long-round)',
    }))
  );

  let totalBucketRounds = $derived(
    insights.session_buckets.reduce((s, b) => s + b.rounds, 0)
  );

  /** The focus window, if any history exists. */
  let hasWindow = $derived(
    insights.active_from_hour !== null && insights.active_to_hour !== null
  );
</script>

<SectionBlock title={m.better_heatmap_title()} hint={m.better_heatmap_sub()}>
  <FocusHeatmap recent={insights.heatmap} daily={insights.daily_all} today={insights.today} />

  <div class="panels">
    <!-- Habit strength -->
    <div class="panel panel--wide">
      <span class="panel-title">{m.better_consistency_title()}</span>

      <div class="panel-body">
        <Donut
          slices={mixSlices}
          centerValue="{Math.round(c.active_ratio * 100)}%"
          centerCaption="{c.active_days}/{c.window_days}d"
          size={116}
          thickness={14}
        />

        <div class="tiles">
          <MetaTile
            label={m.better_consistency_avg()}
            value={fmtAvg(c.avg_rounds_per_day)}
            sub={fmtMins(c.avg_focus_mins_per_day)}
          />
          <MetaTile
            label={m.better_consistency_steady()}
            value={`±${fmtAvg(c.stddev_rounds)}`}
            sub={m.better_consistency_steady_hint()}
          />
          <MetaTile label={m.better_consistency_run()} value={`${c.best_streak_in_window}`} />
          {#if hasWindow}
            <MetaTile
              label={m.better_span_from()}
              value={hourLabel(insights.active_from_hour ?? 0)}
              tone="accent"
            />
            <MetaTile
              label={m.better_span_to()}
              value={hourLabel(insights.active_to_hour ?? 0)}
              tone="accent"
            />
          {/if}
          <MetaTile
            label={m.better_span_concentration()}
            value={insights.top_weekday_share === null
              ? '—'
              : `${Math.round(insights.top_weekday_share * 100)}%`}
            sub={m.better_span_concentration_hint()}
          />
        </div>
      </div>
    </div>

    <!-- Round-length mix -->
    <div class="panel">
      <span class="panel-title">{m.better_length_title()}</span>
      {#if totalBucketRounds > 0}
        <HBarList rows={bucketRows} />
      {:else}
        <span class="empty">{m.better_no_data()}</span>
      {/if}
    </div>
  </div>
</SectionBlock>

<style>
  .panels {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 10px;
  }

  .panel--wide {
    grid-column: span 2;
  }

  @media (max-width: 720px) {
    .panel--wide {
      grid-column: auto;
    }
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

  .panel-body {
    display: flex;
    align-items: center;
    gap: 14px;
    flex-wrap: wrap;
  }

  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(76px, 1fr));
    gap: 6px;
    flex: 1 1 150px;
    min-width: 0;
  }

  .empty {
    font-size: 0.68rem;
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
    padding: 8px 0;
  }
</style>
