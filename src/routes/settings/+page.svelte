<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import { getSettings, getThemes, onSettingsChanged, onThemesChanged } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  import SettingsTitlebar from '$lib/components/settings/SettingsTitlebar.svelte';
  import TimerSection from '$lib/components/settings/sections/TimerSection.svelte';
  import AppearanceSection from '$lib/components/settings/sections/AppearanceSection.svelte';
  import BehaviorSection from '$lib/components/settings/sections/BehaviorSection.svelte';
  import AudioSection from '$lib/components/settings/sections/AudioSection.svelte';
  import ShortcutsSection from '$lib/components/settings/sections/ShortcutsSection.svelte';
  import AdvancedSection from '$lib/components/settings/sections/AdvancedSection.svelte';
  import AboutSection from '$lib/components/settings/sections/AboutSection.svelte';

  type Section = 'timer' | 'appearance' | 'behavior' | 'audio' | 'shortcuts' | 'advanced' | 'about';

  const SECTIONS: { id: Section; label: string }[] = [
    { id: 'timer',      label: 'Timer' },
    { id: 'appearance', label: 'Appearance' },
    { id: 'behavior',   label: 'Behavior' },
    { id: 'audio',      label: 'Audio' },
    { id: 'shortcuts',  label: 'Shortcuts' },
    { id: 'advanced',   label: 'Advanced' },
    { id: 'about',      label: 'About' },
  ];

  let active = $state<Section>('timer');

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    (async () => {
      const s = await getSettings();
      settings.set(s);

      const themes = await getThemes();
      const activeTheme = themes.find((t) => t.name === s.theme) ?? themes[0];
      if (activeTheme) applyTheme(activeTheme);

      cleanups.push(
        await onSettingsChanged(async (updated) => {
          const prevTheme = $settings.theme;
          settings.set(updated);
          if (updated.theme !== prevTheme) {
            const allThemes = await getThemes();
            const t = allThemes.find((t) => t.name === updated.theme);
            if (t) applyTheme(t);
          }
        }),
        await onThemesChanged((updated) => {
          const current = updated.find((t) => t.name === $settings.theme) ?? updated[0];
          if (current) applyTheme(current);
        }),
      );
    })();

    return () => { for (const fn of cleanups) fn(); };
  });
</script>

<div class="window">
  <SettingsTitlebar />

  <div class="body">
    <!-- Left sidebar navigation -->
    <aside class="sidebar">
      <nav>
        {#each SECTIONS as section}
          <button
            class="nav-item"
            class:active={active === section.id}
            onclick={() => { active = section.id; }}
          >
            {section.label}
          </button>
        {/each}
      </nav>
    </aside>

    <!-- Right content area -->
    <main class="content">
      {#if active === 'timer'}
        <TimerSection />
      {:else if active === 'appearance'}
        <AppearanceSection />
      {:else if active === 'behavior'}
        <BehaviorSection />
      {:else if active === 'audio'}
        <AudioSection />
      {:else if active === 'shortcuts'}
        <ShortcutsSection />
      {:else if active === 'advanced'}
        <AdvancedSection />
      {:else if active === 'about'}
        <AboutSection />
      {/if}
    </main>
  </div>
</div>

<style>
  .window {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-background);
    animation: app-fade-in 0.2s ease both;
  }

  .body {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  /* Sidebar */
  .sidebar {
    width: 180px;
    flex-shrink: 0;
    border-right: 1px solid rgba(255, 255, 255, 0.06);
    background: var(--color-background-light);
    overflow-y: auto;
    padding: 8px 0;
  }

  .nav-item {
    display: block;
    width: 100%;
    padding: 9px 16px;
    background: none;
    border: none;
    border-left: 3px solid transparent;
    text-align: left;
    font-size: 0.85rem;
    letter-spacing: 0.03em;
    color: var(--color-foreground-darker, var(--color-foreground));
    cursor: pointer;
    transition: color 0.12s, background 0.12s, border-color 0.12s;
  }

  .nav-item:hover {
    color: var(--color-foreground);
    background: rgba(255, 255, 255, 0.04);
  }

  .nav-item.active {
    color: var(--color-foreground);
    border-left-color: var(--color-accent);
    background: rgba(255, 255, 255, 0.06);
    font-weight: 500;
  }

  /* Content */
  .content {
    flex: 1;
    overflow-y: auto;
    min-width: 0;
  }
</style>
