<script lang="ts">
  // Orchestrator component. Subscribes to timer events, owns keyboard listener,
  // and renders TimerDial + TimerDisplay + TimerFooter.
  import { onMount } from 'svelte';
  import {
    timerToggle,
    timerRestartRound,
    timerSkip,
    getTimerState,
    onTimerTick,
    onTimerPaused,
    onTimerResumed,
    onRoundChange,
    onTimerReset,
  } from '$lib/ipc';
  import { timerState } from '$lib/stores/timer';
  import { settings } from '$lib/stores/settings';
  import { fade } from 'svelte/transition';
  import TimerDial from './TimerDial.svelte';
  import TimerDisplay from './TimerDisplay.svelte';
  import TimerFooter from './TimerFooter.svelte';
  import MiniControls from './MiniControls.svelte';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import * as m from '$paraglide/messages.js';
  import { notificationShow } from '$lib/ipc';

  interface Props {
    isCompact?: boolean;
    uiScale?: number;
  }

  let { isCompact = false, uiScale = 1 }: Props = $props();

  let state = $derived($timerState);

  function roundColor(rt: string): string {
    if (rt === 'work') return 'var(--color-focus-round)';
    if (rt === 'short-break') return 'var(--color-short-round)';
    return 'var(--color-long-round)';
  }

  function roundLabel(rt: string): string {
    if (rt === 'work') return m.round_label_work();
    if (rt === 'short-break') return m.round_label_short_break();
    return m.round_label_long_break();
  }

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    function onKeydown(e: KeyboardEvent) {
      if (e.code === 'Space' && e.target === document.body) {
        e.preventDefault();
        timerToggle();
      }
    }
    window.addEventListener('keydown', onKeydown);

    // Async setup: hydrate state and register event listeners.
    (async () => {
      const initial = await getTimerState();
      timerState.set(initial);

      cleanups.push(
        await onTimerTick(({ elapsed_secs, total_secs }) => {
          timerState.update((s) => ({ ...s, elapsed_secs, total_secs, is_running: true, is_paused: false }));
        }),
        await onTimerPaused(({ elapsed_secs }) => {
          timerState.update((s) => ({ ...s, elapsed_secs, is_running: false, is_paused: true }));
        }),
        await onTimerResumed(({ elapsed_secs }) => {
          timerState.update((s) => ({ ...s, elapsed_secs, is_running: true, is_paused: false }));
        }),
        await onRoundChange((snap) => {
          timerState.set(snap);
          if ($settings.notifications_enabled) {
            let title: string;
            let body: string;
            if (snap.round_type === 'work') {
              title = m.notification_work_title();
              body  = m.notification_work_body();
            } else if (snap.round_type === 'short-break') {
              title = m.notification_short_break_title();
              body  = m.notification_short_break_body();
            } else {
              title = m.notification_long_break_title();
              body  = m.notification_long_break_body();
            }
            notificationShow(title, body).catch(() => {});
          }
        }),
        await onTimerReset((snap) => {
          timerState.set(snap);
        }),
      );
    })();

    return () => {
      for (const unlisten of cleanups) unlisten();
      window.removeEventListener('keydown', onKeydown);
    };
  });
</script>

<div class="timer-outer" class:compact={isCompact}>
  <div class="timer" style="zoom: {uiScale}">
    <!-- Dial + display stacked (display centered over dial) -->
    <div class="dial-stack">
      <TimerDial snap={state} countdown={$settings.dial_countdown} />
      <TimerDisplay {state} />
    </div>

    {#if !isCompact}
      <!-- Round type label sits below the dial as a normal flex child so it
           does not affect the dial-stack height used to centre TimerDisplay. -->
      <div class="round-label" style="color: {roundColor(state.round_type)}">
        {roundLabel(state.round_type)}
      </div>

      <!-- Controls row: back | play/pause | skip -->
      <div class="controls">
        <!-- Back: restart current round -->
        <button class="btn-side" onclick={timerRestartRound} aria-label="Restart round">
          <svg width="18" height="18" viewBox="0 0 16 16">
            <polygon points="15,1 6,8 15,15" fill="currentColor"/>
            <rect x="1" y="1" width="3" height="14" rx="1" fill="currentColor"/>
          </svg>
        </button>

        <!-- Play / Pause — icon fades when state changes -->
        <button
          class="play-pause"
          onclick={timerToggle}
          aria-label={state.is_running ? 'Pause' : 'Play'}
        >
          {#key state.is_running}
            <span class="icon" in:fade={{ duration: 120 }}>
              {#if state.is_running}
                <svg width="24" height="24" viewBox="0 0 24 24">
                  <rect x="4" y="3" width="5" height="18" rx="1.5" fill="currentColor"/>
                  <rect x="15" y="3" width="5" height="18" rx="1.5" fill="currentColor"/>
                </svg>
              {:else}
                <svg width="24" height="24" viewBox="0 0 24 24">
                  <polygon points="5,3 21,12 5,21" fill="currentColor"/>
                </svg>
              {/if}
            </span>
          {/key}
        </button>

        <!-- Skip: advance to next round -->
        <button class="btn-side" onclick={timerSkip} aria-label="Skip round">
          <svg width="18" height="18" viewBox="0 0 16 16">
            <polygon points="1,1 10,8 1,15" fill="currentColor"/>
            <rect x="12" y="1" width="3" height="14" rx="1" fill="currentColor"/>
          </svg>
        </button>
      </div>

      <TimerFooter snap={state} />
    {/if}
  </div>

  {#if isCompact}
    <MiniControls />
  {/if}
</div>

<style>
  .timer-outer {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .timer {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .dial-stack {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .btn-side {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 4px;
    transition: color var(--transition-default), background var(--transition-default);
  }

  .btn-side:hover {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  .play-pause {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 52px;
    height: 52px;
    border-radius: 50%;
    border: 2px solid var(--color-foreground-darker, var(--color-foreground));
    transition: color var(--transition-default), border-color var(--transition-default),
      background var(--transition-default);
    overflow: hidden; /* clip the fading icon within the circle */
  }

  .play-pause:hover {
    color: var(--color-accent);
    border-color: var(--color-accent);
    background: var(--color-hover);
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .round-label {
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    /* Collapse the gap above: the flex gap already provides spacing from the dial. */
    margin-top: -8px;
  }
</style>
