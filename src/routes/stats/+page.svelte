<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import {
    getSettings, getThemes,
    onSettingsChanged, onThemesChanged, onRoundChange, onSessionsCleared,
    statsGetDetailed, statsGetHeatmap, statsGetLabelBreakdown, statsGetWeeklyLabels,
    sessionsRenameLabel,
  } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { setLocale } from '$lib/locale.svelte.js';
  import { resolveThemeName } from '$lib/utils/theme';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { isMac } from '$lib/utils/platform';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import type { DetailedStats, HeatmapStats, LabelStat, DayLabelStat } from '$lib/types';
  import * as m from '$paraglide/messages.js';
  import { info, error as logError } from '@tauri-apps/plugin-log';

  import DailyView from '$lib/components/stats/DailyView.svelte';
  import WeeklyView from '$lib/components/stats/WeeklyView.svelte';
  import YearlyView from '$lib/components/stats/YearlyView.svelte';

  type Tab = 'today' | 'week' | 'alltime';

  let activeTab = $state<Tab>('today');
  let detailed = $state<DetailedStats | null>(null);
  let heatmap = $state<HeatmapStats | null>(null);
  let heatmapLoaded = $state(false);
  let labelBreakdown = $state<LabelStat[]>([]);
  let weeklyLabels = $state<DayLabelStat[]>([]);

  async function loadLabelBreakdown(tab: Tab) {
    try {
      labelBreakdown = await statsGetLabelBreakdown(tab);
    } catch (e) {
      await logError(`[stats] failed to load label breakdown: ${e}`);
    }
  }

  async function handleRename(from: string, to: string) {
    try {
      await sessionsRenameLabel(from, to);
      detailed = await statsGetDetailed();
      if (heatmapLoaded) heatmap = await statsGetHeatmap();
      await loadLabelBreakdown(activeTab);
      weeklyLabels = await statsGetWeeklyLabels();
    } catch (e) {
      await logError(`[stats] rename label failed: ${e}`);
    }
  }

  async function switchTab(tab: Tab) {
    activeTab = tab;
    await loadLabelBreakdown(tab);
    if (tab === 'alltime' && !heatmapLoaded) {
      try {
        heatmap = await statsGetHeatmap();
        heatmapLoaded = true;
      } catch (e) {
        await logError(`[stats] failed to load heatmap: ${e}`);
      }
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
        await info(`[stats] settings loaded, locale=${s.language}`);

        const themes = await getThemes();
        const osDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const activeTheme = themes.find((t) => t.name === resolveThemeName(s, osDark)) ?? themes[0];
        if (activeTheme) applyTheme(activeTheme);
        
        // Show window immediately after theme is applied
        await getCurrentWebviewWindow().show();

        detailed = await statsGetDetailed();
        await info(`[stats] initialized, theme=${activeTheme?.name ?? 'none'}`);
      } catch (e) {
        await logError(`[stats] initialization failed: ${e}`);
        throw e;
      }

      // Load label breakdown outside the critical path so a failure here
      // cannot prevent event listener registration.
      await loadLabelBreakdown('today');
      try { weeklyLabels = await statsGetWeeklyLabels(); } catch { /* non-critical */ }

      cleanups.push(
        await onRoundChange(async () => {
          try {
            detailed = await statsGetDetailed();
            if (heatmapLoaded) heatmap = await statsGetHeatmap();
            await loadLabelBreakdown(activeTab);
            weeklyLabels = await statsGetWeeklyLabels();
          } catch (e) {
            await logError(`[stats] failed to refresh stats after round change: ${e}`);
          }
        }),
        await onSessionsCleared(async () => {
          try {
            detailed = await statsGetDetailed();
            if (heatmapLoaded) heatmap = await statsGetHeatmap();
            labelBreakdown = [];
            weeklyLabels = [];
          } catch (e) {
            await logError(`[stats] failed to refresh stats after session clear: ${e}`);
          }
        }),
        await onSettingsChanged(async (updated) => {
          const prev = {
            mode: $settings.theme_mode,
            light: $settings.theme_light,
            dark: $settings.theme_dark,
            language: $settings.language,
          };
          settings.set(updated);
          if (updated.language !== prev.language) {
            setLocale(updated.language);
          }
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
        }),
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
    <span class="titlebar-label">{m.stats_title()}</span>
    {#if !isMac}
      <button class="btn-close" onclick={close} aria-label="Close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    {/if}
  </nav>

  <!-- Tab bar -->
  <div class="tabs">
    <button class="tab" class:active={activeTab === 'today'}   onclick={() => switchTab('today')}>{m.stats_tab_today()}</button>
    <button class="tab" class:active={activeTab === 'week'}    onclick={() => switchTab('week')}>{m.stats_tab_week()}</button>
    <button class="tab" class:active={activeTab === 'alltime'} onclick={() => switchTab('alltime')}>{m.stats_tab_alltime()}</button>
  </div>

  <!-- Content -->
  <div class="content">
    {#if activeTab === 'today'}
      <DailyView today={detailed?.today ?? null} {labelBreakdown} onrename={handleRename} />
    {:else if activeTab === 'week'}
      <WeeklyView week={detailed?.week ?? null} streak={detailed?.streak ?? null} {labelBreakdown} {weeklyLabels} onrename={handleRename} />
    {:else}
      <YearlyView {heatmap} {labelBreakdown} onrename={handleRename} />
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

  .macos { padding-left: 72px; }

  .titlebar-label {
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    pointer-events: none;
  }

  .btn-close {
    position: absolute;
    right: 8px;
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
    transition: color 0.15s, background 0.15s;
  }

  .btn-close:hover {
    color: var(--color-background);
    background: var(--color-focus-round);
  }

  /* ── Tabs ──────────────────────────────────────────────── */
  .tabs {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--color-separator);
    flex-shrink: 0;
    padding: 0 24px;
  }

  .tab {
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    padding: 10px 20px;
    font-size: 0.78rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
  }

  .tab:hover {
    color: var(--color-foreground);
  }

  .tab.active {
    color: var(--color-focus-round);
    border-bottom-color: var(--color-focus-round);
  }

  /* ── Content ───────────────────────────────────────────── */
  .content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
</style>
