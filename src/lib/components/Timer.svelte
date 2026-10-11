<script lang="ts">
  // Orchestrator component. Subscribes to timer events, owns keyboard listener,
  // and renders TimerDial + TimerDisplay + TimerFooter.
  import { onMount } from 'svelte';
  import {
    timerToggle,
    timerRecordDistraction,
    getTimerState,
    onTimerTick,
    onTimerPaused,
    onTimerResumed,
    onRoundChange,
    onTimerReset,
    onTimerDurationChanged,
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
  import { error as logError } from '@tauri-apps/plugin-log';

  interface Props {
    isCompact?: boolean;
    uiScale?: number;
  }

  let { isCompact = false, uiScale = 1 }: Props = $props();

  let state = $derived($timerState);

  async function recordDistraction() {
    try {
      timerState.set(await timerRecordDistraction());
    } catch (error) {
      await logError(`[timer] failed to record distraction: ${error}`);
    }
  }

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

    // Async setup: hydrate state and register event listeners.
    (async () => {
      const initial = await getTimerState();
      timerState.set(initial);

      cleanups.push(
        await onTimerTick(({ elapsed_secs, total_secs }) => {
          timerState.update((s) => ({
            ...s,
            elapsed_secs,
            total_secs,
            is_running: true,
            is_paused: false,
          }));
        }),
        await onTimerPaused(({ elapsed_secs }) => {
          timerState.update((s) => ({
            ...s,
            elapsed_secs,
            is_running: false,
            is_paused: true,
          }));
        }),
        await onTimerResumed(({ elapsed_secs }) => {
          timerState.update((s) => ({
            ...s,
            elapsed_secs,
            is_running: true,
            is_paused: false,
          }));
        }),
        await onRoundChange((snap) => {
          timerState.set(snap);
          if ($settings.notifications_enabled) {
            let title: string;
            let body: string;
            if (snap.round_type === 'work') {
              const afterBreak =
                snap.previous_round_type === 'short-break' ||
                snap.previous_round_type === 'long-break';
              title = afterBreak ? m.notification_work_title() : m.notification_work_start_title();
              body = afterBreak ? m.notification_work_body() : m.notification_work_start_body();
            } else if (snap.round_type === 'short-break') {
              title = m.notification_short_break_title();
              body = m.notification_short_break_body();
            } else {
              title = m.notification_long_break_title();
              body = m.notification_long_break_body();
            }
            notificationShow(title, body).catch(() => {});
          }
        }),
        await onTimerReset((snap) => {
          timerState.set(snap);
        }),
        await onTimerDurationChanged((snap) => {
          timerState.set(snap);
        })
      );
    })();

    return () => {
      for (const unlisten of cleanups) unlisten();
    };
  });
</script>

<div class="timer-outer" class:compact={isCompact}>
  <div class="timer" style="zoom: {uiScale}">
    <TimerDisplay {state} />

    <div class="dial-stack">
      <TimerDial snap={state} countdown={$settings.dial_countdown} />
      <button
        class="distraction-counter"
        class:has-count={state.distraction_count > 0}
        onclick={recordDistraction}
        disabled={state.round_type !== 'work'}
        aria-label={m.timer_distraction_add()}
        title={m.timer_distraction_hint()}
        onkeydown={(event) => {
          if (event.key === ' ' || event.key === 'Enter') event.stopPropagation();
        }}
      >
        <span aria-live="polite" aria-atomic="true">
          {#if state.distraction_count > 0}
            <span aria-hidden="true">{state.distraction_count}</span>
            <span class="sr-only">{m.timer_distraction_count({ n: state.distraction_count })}</span>
          {:else if state.round_type === 'work'}
            <span class="distraction-prompt">{m.timer_distraction_hit()}</span>
            <span class="distraction-caption">{m.timer_distraction_when()}</span>
          {:else}
            <span class="distraction-caption">{roundLabel(state.round_type)}</span>
          {/if}
        </span>
      </button>
    </div>

    {#if !isCompact}
      <div class="round-label" style="color: {roundColor(state.round_type)}">
        {roundLabel(state.round_type)}
      </div>

      <div class="controls-wrapper">
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
                  <rect x="5" y="3" width="5" height="18" rx="1.5" fill="currentColor" />
                  <rect x="14" y="3" width="5" height="18" rx="1.5" fill="currentColor" />
                </svg>
              {:else}
                <svg width="18" height="18" viewBox="0 0 24 24" style="overflow: visible;">
                  <polygon points="4,0 28,12 4,24" fill="currentColor" />
                </svg>
              {/if}
            </span>
          {/key}
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
    gap: 10px;
    width: 300px;
  }

  .compact .timer {
    width: auto;
  }

  .dial-stack {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .distraction-counter {
    position: absolute;
    inset: 20px;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: var(--color-foreground);
    font: inherit;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background var(--transition-default);
  }

  .distraction-counter:hover:not(:disabled) {
    background: var(--color-hover);
  }

  .distraction-counter:focus-visible {
    outline: 2px solid var(--color-focus-round);
    outline-offset: 4px;
  }

  .distraction-counter:disabled {
    cursor: default;
  }

  .distraction-prompt,
  .distraction-caption {
    display: block;
  }

  .distraction-prompt {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 6px;
  }

  .distraction-caption {
    font-size: 0.85rem;
    line-height: 1.4;
    color: var(--color-foreground-darker, var(--color-foreground));
  }

  .has-count {
    font-family: 'Mona Sans Mono', monospace;
    font-size: 2.8rem;
    font-weight: 300;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }

  .controls-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .play-pause {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 50%;
    border: 2px solid var(--color-foreground-darker, var(--color-foreground));
    transition:
      color var(--transition-default),
      border-color var(--transition-default),
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
