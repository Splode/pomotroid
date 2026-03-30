<script lang="ts">
  import { onMount } from 'svelte';
  import { achievementsGetAll } from '$lib/ipc';
  import AchievementCard from './AchievementCard.svelte';
  import type { AchievementView, AchievementCategory } from '$lib/types';
  import { error as logError } from '@tauri-apps/plugin-log';

  const CATEGORY_ORDER: AchievementCategory[] = ['Milestone', 'Habit', 'Discovery'];

  let achievements = $state<AchievementView[]>([]);
  let loading = $state(true);

  export async function refresh() {
    try {
      achievements = await achievementsGetAll();
    } catch (e) {
      await logError(`[achievements] failed to load: ${e}`);
    }
  }

  // Flat list sorted Milestone → Habit → Discovery so there is only one grid
  // and therefore at most one lone last card (when total is odd).
  const sorted = $derived(
    CATEGORY_ORDER.flatMap((cat) => achievements.filter((a) => a.category === cat)),
  );

  const totalEarned = $derived(achievements.filter((a) => a.earned).length);
  const total = $derived(achievements.length);

  onMount(async () => {
    try {
      achievements = await achievementsGetAll();
    } catch (e) {
      await logError(`[achievements] failed to load on mount: ${e}`);
    } finally {
      loading = false;
    }
  });
</script>

<div class="tab">
  {#if loading}
    <div class="empty">Loading…</div>
  {:else if achievements.length === 0}
    <div class="empty">No achievements yet.</div>
  {:else}
    <div class="summary">{totalEarned} / {total} unlocked</div>

    <div class="list">
      <div class="grid">
        {#each sorted as achievement, i (achievement.id)}
          <AchievementCard {achievement} delay={i * 40} />
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .summary {
    flex-shrink: 0;
    padding: 10px 24px 8px;
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    border-bottom: 1px solid var(--color-separator);
  }

  .list {
    flex: 1;
    overflow-y: auto;
    padding: 16px 24px 24px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .empty {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8rem;
    color: var(--color-foreground-darker);
  }
</style>
