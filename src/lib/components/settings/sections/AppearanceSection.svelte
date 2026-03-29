<script lang="ts">
  import { onMount } from 'svelte';
  import type { Theme } from '$lib/types';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { getThemes, setSetting, onThemesChanged } from '$lib/ipc';
  import { resolveThemeName } from '$lib/utils/theme';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import * as m from '$paraglide/messages.js';

  let themes = $state<Theme[]>([]);
  let osDark = $state(window.matchMedia('(prefers-color-scheme: dark)').matches);
  // Accordion: at most one picker open at a time.
  let openPicker = $state<'light' | 'dark' | null>(null);

  // Light picker is active when mode='light', or mode='auto' and OS is light.
  let lightIsActive = $derived(
    $settings.theme_mode === 'light' ||
    ($settings.theme_mode === 'auto' && !osDark)
  );
  // Dark picker is active when mode='dark', or mode='auto' and OS is dark.
  let darkIsActive = $derived(
    $settings.theme_mode === 'dark' ||
    ($settings.theme_mode === 'auto' && osDark)
  );

  let selectedLightTheme = $derived(themes.find((t) => t.name === $settings.theme_light));
  let selectedDarkTheme = $derived(themes.find((t) => t.name === $settings.theme_dark));

  function togglePicker(picker: 'light' | 'dark') {
    openPicker = openPicker === picker ? null : picker;
  }

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    // Track live OS color scheme changes to update picker highlights.
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    const mqListener = (e: MediaQueryListEvent) => { osDark = e.matches; };
    mq.addEventListener('change', mqListener);
    cleanups.push(() => mq.removeEventListener('change', mqListener));

    (async () => {
      themes = await getThemes();
      cleanups.push(await onThemesChanged((updated) => { themes = updated; }));
    })();

    return () => { for (const fn of cleanups) fn(); };
  });

  // Mode selector: save + immediately apply the resolved theme.
  async function setMode(mode: string) {
    const resolved = resolveThemeName({ ...$settings, theme_mode: mode }, osDark);
    const t = themes.find((th) => th.name === resolved);
    if (t) applyTheme(t);
    await setSetting('theme_mode', mode);
  }

  // Light picker: save + apply only if light picker is currently active.
  async function selectLight(theme: Theme) {
    if (lightIsActive) applyTheme(theme);
    await setSetting('theme_light', theme.name);
  }

  // Dark picker: save + apply only if dark picker is currently active.
  async function selectDark(theme: Theme) {
    if (darkIsActive) applyTheme(theme);
    await setSetting('theme_dark', theme.name);
  }
</script>

