<script lang="ts">
  import AchievementBadge from '$lib/components/AchievementBadge.svelte';
  import type { AchievementView } from '$lib/types';

  let { achievement, delay = 0 }: { achievement: AchievementView; delay?: number } = $props();

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

<div class="card" class:earned={achievement.earned} class:secret={isSecret} style="--delay: {delay}ms">
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
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border-radius: 8px;
    background: var(--color-background-light);
    animation: card-rise 0.35s cubic-bezier(0.22, 1, 0.36, 1) both;
    animation-delay: var(--delay, 0ms);
  }

  @keyframes card-rise {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
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
