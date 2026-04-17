<script lang="ts">
  // Displays the remaining time (MM:SS).
  import type { TimerState } from '$lib/types';

  interface Props {
    state: TimerState;
  }

  let { state }: Props = $props();

  let remaining = $derived(state.total_secs - state.elapsed_secs);
  let minutes = $derived(Math.floor(remaining / 60));
  let seconds = $derived(remaining % 60);
  let display = $derived(`${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`);
</script>

<div class="display">
  <span class="time">{display}</span>
</div>

<style>
  .display {
    /* Fill the dial-stack and flex-center the time. */
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .time {
    font-family: 'Mona Sans Mono', monospace;
    font-size: 2.8rem;
    font-weight: 300;
    font-stretch: 85%;
    letter-spacing: -0.02em;
    color: var(--color-foreground);
  }
</style>
