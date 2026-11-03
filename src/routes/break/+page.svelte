<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import { tweened } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';
  import { fade, scale } from 'svelte/transition';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import {
    getSettings,
    getThemes,
    getTimerState,
    onTimerTick,
    onTimerPaused,
    onTimerResumed,
    onRoundChange,
    onTimerReset,
    timerToggle,
    timerSkip,
  } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { resolveThemeName } from '$lib/utils/theme';
  import { setLocale } from '$lib/locale.svelte.js';
  import type { TimerState } from '$lib/types';
  import * as m from '$paraglide/messages.js';
  import { info, error as logError } from '@tauri-apps/plugin-log';

  let state = $state<TimerState>({
    round_type: 'short-break',
    previous_round_type: 'work',
    elapsed_secs: 0,
    total_secs: 300,
    is_running: true,
    is_paused: false,
    work_round_number: 1,
    work_rounds_total: 4,
    session_work_count: 1,
  });

  // Geometry for the large break progress dial (radius = 130)
  const CIRCUMFERENCE = 816.81; // 2 * π * 130
  const dashOffset = tweened(CIRCUMFERENCE, { duration: 800, easing: cubicOut });

  let remaining = $derived(Math.max(0, state.total_secs - state.elapsed_secs));
  let minutes = $derived(Math.floor(remaining / 60));
  let seconds = $derived(remaining % 60);
  let displayTime = $derived(
    `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
  );

  let isLongBreak = $derived(state.round_type === 'long-break');
  let roundColor = $derived(isLongBreak ? 'var(--color-long-round)' : 'var(--color-short-round)');
  let roundTitle = $derived(
    isLongBreak ? m.break_shield_title_long() : m.break_shield_title_short()
  );

  $effect(() => {
    const progress = state.total_secs > 0 ? state.elapsed_secs / state.total_secs : 0;
    const countdown = $settings.dial_countdown;
    const target = countdown ? CIRCUMFERENCE * progress : CIRCUMFERENCE * (1 - progress);
    dashOffset.set(target);
  });

  async function dismiss() {
    try {
      await getCurrentWebviewWindow().close();
    } catch (e) {
      await logError(`[break-shield] failed to close window on dismiss: ${e}`);
    }
  }

  async function handleSkip() {
    try {
      await timerSkip();
      await getCurrentWebviewWindow().close();
    } catch (e) {
      await logError(`[break-shield] failed to skip break: ${e}`);
    }
  }

  async function handleToggle() {
    try {
      await timerToggle();
    } catch (e) {
      await logError(`[break-shield] failed to toggle timer: ${e}`);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      dismiss();
    } else if (e.key === ' ' && !(e.target instanceof HTMLButtonElement)) {
      e.preventDefault();
      handleToggle();
    }
  }

  onMount(() => {
    const cleanups: UnlistenFn[] = [];
    const win = getCurrentWebviewWindow();

    document.addEventListener('keydown', handleKeydown);
    cleanups.push(() => document.removeEventListener('keydown', handleKeydown));

    (async () => {
      try {
        const s = await getSettings();
        settings.set(s);
        setLocale(s.language);

        const themes = await getThemes();
        const osDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const activeTheme = themes.find((t) => t.name === resolveThemeName(s, osDark)) ?? themes[0];
        if (activeTheme) applyTheme(activeTheme);

        const currentSnap = await getTimerState();
        state = currentSnap;

        // If for any reason the timer is already in work mode, close immediately
        if (currentSnap.round_type === 'work') {
          await win.close();
          return;
        }

        await win.setFullscreen(true);
        await win.setAlwaysOnTop(true);
        await win.show();
        await win.setFocus();
        await info('[break-shield] window shown and focused');
      } catch (e) {
        await logError(`[break-shield] init failed: ${e}`);
      }

      cleanups.push(
        await onTimerTick(({ elapsed_secs, total_secs }) => {
          state = {
            ...state,
            elapsed_secs,
            total_secs,
            is_running: true,
            is_paused: false,
          };
        }),
        await onTimerPaused(({ elapsed_secs }) => {
          state = {
            ...state,
            elapsed_secs,
            is_running: false,
            is_paused: true,
          };
        }),
        await onTimerResumed(({ elapsed_secs }) => {
          state = {
            ...state,
            elapsed_secs,
            is_running: true,
            is_paused: false,
          };
        }),
        await onRoundChange(async (snap) => {
          state = snap;
          // When break finishes and transitions to work, close window automatically
          if (snap.round_type === 'work') {
            await win.close();
          }
        }),
        await onTimerReset(async () => {
          await win.close();
        })
      );
    })();

    return () => {
      for (const fn of cleanups) fn();
    };
  });
</script>

<div class="shield-container" style="--round-color: {roundColor}">
  <!-- Ambient background glow aura -->
  <div class="ambient-glow" aria-hidden="true"></div>

  <!-- Content card -->
  <div class="shield-content" in:scale={{ duration: 350, start: 0.95, easing: cubicOut }}>
    <!-- Header badge -->
    <header class="badge-wrapper">
      <div class="round-badge">
        <span class="pulse-dot"></span>
        <span class="badge-text">{roundTitle}</span>
      </div>
    </header>

    <!-- Center Timer Dial -->
    <main class="dial-container">
      <svg class="dial-svg" viewBox="0 0 300 300" aria-hidden="true">
        <!-- Background track -->
        <circle
          cx="150"
          cy="150"
          r="130"
          fill="none"
          stroke="var(--color-background-light)"
          stroke-width="4"
          opacity="0.5"
        />
        <!-- Progress arc -->
        <circle
          cx="150"
          cy="150"
          r="130"
          fill="none"
          stroke="var(--round-color)"
          stroke-width="12"
          stroke-linecap="round"
          stroke-dasharray={CIRCUMFERENCE}
          stroke-dashoffset={$dashOffset}
          transform="rotate(-90 150 150)"
        />
      </svg>

      <!-- Center time display -->
      <div class="time-overlay">
        <span class="time-text">{displayTime}</span>
        {#if state.is_paused}
          <span class="paused-pill" in:fade={{ duration: 150 }}>PAUSED</span>
        {/if}
      </div>
    </main>

    <!-- Subtitle / Mindfulness prompt -->
    <p class="subtitle">{m.break_shield_subtitle()}</p>

    <!-- Controls -->
    <footer class="controls-row">
      <!-- Play / Pause -->
      <button
        class="ctrl-btn ctrl-btn--icon"
        onclick={handleToggle}
        aria-label={state.is_running ? 'Pause' : 'Resume'}
        title={state.is_running ? 'Pause' : 'Resume'}
      >
        {#if state.is_running}
          <svg width="22" height="22" viewBox="0 0 24 24">
            <rect x="5" y="3" width="5" height="18" rx="1.5" fill="currentColor" />
            <rect x="14" y="3" width="5" height="18" rx="1.5" fill="currentColor" />
          </svg>
        {:else}
          <svg width="22" height="22" viewBox="0 0 24 24" style="margin-left: 2px;">
            <polygon points="5,3 21,12 5,21" fill="currentColor" />
          </svg>
        {/if}
      </button>

      <!-- Unlock / Dismiss screen -->
      <button
        class="ctrl-btn ctrl-btn--primary"
        onclick={dismiss}
        aria-label={m.break_shield_btn_dismiss()}
      >
        <svg
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
          <path d="M7 11V7a5 5 0 0 1 9.9-1" />
        </svg>
        <span>{m.break_shield_btn_dismiss()}</span>
        <kbd class="kbd-hint">Esc</kbd>
      </button>

      <!-- Skip Break -->
      <button
        class="ctrl-btn ctrl-btn--secondary"
        onclick={handleSkip}
        aria-label={m.break_shield_btn_skip()}
      >
        <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
          <polygon points="1,1 10,8 1,15" />
          <rect x="12" y="1" width="3" height="14" rx="1" />
        </svg>
        <span>{m.break_shield_btn_skip()}</span>
      </button>
    </footer>
  </div>
</div>

<style>
  .shield-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
    background-color: var(--color-background);
    color: var(--color-foreground);
    user-select: none;
  }

  .ambient-glow {
    position: absolute;
    width: 700px;
    height: 700px;
    border-radius: 50%;
    background: radial-gradient(
      circle,
      color-mix(in oklch, var(--round-color) 16%, transparent) 0%,
      color-mix(in oklch, var(--round-color) 4%, transparent) 45%,
      transparent 70%
    );
    pointer-events: none;
    animation: breathing 6s ease-in-out infinite alternate;
  }

  @keyframes breathing {
    0% {
      transform: scale(0.92);
      opacity: 0.7;
    }
    100% {
      transform: scale(1.12);
      opacity: 1;
    }
  }

  .shield-content {
    position: relative;
    z-index: 2;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 28px;
    max-width: 640px;
    padding: 32px;
    text-align: center;
  }

  .badge-wrapper {
    display: flex;
    justify-content: center;
  }

  .round-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 16px;
    border-radius: 9999px;
    background: color-mix(in oklch, var(--round-color) 12%, transparent);
    border: 1px solid color-mix(in oklch, var(--round-color) 25%, transparent);
    color: var(--round-color);
  }

  .pulse-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--round-color);
    box-shadow: 0 0 10px var(--round-color);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(0.85);
    }
  }

  .badge-text {
    font-size: 0.85rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .dial-container {
    position: relative;
    width: 320px;
    height: 320px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dial-svg {
    width: 100%;
    height: 100%;
    transform: rotate(0deg);
  }

  .time-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .time-text {
    font-family: 'Mona Sans Mono', monospace;
    font-size: 4.8rem;
    font-weight: 250;
    font-stretch: 85%;
    letter-spacing: -0.03em;
    color: var(--color-foreground);
    line-height: 1;
  }

  .paused-pill {
    margin-top: 10px;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.14em;
    padding: 3px 10px;
    border-radius: 4px;
    background: var(--color-hover);
    color: var(--color-foreground-darker);
    border: 1px solid var(--color-separator);
  }

  .subtitle {
    font-size: 1.05rem;
    font-weight: 400;
    color: var(--color-foreground-darker, #a3aec4);
    line-height: 1.5;
    max-width: 440px;
    margin: 0;
  }

  .controls-row {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-top: 8px;
  }

  .ctrl-btn {
    font-family: inherit;
    font-size: 0.9rem;
    font-weight: 500;
    display: inline-flex;
    align-items: center;
    gap: 10px;
    border-radius: 9999px;
    border: 1px solid transparent;
    cursor: pointer;
    transition: all var(--transition-default);
    user-select: none;
    outline: none;
  }

  .ctrl-btn--icon {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    justify-content: center;
    background: var(--color-hover);
    border-color: var(--color-separator);
    color: var(--color-foreground);
  }

  .ctrl-btn--icon:hover {
    background: color-mix(in oklch, var(--color-foreground) 16%, transparent);
    color: var(--round-color);
    border-color: var(--round-color);
  }

  .ctrl-btn--primary {
    padding: 13px 26px;
    background: color-mix(in oklch, var(--color-foreground) 10%, transparent);
    border-color: var(--color-separator);
    color: var(--color-foreground);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  }

  .ctrl-btn--primary:hover {
    background: color-mix(in oklch, var(--color-foreground) 18%, transparent);
    border-color: var(--color-foreground-darker);
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.22);
  }

  .ctrl-btn--primary:active {
    transform: translateY(0);
  }

  .kbd-hint {
    font-family: 'Mona Sans Mono', monospace;
    font-size: 0.72rem;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 4px;
    background: color-mix(in oklch, var(--color-foreground) 14%, transparent);
    color: var(--color-foreground-darker);
    border: 1px solid var(--color-separator);
    margin-left: 2px;
  }

  .ctrl-btn--secondary {
    padding: 13px 22px;
    background: transparent;
    border-color: var(--color-separator);
    color: var(--color-foreground-darker);
  }

  .ctrl-btn--secondary:hover {
    background: var(--color-hover);
    color: var(--color-foreground);
    border-color: var(--color-foreground-darker);
  }
</style>
