<script lang="ts">
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { isMac } from '$lib/utils/platform';
  import * as m from '$paraglide/messages.js';

  function close() {
    getCurrentWebviewWindow().close();
  }
</script>

<nav class="titlebar" class:macos={isMac} data-tauri-drag-region>
  <h1 class="title">{m.settings_title()}</h1>
  <!-- Hidden on macOS; the native traffic light close button handles this. -->
  {#if !isMac}
    <button class="btn-close" onclick={close} aria-label="Close">
      <svg width="12" height="12" viewBox="0 0 12 12">
        <line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        <line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>
  {/if}
</nav>

<style>
  .titlebar {
    height: 40px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    flex-shrink: 0;
    border-bottom: 1px solid var(--color-separator);
  }

  /* Shift the centered title right of the traffic lights on macOS. */
  .macos {
    padding-left: 72px;
  }

  .title {
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    pointer-events: none;
  }

  .btn-close {
    position: absolute;
    right: 8px;
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
    transition: color 0.15s, background 0.15s;
  }

  .btn-close:hover {
    color: var(--color-background);
    background: var(--color-focus-round);
  }
</style>
