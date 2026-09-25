<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import {
    getSettings,
    getThemes,
    onSettingsChanged,
    onThemesChanged,
    onRoundChange,
    onSessionsCleared,
    statsGetInsights,
  } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { setLocale } from '$lib/locale.svelte.js';
  import { resolveThemeName } from '$lib/utils/theme';
  import { isMac } from '$lib/utils/platform';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import type { Insights } from '$lib/types';
  import { info, error as logError } from '@tauri-apps/plugin-log';
  import * as m from '$paraglide/messages.js';

  import MomentumCharts from '$lib/components/better-stats/MomentumCharts.svelte';
  import HeroSection from '$lib/components/better-stats/HeroSection.svelte';
  import ConsistencySection from '$lib/components/better-stats/ConsistencySection.svelte';
  import RhythmSection from '$lib/components/better-stats/RhythmSection.svelte';
  import RecordsSection from '$lib/components/better-stats/RecordsSection.svelte';

  let insights = $state<Insights | null>(null);
  let loading = $state(true);
  let loadError = $state<string | null>(null);

  /** True once history exists to visualise. */
  const hasData = $derived((insights?.records.total_rounds ?? 0) > 0);

  async function refresh() {
    try {
      insights = await statsGetInsights();
      loadError = null;
    } catch (e) {
      loadError = String(e);
      await logError(`[better-stats] failed to load insights: ${e}`);
    } finally {
      loading = false;
    }
  }

  function close() {
    getCurrentWebviewWindow().close();
  }

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    (async () => {
      try {
        const s = await getSettings();
        settings.set(s);
        setLocale(s.language);

        const themes = await getThemes();
        const osDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const activeTheme = themes.find((t) => t.name === resolveThemeName(s, osDark)) ?? themes[0];
        if (activeTheme) applyTheme(activeTheme);

        // Show the window only once the theme is applied, to avoid a flash.
        await getCurrentWebviewWindow().show();

        await refresh();
        await info('[better-stats] initialized');
      } catch (e) {
        await logError(`[better-stats] initialization failed: ${e}`);
        loadError = String(e);
        loading = false;
      }

      cleanups.push(
        await onRoundChange(() => void refresh()),
        await onSessionsCleared(() => void refresh()),
        await onSettingsChanged(async (updated) => {
          const prev = {
            mode: $settings.theme_mode,
            light: $settings.theme_light,
            dark: $settings.theme_dark,
            language: $settings.language,
          };
          settings.set(updated);
          if (updated.language !== prev.language) setLocale(updated.language);
          if (
            updated.theme_mode !== prev.mode ||
            updated.theme_light !== prev.light ||
            updated.theme_dark !== prev.dark
          ) {
            const allThemes = await getThemes();
            const dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const t = allThemes.find((th) => th.name === resolveThemeName(updated, dark));
            if (t) applyTheme(t);
          }
        }),
        await onThemesChanged((updated) => {
          const dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const current =
            updated.find((t) => t.name === resolveThemeName($settings, dark)) ?? updated[0];
          if (current) applyTheme(current);
        })
      );
    })();

    return () => {
      for (const fn of cleanups) fn();
    };
  });
</script>

<div class="window">
  <!-- Titlebar -->
  <nav class="titlebar" class:macos={isMac} data-tauri-drag-region>
    <span class="titlebar-label">{m.stats_better_title()}</span>
    {#if !isMac}
      <button class="btn-refresh" onclick={() => void refresh()} aria-label={m.better_refresh()}>
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <path
            d="M12 7a5 5 0 1 1-1.6-3.7"
            stroke="currentColor"
            stroke-width="1.4"
            stroke-linecap="round"
          />
          <path
            d="M12 1.6V4.4H9.2"
            stroke="currentColor"
            stroke-width="1.4"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
      <button class="btn-close" onclick={close} aria-label="Close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          <line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        </svg>
      </button>
    {/if}
  </nav>

  <div class="content">
    {#if loading}
      <div class="state">
        <span class="state-text">{m.better_loading()}</span>
      </div>
    {:else if loadError}
      <div class="state">
        <span class="state-title">{m.better_error()}</span>
        <span class="state-body">{loadError}</span>
        <button class="retry" onclick={() => void refresh()}>{m.better_refresh()}</button>
      </div>
    {:else if !hasData}
      <div class="state">
        <span class="state-title">{m.better_empty_title()}</span>
        <span class="state-body">{m.better_empty_body()}</span>
      </div>
    {:else if insights}
      <div class="dashboard">
        <HeroSection {insights} />
        <MomentumCharts {insights} />
        <ConsistencySection {insights} />
        <RhythmSection {insights} />
        <RecordsSection {insights} />
      </div>
    {/if}
  </div>
</div>

<style>
  .window {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--color-background);
    color: var(--color-foreground);
    animation: app-fade-in 0.18s ease;
    overflow: hidden;
    cursor: default;
  }

  /* ── Titlebar ──────────────────────────────────────────── */
  .titlebar {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    flex-shrink: 0;
    border-bottom: 1px solid var(--color-separator);
  }

  .macos {
    padding-left: 72px;
  }

  .titlebar-label {
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    pointer-events: none;
  }

  .btn-close,
  .btn-refresh {
    position: absolute;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 4px;
    transition:
      color 0.15s,
      background 0.15s;
  }

  .btn-close {
    right: 8px;
  }

  .btn-refresh {
    right: 40px;
  }

  .btn-close:hover {
    color: var(--color-background);
    background: var(--color-focus-round);
  }

  .btn-refresh:hover {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  /* ── Content ───────────────────────────────────────────── */
  .content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .dashboard {
    display: flex;
    flex-direction: column;
    padding: 18px 20px 24px;
    gap: 16px;
  }

  /* ── Empty / loading / error states ────────────────────── */
  .state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 40px 48px;
    text-align: center;
  }

  .state-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-foreground);
  }

  .state-text,
  .state-body {
    font-size: 0.8rem;
    line-height: 1.5;
    max-width: 42ch;
    color: color-mix(in oklch, var(--color-foreground-darker) 85%, transparent);
  }

  .retry {
    margin-top: 6px;
    padding: 6px 14px;
    border-radius: 4px;
    border: 1px solid var(--color-separator);
    background: var(--color-hover);
    color: var(--color-foreground);
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
  }

  .retry:hover {
    border-color: var(--color-focus-round);
    color: var(--color-focus-round);
  }
</style>
