<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import { getSettings, getThemes, onSettingsChanged, onThemesChanged, achievementRecordEvent } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { applyTheme } from '$lib/stores/theme';
  import { resolveThemeName } from '$lib/utils/theme';
  import { setLocale } from '$lib/locale.svelte.js';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { info, error as logError } from '@tauri-apps/plugin-log';
  import { createLocalShortcutHandler } from '$lib/utils/localShortcuts';

  import SettingsTitlebar from '$lib/components/settings/SettingsTitlebar.svelte';
  import TimerSection from '$lib/components/settings/sections/TimerSection.svelte';
  import AppearanceSection from '$lib/components/settings/sections/AppearanceSection.svelte';
  import NotificationsSection from '$lib/components/settings/sections/NotificationsSection.svelte';
  import ShortcutsSection from '$lib/components/settings/sections/ShortcutsSection.svelte';
  import SystemSection from '$lib/components/settings/sections/SystemSection.svelte';
  import AboutSection from '$lib/components/settings/sections/AboutSection.svelte';

  import * as m from '$paraglide/messages.js';

  type Section = 'timer' | 'appearance' | 'notifications' | 'shortcuts' | 'system' | 'about';

  const SECTIONS: { id: Section; label: () => string }[] = [
    { id: 'timer',         label: m.nav_timer },
    { id: 'appearance',    label: m.nav_appearance },
    { id: 'notifications', label: m.nav_notifications },
    { id: 'shortcuts',     label: m.nav_shortcuts },
    { id: 'system',        label: m.nav_system },
    { id: 'about',         label: m.nav_about },
  ];

  let active = $state<Section>('timer');

  // Local shortcut state for the settings window.
  let localVolume = $state(1.0);
  let preMuteVolume = $state(0.5);
  let isFullscreen = $state(false);

  onMount(() => {
    const cleanups: UnlistenFn[] = [];

    // Mount local keyboard shortcut handler.
    const shortcutHandler = createLocalShortcutHandler({
      getSettings: () => $settings,
      getVolume: () => localVolume,
      setVolume: (v) => { localVolume = v; },
      getPreMuteVolume: () => preMuteVolume,
      setPreMuteVolume: (v) => { preMuteVolume = v; },
      getFullscreen: () => isFullscreen,
      setFullscreen: (v) => { isFullscreen = v; },
    });
    document.addEventListener('keydown', shortcutHandler);
    cleanups.push(() => document.removeEventListener('keydown', shortcutHandler));

    (async () => {
      try {
      const s = await getSettings();
      settings.set(s);
      localVolume = s.volume;

      // Apply the stored locale on mount.
      setLocale(s.language);
      await info(`[settings] settings loaded, locale=${s.language}`);

      const themes = await getThemes();
      const osDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      const activeTheme = themes.find((t) => t.name === resolveThemeName(s, osDark)) ?? themes[0];
      if (activeTheme) applyTheme(activeTheme);
      await info(`[settings] initialized, theme=${activeTheme?.name ?? 'none'}`);

      // Show the window now that the theme is applied (avoids white flash)
      await getCurrentWebviewWindow().show();

      // Achievement: record settings page opened.
      achievementRecordEvent('settings_opened').catch(() => {});
      } catch (e) {
        await logError(`[settings] initialization failed: ${e}`);
        throw e;
      }

      // Live OS color scheme changes — re-resolve only in auto mode.
      const mq = window.matchMedia('(prefers-color-scheme: dark)');
      const mqListener = async (e: MediaQueryListEvent) => {
        if ($settings.theme_mode !== 'auto') return;
        const allThemes = await getThemes();
        const t = allThemes.find((th) => th.name === resolveThemeName($settings, e.matches));
        if (t) applyTheme(t);
      };
      mq.addEventListener('change', mqListener);
      cleanups.push(() => mq.removeEventListener('change', mqListener));

      cleanups.push(
        await onSettingsChanged(async (updated) => {
          const prevMode = $settings.theme_mode;
          const prevLight = $settings.theme_light;
          const prevDark = $settings.theme_dark;
          const prevLanguage = $settings.language;
          settings.set(updated);
          localVolume = updated.volume;
          if (updated.language !== prevLanguage) {
            setLocale(updated.language);
          }
          if (
            updated.theme_mode !== prevMode ||
            updated.theme_light !== prevLight ||
            updated.theme_dark !== prevDark
          ) {
            const allThemes = await getThemes();
            const dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const t = allThemes.find((th) => th.name === resolveThemeName(updated, dark));
            if (t) applyTheme(t);
          }
        }),
        await onThemesChanged((updated) => {
          const dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const current = updated.find((t) => t.name === resolveThemeName($settings, dark)) ?? updated[0];
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
            {section.label()}
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
      {:else if active === 'notifications'}
        <NotificationsSection />
      {:else if active === 'shortcuts'}
        <ShortcutsSection />
      {:else if active === 'system'}
        <SystemSection />
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
    min-height: 0;
    display: flex;
    overflow: hidden;
  }

  /* Sidebar */
  .sidebar {
    width: 180px;
    flex-shrink: 0;
    border-right: 1px solid var(--color-separator);
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
    background: var(--color-hover);
  }

  .nav-item.active {
    color: var(--color-foreground);
    border-left-color: var(--color-accent);
    background: var(--color-hover);
    font-weight: 500;
  }

  /* Content */
  .content {
    flex: 1;
    overflow-y: auto;
    min-width: 0;
  }
</style>
