<script lang="ts">
  // Theme selection panel. Each card is rendered with its own theme colors.
  import { onMount } from 'svelte';
  import type { Theme } from '$lib/types';
  import { settings } from '$lib/stores/settings';
  import { activeTheme, applyTheme } from '$lib/stores/theme';
  import { getThemes, setSetting, onThemesChanged } from '$lib/ipc';
  import { resolveThemeName } from '$lib/utils/theme';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  let themes = $state<Theme[]>([]);
  let osDark = $state(window.matchMedia('(prefers-color-scheme: dark)').matches);
  let activeName = $derived(resolveThemeName($settings, osDark));

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

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

  async function select(theme: Theme) {
    // Apply immediately for instant preview.
    applyTheme(theme);
    // Save to whichever picker is currently active.
    const key = osDark || $settings.theme_mode === 'dark' ? 'theme_dark' : 'theme_light';
    await setSetting(key, theme.name);
  }
</script>

<div class="panel">
  <p class="heading">Themes</p>

  {#each themes as theme (theme.name)}
    {@const bg = theme.colors['--color-background'] ?? '#2f384b'}
    {@const fg = theme.colors['--color-foreground'] ?? '#d7e1f4'}
    {@const accent = theme.colors['--color-accent'] ?? '#e25d60'}
    {@const isActive = theme.name === activeName}
    <button
      class="card"
      class:active={isActive}
      style="background:{bg}; border-color:{accent};"
      onclick={() => select(theme)}
    >
      <span class="card-name" style="color:{fg}">{theme.name}</span>
      {#if theme.is_custom}
        <span class="badge" style="color:{accent}">custom</span>
      {/if}
      {#if isActive}
        <svg width="18" height="18" viewBox="0 0 24 24" style="fill:{accent}; flex-shrink:0;">
          <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/>
        </svg>
      {/if}
    </button>
  {/each}
</div>

<style>
  .panel {
    padding: 8px 12px;
    overflow-y: auto;
    height: 100%;
  }

  .heading {
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    margin-bottom: 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--color-separator);
  }

  .card {
    align-items: center;
    border-left: 3px solid;
    border-top: none;
    border-right: none;
    border-bottom: none;
    border-radius: 0 4px 4px 0;
    cursor: pointer;
    display: flex;
    gap: 8px;
    justify-content: space-between;
    margin: 6px 0;
    min-height: 44px;
    padding: 0 12px;
    width: 100%;
    transition: opacity 0.15s;
  }

  .card:hover {
    opacity: 0.9;
  }

  .card-name {
    font-size: 0.8rem;
    letter-spacing: 0.04em;
    text-align: left;
    flex: 1;
  }

  .badge {
    font-size: 0.6rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    opacity: 0.7;
  }
</style>