<div class="section">

  <!-- Mode selector -->
  <div class="group-label">{m.appearance_group_mode()}</div>
  <div class="mode-selector">
    {#each [['auto', m.appearance_mode_auto()], ['light', m.appearance_mode_light()], ['dark', m.appearance_mode_dark()]] as [value, label] (value)}
      <button
        class="mode-btn"
        class:active={$settings.theme_mode === value}
        onclick={() => setMode(value)}
      >
        {label}
      </button>
    {/each}
  </div>

  <!-- Light theme picker -->
  <div class="picker-group">
    <button
      class="picker-trigger"
      class:open={openPicker === 'light'}
      onclick={() => togglePicker('light')}
    >
      <span class="trigger-label">
        {m.appearance_group_light_theme()}
        {#if lightIsActive}<span class="active-badge">{m.appearance_badge_active()}</span>{/if}
      </span>
      <span class="trigger-preview">
        <span class="preview-name">{$settings.theme_light}</span>
        {#if selectedLightTheme}
          {@const bg = selectedLightTheme.colors['--color-background'] ?? '#2f384b'}
          {@const focusRound = selectedLightTheme.colors['--color-focus-round'] ?? '#e25d60'}
          {@const shortRound = selectedLightTheme.colors['--color-short-round'] ?? '#3baf82'}
          {@const longRound = selectedLightTheme.colors['--color-long-round'] ?? '#3d85c8'}
          <span class="preview-swatches" style="background:{bg}">
            <span class="swatch" style="background:{focusRound}"></span>
            <span class="swatch" style="background:{shortRound}"></span>
            <span class="swatch" style="background:{longRound}"></span>
          </span>
        {/if}
      </span>
      <svg class="chevron" class:rotated={openPicker === 'light'} width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
        <path d="M7 10l5 5 5-5z"/>
      </svg>
    </button>
    {#if openPicker === 'light'}
    <div class="theme-list">
      {#each themes as theme (theme.name)}
        {@const bg = theme.colors['--color-background'] ?? '#2f384b'}
        {@const fg = theme.colors['--color-foreground'] ?? '#d7e1f4'}
        {@const accent = theme.colors['--color-accent'] ?? '#e25d60'}
        {@const focusRound = theme.colors['--color-focus-round'] ?? '#e25d60'}
        {@const shortRound = theme.colors['--color-short-round'] ?? '#3baf82'}
        {@const longRound = theme.colors['--color-long-round'] ?? '#3d85c8'}
        {@const isSelected = theme.name === $settings.theme_light}
        <button
          class="card"
          class:selected={isSelected}
          class:highlighted={isSelected && lightIsActive}
          style="--card-bg:{bg}; --card-fg:{fg}; --card-accent:{accent};"
          onclick={() => selectLight(theme)}
        >
          <span class="swatches">
            <span class="swatch" style="background:{focusRound}"></span>
            <span class="swatch" style="background:{shortRound}"></span>
            <span class="swatch" style="background:{longRound}"></span>
          </span>
          <span class="card-name" style="color:{fg}">{theme.name}</span>
          <span class="card-right">
            {#if theme.is_custom}
              <span class="badge" style="color:{accent}">{m.appearance_badge_custom()}</span>
            {/if}
            {#if isSelected}
              <svg width="16" height="16" viewBox="0 0 24 24" style="fill:{accent}; flex-shrink:0;">
                <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/>
              </svg>
            {/if}
          </span>
        </button>
      {/each}
    </div>
    {/if}
  </div>

  <!-- Dark theme picker -->
  <div class="picker-group">
    <button
      class="picker-trigger"
      class:open={openPicker === 'dark'}
      onclick={() => togglePicker('dark')}
    >
      <span class="trigger-label">
        {m.appearance_group_dark_theme()}
        {#if darkIsActive}<span class="active-badge">{m.appearance_badge_active()}</span>{/if}
      </span>
      <span class="trigger-preview">
        <span class="preview-name">{$settings.theme_dark}</span>
        {#if selectedDarkTheme}
          {@const bg = selectedDarkTheme.colors['--color-background'] ?? '#2f384b'}
          {@const focusRound = selectedDarkTheme.colors['--color-focus-round'] ?? '#e25d60'}
          {@const shortRound = selectedDarkTheme.colors['--color-short-round'] ?? '#3baf82'}
          {@const longRound = selectedDarkTheme.colors['--color-long-round'] ?? '#3d85c8'}
          <span class="preview-swatches" style="background:{bg}">
            <span class="swatch" style="background:{focusRound}"></span>
            <span class="swatch" style="background:{shortRound}"></span>
            <span class="swatch" style="background:{longRound}"></span>
          </span>
        {/if}
      </span>
      <svg class="chevron" class:rotated={openPicker === 'dark'} width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
        <path d="M7 10l5 5 5-5z"/>
      </svg>
    </button>
    {#if openPicker === 'dark'}
    <div class="theme-list">
      {#each themes as theme (theme.name)}
        {@const bg = theme.colors['--color-background'] ?? '#2f384b'}
        {@const fg = theme.colors['--color-foreground'] ?? '#d7e1f4'}
        {@const accent = theme.colors['--color-accent'] ?? '#e25d60'}
        {@const focusRound = theme.colors['--color-focus-round'] ?? '#e25d60'}
        {@const shortRound = theme.colors['--color-short-round'] ?? '#3baf82'}
        {@const longRound = theme.colors['--color-long-round'] ?? '#3d85c8'}
        {@const isSelected = theme.name === $settings.theme_dark}
        <button
          class="card"
          class:selected={isSelected}
          class:highlighted={isSelected && darkIsActive}
          style="--card-bg:{bg}; --card-fg:{fg}; --card-accent:{accent};"
          onclick={() => selectDark(theme)}
        >
          <span class="swatches">
            <span class="swatch" style="background:{focusRound}"></span>
            <span class="swatch" style="background:{shortRound}"></span>
            <span class="swatch" style="background:{longRound}"></span>
          </span>
          <span class="card-name" style="color:{fg}">{theme.name}</span>
          <span class="card-right">
            {#if theme.is_custom}
              <span class="badge" style="color:{accent}">{m.appearance_badge_custom()}</span>
            {/if}
            {#if isSelected}
              <svg width="16" height="16" viewBox="0 0 24 24" style="fill:{accent}; flex-shrink:0;">
                <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/>
              </svg>
            {/if}
          </span>
        </button>
      {/each}
    </div>
    {/if}
  </div>

</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .group-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    margin: 0;
    padding: 16px 20px 6px;
  }

  .active-badge {
    font-size: 0.6rem;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    padding: 1px 5px;
    border-radius: 3px;
    background: color-mix(in oklch, var(--color-accent) 20%, transparent);
    color: var(--color-accent);
    opacity: 1;
  }

  /* Mode selector */
  .mode-selector {
    display: flex;
    margin: 0 20px 4px;
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .mode-btn {
    flex: 1;
    padding: 6px 0;
    background: none;
    border: none;
    border-right: 1px solid var(--color-separator);
    font-size: 0.8rem;
    letter-spacing: 0.03em;
    color: var(--color-foreground-darker, var(--color-foreground));
    cursor: pointer;
    transition: background 0.12s, color 0.12s;
  }

  .mode-btn:last-child {
    border-right: none;
  }

  .mode-btn:hover {
    background: color-mix(in oklch, var(--color-background) 88%, var(--color-foreground) 12%);
  }

  .mode-btn.active {
    background: color-mix(in oklch, var(--color-accent) 18%, transparent);
    color: var(--color-accent);
    font-weight: 600;
  }

  /* Collapsible picker groups */
  .picker-group {
    margin: 8px 20px 0;
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .picker-group:last-child {
    margin-bottom: 12px;
  }

  .picker-trigger {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 9px 12px;
    background: none;
    border: none;
    cursor: pointer;
    gap: 8px;
    transition: background 0.12s;
  }

  .picker-trigger:hover {
    background: color-mix(in oklch, var(--color-background) 88%, var(--color-foreground) 12%);
  }

  .picker-trigger.open {
    border-bottom: 1px solid var(--color-separator);
  }

  .trigger-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.7;
    flex-shrink: 0;
  }

  .trigger-preview {
    display: flex;
    align-items: center;
    gap: 7px;
    flex: 1;
    justify-content: flex-end;
    min-width: 0;
  }

  .preview-swatches {
    display: flex;
    gap: 3px;
    flex-shrink: 0;
    padding: 4px 6px;
    border-radius: 4px;
  }

  .preview-name {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 0;
  }

  .chevron {
    flex-shrink: 0;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.5;
    transition: transform 0.15s;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  /* Theme list */
  .theme-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px 10px 10px;
  }

  .card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    border-radius: 6px;
    border: 1px solid transparent;
    background: var(--card-bg);
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: border-color 0.15s, background 0.15s;
  }

  .card:hover {
    background: color-mix(in srgb, var(--card-bg) 88%, white 12%);
  }

  /* selected but NOT the active picker: dimmed border */
  .card.selected {
    border-color: color-mix(in oklch, var(--card-accent) 40%, transparent);
  }

  /* selected AND the active picker: full accent border + checkmark shown */
  .card.highlighted {
    border-color: var(--card-accent);
  }

  .swatches {
    display: flex;
    gap: 3px;
    flex-shrink: 0;
  }

  .swatch {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 14%, transparent);
  }

  .card-name {
    font-size: 0.85rem;
    letter-spacing: 0.03em;
    flex: 1;
  }

  .card-right {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .badge {
    font-size: 0.65rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    opacity: 0.8;
  }
</style>
