<script lang="ts">
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { setWindowVisibility } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';

  interface Props {
    drawerOpen?: boolean;
    ontoggle?: () => void;
  }

  let { drawerOpen = false, ontoggle }: Props = $props();

  async function minimize() {
    if ($settings.min_to_tray) {
      await setWindowVisibility(false);
    } else {
      await getCurrentWebviewWindow().minimize();
    }
  }

  function close() {
    getCurrentWebviewWindow().close();
  }
</script>

<nav class="titlebar" data-tauri-drag-region>
  <!-- Hamburger / close drawer toggle -->
  <button class="btn-icon hamburger" class:open={drawerOpen} onclick={ontoggle} aria-label="Menu">
    <span class="bar bar-top"></span>
    <span class="bar bar-bottom"></span>
  </button>

  <!-- Title -->
  <h1 class="title">Pomotroid</h1>

  <!-- Window controls -->
  <div class="controls">
    <button class="btn-icon" onclick={minimize} aria-label="Minimize">
      <svg width="12" height="12" viewBox="0 0 12 12">
        <line x1="1" y1="6" x2="11" y2="6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>
    <button class="btn-icon close" onclick={close} aria-label="Close">
      <svg width="12" height="12" viewBox="0 0 12 12">
        <line x1="1" y1="1" x2="11" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        <line x1="11" y1="1" x2="1" y2="11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>
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

  .title {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--color-short-round);
    pointer-events: none;
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
    transition: color 0.15s, background 0.15s;
  }

  .btn-icon:hover {
    color: var(--color-foreground);
    background: rgba(255, 255, 255, 0.06);
  }

  .btn-icon.close:hover {
    color: #fff;
    background: #e74c3c;
  }

  /* Hamburger bars */
  .hamburger {
    flex-direction: column;
    gap: 5px;
    width: 32px;
    height: 32px;
  }

  .bar {
    display: block;
    width: 18px;
    height: 2px;
    background: currentColor;
    border-radius: 1px;
    transition: transform 0.2s ease, opacity 0.2s ease;
  }

  .hamburger.open .bar-top {
    transform: translateY(3.5px) rotate(45deg);
  }

  .hamburger.open .bar-bottom {
    transform: translateY(-3.5px) rotate(-45deg);
  }
</style>
