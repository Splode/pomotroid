<script lang="ts">
  import AchievementBadge from '$lib/components/AchievementBadge.svelte';
  import type { AchievementView } from '$lib/types';

  let { achievement, delay = 0, highlighted = false, instant = false }: {
    achievement: AchievementView;
    delay?: number;
    highlighted?: boolean;
    instant?: boolean;
  } = $props();

  const isSecret = $derived(achievement.secret && !achievement.earned);
  const showProgress = $derived(
    !achievement.earned &&
    achievement.category === 'Milestone' &&
    achievement.progress_total !== null &&
    (achievement.progress_current ?? 0) < (achievement.progress_total ?? 0),
  );

  function fmtDate(ts: number | null): string {
    if (ts === null) return '';
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<div
  class="card"
  class:earned={achievement.earned}
  class:secret={isSecret}
  class:highlighted
  class:instant
  data-achievement-id={achievement.id}
  style="--delay: {delay}ms; --achievement-color: {achievement.color ?? 'transparent'}"
>
  <div class="badge-wrap">
    <AchievementBadge
      color={achievement.color}
      emoji={isSecret ? '' : achievement.emoji}
      earned={achievement.earned}
      locked={isSecret}
    />
  </div>

  <div class="info">
    <span class="name">{isSecret ? '???' : (achievement.name ?? '???')}</span>

    {#if !isSecret && achievement.description}
      <span class="desc">{achievement.description}</span>
    {/if}

    {#if achievement.earned && achievement.unlocked_at !== null}
      <span class="date">{fmtDate(achievement.unlocked_at)}</span>
    {:else if showProgress}
      <span class="progress">
        {achievement.progress_current ?? 0} / {achievement.progress_total}
      </span>
    {/if}
  </div>
</div>

<style>
  .card {
    position: relative;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border-radius: 8px;
    background: var(--color-background-light);
    animation: card-rise 0.35s cubic-bezier(0.22, 1, 0.36, 1) both;
    animation-delay: var(--delay, 0ms);
    opacity: 0.55;
  }

  .card.instant {
    animation: none;
  }

  .card.earned {
    opacity: 1;
    border-left: 3px solid var(--achievement-color, transparent);
    padding-left: 9px; /* compensate for border */
  }

  .card.highlighted::after {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 8px;
    pointer-events: none;
    animation: highlight-ring 2s ease-out both;
    animation-delay: calc(var(--delay, 0ms) + 200ms);
  }

  @keyframes card-rise {
    from { opacity: 0; transform: translateY(8px); }
    to   { transform: translateY(0); }
  }

  @keyframes highlight-ring {
    0%   { box-shadow: 0 0 0 3px color-mix(in oklch, var(--achievement-color, var(--color-focus-round)) 90%, transparent); }
    50%  { box-shadow: 0 0 8px 3px color-mix(in oklch, var(--achievement-color, var(--color-focus-round)) 70%, transparent); }
    100% { box-shadow: 0 0 0 2px color-mix(in oklch, var(--achievement-color, var(--color-focus-round)) 30%, transparent); }
  }

  .badge-wrap {
    flex-shrink: 0;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .name {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--color-foreground);
    line-height: 1.2;
  }

  .desc {
    font-size: 0.72rem;
    color: var(--color-foreground-darker);
    line-height: 1.4;
  }

  .date {
    font-size: 0.68rem;
    color: var(--color-focus-round);
    margin-top: 2px;
  }

  .progress {
    font-size: 0.68rem;
    color: var(--color-foreground-darker);
    margin-top: 2px;
    font-variant-numeric: tabular-nums;
  }
</style>
