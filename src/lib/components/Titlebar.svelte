<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { setWindowVisibility } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import { isMac } from '$lib/utils/platform';
  import Tooltip from './Tooltip.svelte';
  import * as m from '$paraglide/messages.js';

  let maximized = $state(false);

  onMount(() => {
    const win = getCurrentWebviewWindow();
    win.isMaximized().then((v) => {
      maximized = v;
    });
    const unlisten = win.onResized(async () => {
      maximized = await win.isMaximized();
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });

  async function openSettings() {
    const existing = await WebviewWindow.getByLabel('settings');
    if (existing) {
      await existing.show();
      await existing.setFocus();
      return;
    }
    new WebviewWindow('settings', {
      url: '/settings',
      title: 'Pomotroid — Settings',
      width: 720,
      height: 520,
      // On macOS: native decorations + overlay titlebar for rounded corners and
      // traffic light buttons. On other platforms: custom decorations-free window.
      decorations: isMac,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      titleBarStyle: isMac ? ('Overlay' as any) : undefined,
      hiddenTitle: isMac ? true : undefined,
      resizable: false,
      visible: false,
    });
  }

  async function openStats() {
    const existing = await WebviewWindow.getByLabel('stats');
    if (existing) {
      await existing.show();
      await existing.setFocus();
      return;
    }
    new WebviewWindow('stats', {
      url: '/stats',
      title: 'Pomotroid — Statistics',
      width: 840,
      height: 520,
      decorations: isMac,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      titleBarStyle: isMac ? ('Overlay' as any) : undefined,
      hiddenTitle: isMac ? true : undefined,
      resizable: false,
      visible: false,
    });
  }

  async function minimize() {
    if ($settings.min_to_tray) {
      await setWindowVisibility(false);
    } else {
      await getCurrentWebviewWindow().minimize();
    }
  }

  function toggleMaximize() {
    getCurrentWebviewWindow().toggleMaximize();
  }

  function close() {
    getCurrentWebviewWindow().close();
  }
</script>

{#snippet settingsBtn()}
  <Tooltip text={m.tooltip_settings()}>
    <button class="btn-icon" onclick={openSettings} aria-label="Settings">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <line
          x1="2"
          y1="4"
          x2="14"
          y2="4"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
        <circle
          cx="5"
          cy="4"
          r="1.8"
          fill="var(--color-background)"
          stroke="currentColor"
          stroke-width="1.3"
        />
        <line
          x1="2"
          y1="8"
          x2="14"
          y2="8"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
        <circle
          cx="11"
          cy="8"
          r="1.8"
          fill="var(--color-background)"
          stroke="currentColor"
          stroke-width="1.3"
        />
        <line
          x1="2"
          y1="12"
          x2="14"
          y2="12"
          stroke="currentColor"
          stroke-width="1.3"
          stroke-linecap="round"
        />
        <circle
          cx="7"
          cy="12"
          r="1.8"
          fill="var(--color-background)"
          stroke="currentColor"
          stroke-width="1.3"
        />
      </svg>
    </button>
  </Tooltip>
{/snippet}

{#snippet statsBtn()}
  <Tooltip text={m.tooltip_statistics()}>
    <button class="btn-icon" onclick={openStats} aria-label="Statistics">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <rect x="2" y="9" width="3" height="5" rx="0.5" fill="currentColor" opacity="0.6" />
        <rect x="6.5" y="5" width="3" height="9" rx="0.5" fill="currentColor" opacity="0.8" />
        <rect x="11" y="2" width="3" height="12" rx="0.5" fill="currentColor" />
      </svg>
    </button>
  </Tooltip>
{/snippet}

<nav class="titlebar" data-tauri-drag-region>
  <!-- Left: settings + stats buttons on Linux/Windows. On macOS the traffic
       lights live here; the action buttons move to the right side instead. -->
  {#if !isMac}
    {@render settingsBtn()}
    {@render statsBtn()}
  {/if}

  <!-- Right: settings + stats buttons on macOS, window controls on Linux/Windows. -->
  <div class="controls">
    {#if isMac}
      {@render statsBtn()}
      {@render settingsBtn()}
    {:else}
      <button class="btn-icon" onclick={minimize} aria-label="Minimize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line
            x1="1"
            y1="6"
            x2="11"
            y2="6"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
        </svg>
      </button>
      <button
        class="btn-icon"
        onclick={toggleMaximize}
        aria-label={maximized ? 'Restore' : 'Maximize'}
      >
        {#if maximized}
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect
              x="3"
              y="1"
              width="8"
              height="8"
              rx="1"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
            />
            <path
              d="M1 4 L1 11 L8 11"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        {:else}
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect
              x="1"
              y="1"
              width="10"
              height="10"
              rx="1"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
            />
          </svg>
        {/if}
      </button>
      <button class="btn-icon close" onclick={close} aria-label="Close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line
            x1="1"
            y1="1"
            x2="11"
            y2="11"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
          <line
            x1="11"
            y1="1"
            x2="1"
            y2="11"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
        </svg>
      </button>
    {/if}
  </div>
</nav>

<style>
  .titlebar {
    height: 40px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    position: relative;
    flex-shrink: 0;
  }

  .controls {
    display: flex;
    gap: 4px;
    margin-left: auto;
  }

  .btn-icon {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
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

  .btn-icon:hover {
    color: var(--color-foreground);
    background: var(--color-hover);
  }

  .btn-icon.close:hover {
    color: var(--color-background);
    background: var(--color-focus-round);
  }
</style>
