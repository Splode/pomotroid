<script lang="ts">
  // Round counter, reset/skip buttons, and volume slider.
  import type { TimerState } from '$lib/types';
  import { timerReset, setSetting } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';

  interface Props {
    snap: TimerState;
  }

  let { snap }: Props = $props();

  let showVolume = $state(false);

  // Local volume for immediate slider feedback — avoids waiting for the
  // async IPC round-trip before the thumb visually moves.
  let localVolume = $state($settings.volume);
  $effect(() => { localVolume = $settings.volume; });

  // Remembered pre-mute level so the button can restore it on unmute.
  let premuteVolume = $state<number | null>(null);

  function handleVolumeChange(e: Event) {
    const val = (e.target as HTMLInputElement).valueAsNumber;
    localVolume = val;
    setSetting('volume', String(Math.round(val * 100)));
  }

  function toggleMute() {
    if (localVolume === 0) {
      const restore = premuteVolume ?? 1.0;
      premuteVolume = null;
      localVolume = restore;
      setSetting('volume', String(Math.round(restore * 100)));
    } else {
      premuteVolume = localVolume;
      localVolume = 0;
      setSetting('volume', '0');
    }
  }
</script>

<div class="footer">
  <!-- Round counter -->
  <span class="rounds">
    {snap.work_round_number} / {snap.work_rounds_total}
  </span>

  <!-- Reset -->
  <button class="btn-text" onclick={timerReset} aria-label="Reset timer">
    Reset
  </button>

  <!-- Volume -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="volume-wrapper"
    onmouseenter={() => (showVolume = true)}
    onmouseleave={() => (showVolume = false)}
  >
    <button class="btn-icon" onclick={toggleMute} aria-label={localVolume === 0 ? 'Unmute' : 'Mute'}>
      {#if localVolume === 0}
        <svg width="16" height="16" viewBox="0 0 16 16">
          <polygon points="1,5 5,5 10,1 10,15 5,11 1,11" fill="currentColor"/>
          <line x1="12" y1="5" x2="16" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <line x1="16" y1="5" x2="12" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      {:else}
        <svg width="16" height="16" viewBox="0 0 16 16">
          <polygon points="1,5 5,5 10,1 10,15 5,11 1,11" fill="currentColor"/>
          <path d="M12,5 Q15,8 12,11" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      {/if}
    </button>

    {#if showVolume}
      <div class="volume-slider-wrapper">
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={localVolume}
          oninput={handleVolumeChange}
          class="volume-slider"
          aria-label="Volume"
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .footer {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 8px 16px;
    width: 100%;
  }

  .rounds {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    min-width: 48px;
    text-align: center;
  }

  .btn-text {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    padding: 4px 8px;
    border-radius: 4px;
    transition: color 0.15s, background 0.15s;
  }

  .btn-text:hover {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  .btn-icon {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 4px;
    transition: color 0.15s, background 0.15s;
  }

  .btn-icon:hover {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  .volume-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .volume-slider-wrapper {
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    padding: 8px;
    background: var(--color-background-light);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
    /* Fixed size to contain the rotated slider without layout overflow. */
    width: 36px;
    height: 100px;
  }

  .volume-slider {
    /* Rotate a normal horizontal slider so it appears vertical.
       Unlike writing-mode, transform preserves correct pointer-event
       mapping on WebKit: dragging up increases value, dragging down
       decreases it. */
    width: 80px;
    transform: rotate(-90deg);
    cursor: pointer;
    accent-color: var(--color-accent);
  }
</style>
