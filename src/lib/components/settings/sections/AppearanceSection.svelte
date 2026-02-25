<script lang="ts">
  import { onMount } from 'svelte';
  import type { Theme } from '$lib/types';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { getThemes, applyThemeByName, onThemesChanged } from '$lib/ipc';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  let themes = $state<Theme[]>([]);
  let activeName = $derived($settings.theme);

  onMount(() => {
    const cleanups: UnlistenFn[] = [];
    (async () => {
      themes = await getThemes();
      cleanups.push(await onThemesChanged((updated) => { themes = updated; }));
    })();
    return () => { for (const fn of cleanups) fn(); };
  });

  async function select(theme: Theme) {
    applyTheme(theme);
    const applied = await applyThemeByName(theme.name);
    applyTheme(applied);
    settings.update((s) => ({ ...s, theme: theme.name }));
  }
</script>

<div class="section">
  {#each themes as theme (theme.name)}
    {@const bg = theme.colors['--color-background'] ?? '#2f384b'}
    {@const fg = theme.colors['--color-foreground'] ?? '#d7e1f4'}
    {@const accent = theme.colors['--color-accent'] ?? '#e25d60'}
    {@const focusRound = theme.colors['--color-focus-round'] ?? '#e25d60'}
    {@const shortRound = theme.colors['--color-short-round'] ?? '#3baf82'}
    {@const longRound = theme.colors['--color-long-round'] ?? '#3d85c8'}
    {@const isActive = theme.name === activeName}
    <button
      class="card"
      class:active={isActive}
      style="--card-bg:{bg}; --card-fg:{fg}; --card-accent:{accent};"
      onclick={() => select(theme)}
    >
      <!-- Color swatches: show the three round colors, which are always
           distinct from the background and from each other. -->
      <span class="swatches">
        <span class="swatch" style="background:{focusRound}"></span>
        <span class="swatch" style="background:{shortRound}"></span>
        <span class="swatch" style="background:{longRound}"></span>
      </span>
      <span class="card-name" style="color:{fg}">{theme.name}</span>
      <span class="card-right">
        {#if theme.is_custom}
          <span class="badge" style="color:{accent}">custom</span>
        {/if}
        {#if isActive}
          <svg width="16" height="16" viewBox="0 0 24 24" style="fill:{accent}; flex-shrink:0;">
            <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/>
          </svg>
        {/if}
      </span>
    </button>
  {/each}
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px 20px;
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

  .card.active {
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
    border: 1px solid rgba(255, 255, 255, 0.1);
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
