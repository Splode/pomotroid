<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import Timer from '$lib/components/Timer.svelte';
  import Drawer from '$lib/components/drawer/Drawer.svelte';
  import { getSettings, getThemes, onSettingsChanged, onThemesChanged } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  let drawerOpen = $state(false);

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    (async () => {
      // Load settings from backend.
      const s = await getSettings();
      settings.set(s);

      // Load and apply the active theme.
      const themes = await getThemes();
      const active = themes.find((t) => t.name === s.theme) ?? themes[0];
      if (active) applyTheme(active);

      // Keep settings store in sync with backend changes.
      cleanups.push(
        await onSettingsChanged((updated) => {
          settings.set(updated);
        }),
        // Re-apply theme when custom themes are hot-reloaded.
        await onThemesChanged((updated) => {
          const current = updated.find((t) => t.name === s.theme) ?? updated[0];
          if (current) applyTheme(current);
        }),
      );
    })();

    return () => {
      for (const fn of cleanups) fn();
    };
  });
</script>

<div class="app">
  <Titlebar {drawerOpen} ontoggle={() => (drawerOpen = !drawerOpen)} />
  <main>
    <Timer />
  </main>
  <Drawer open={drawerOpen} />
</div>

<style>
  .app {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: app-fade-in 0.4s var(--transition-slow) both;
  }

  main {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
