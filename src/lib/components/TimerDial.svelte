<script lang="ts">
  // SVG arc dial showing timer progress.
  // Replicates the original Pomotroid dial: fills from 0% to 100% as time elapses.
  // Uses Svelte tweened store for smooth animation.
  import { tweened } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';
  import type { TimerState } from '$lib/types';

  interface Props {
    snap: TimerState;
  }

  let { snap }: Props = $props();

  // SVG constants (matching original Pomotroid geometry)
  const CIRCUMFERENCE = 691.15;  // 2π × 110 ≈ 691.15

  // Tweened offset: starts at full circumference (invisible), animates toward 0 (full arc).
  const dashOffset = tweened(CIRCUMFERENCE, { duration: 800, easing: cubicOut });

  // Round-type → CSS custom property for stroke color.
  function strokeColor(rt: string): string {
    if (rt === 'work') return 'var(--color-focus-round)';
    if (rt === 'short-break') return 'var(--color-short-round)';
    return 'var(--color-long-round)';
  }

  // Track previous round to detect round changes and snap the animation.
  // Not reactive — only used for comparison inside $effect.
  let prevRound = $state<string>('');

  $effect(() => {
    const rt = snap.round_type;
    const progress = snap.total_secs > 0
      ? snap.elapsed_secs / snap.total_secs
      : 0;
    const target = CIRCUMFERENCE * (1 - progress);

    // On round change: snap to 0% fill immediately (new round starts from empty).
    if (rt !== prevRound) {
      dashOffset.set(CIRCUMFERENCE, { duration: 0 });
      prevRound = rt;
    } else {
      dashOffset.set(target);
    }
  });
</script>

<svg class="dial" viewBox="0 0 230 230" aria-hidden="true">
  <!-- Background track -->
  <path
    class="track"
    d="M115,5c60.8,0,110,49.2,110,110s-49.2,110-110,110S5,175.8,5,115S54.2,5,115,5"
    fill="none"
    stroke="var(--color-background-light)"
    stroke-width="2"
  />
  <!-- Progress arc -->
  <path
    class="progress"
    d="M115,5c60.8,0,110,49.2,110,110s-49.2,110-110,110S5,175.8,5,115S54.2,5,115,5"
    fill="none"
    stroke={strokeColor(snap.round_type)}
    stroke-width="10"
    stroke-linecap="round"
    stroke-dasharray={CIRCUMFERENCE}
    stroke-dashoffset={$dashOffset}
  />
</svg>

<style>
  .dial {
    width: 220px;
    height: 220px;
    display: block;
  }
</style>
