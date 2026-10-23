<script lang="ts">
  /** All-time totals and personal bests. */
  import type { Insights } from '$lib/types';
  import { fmtMins, fmtAvg, fmtDay, fmtWeek } from '$lib/utils/statsFormat';
  import MetaTile from './MetaTile.svelte';
  import SectionBlock from './SectionBlock.svelte';
  import * as m from '$paraglide/messages.js';

  let { insights }: { insights: Insights } = $props();

  let r = $derived(insights.records);
</script>

<SectionBlock title={m.better_records_title()}>
  <div class="tiles">
    <MetaTile
      label={m.better_records_total_rounds()}
      value={`${r.total_rounds}`}
      sub={m.better_records_tracked_days() + `: ${r.tracked_days}`}
      tone="accent"
    />
    <MetaTile
      label={m.better_records_total_focus()}
      value={fmtMins(r.total_focus_mins)}
      sub={`${(r.total_focus_mins / 60).toFixed(1)}h`}
    />
    <MetaTile
      label={m.better_records_avg_session()}
      value={r.avg_session_mins === null ? '—' : fmtAvg(r.avg_session_mins, 'm')}
    />
    <MetaTile
      label={m.better_records_longest_session()}
      value={r.longest_session_mins === null ? '—' : `${r.longest_session_mins}m`}
    />
    <MetaTile
      label={m.better_records_streak()}
      value={`${insights.streak.current}`}
      sub={`${m.better_records_longest()}: ${insights.streak.longest}`}
      tone={insights.streak.at_risk ? 'warn' : 'ok'}
    />
    <MetaTile
      label={m.better_records_avg_active_day()}
      value={fmtAvg(r.avg_rounds_per_active_day)}
    />
    <MetaTile
      label={m.better_records_best_day()}
      value={r.best_day_rounds ? `${r.best_day_rounds.rounds}` : '—'}
      sub={
        r.best_day_rounds
          ? `${fmtDay(r.best_day_rounds.date)} · ${fmtMins(r.best_day_rounds.focus_mins)}`
          : null
      }
      tone="accent"
    />
    <MetaTile
      label={m.better_records_best_week()}
      value={r.best_week ? fmtMins(r.best_week.focus_mins) : '—'}
      sub={r.best_week ? `${fmtWeek(r.best_week.date)} · ${r.best_week.rounds}` : null}
    />
  </div>
</SectionBlock>

<style>
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(128px, 1fr));
    gap: 8px;
  }
</style>
