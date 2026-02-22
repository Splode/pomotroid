<script lang="ts">
  import type { DrawerPanel } from '$lib/types';

  interface Props {
    active?: DrawerPanel;
    onselect?: (panel: DrawerPanel) => void;
  }

  let { active = 'timer', onselect }: Props = $props();
</script>

<nav class="menu">
  <!-- Timer -->
  <button
    class="menu-btn"
    class:active={active === 'timer'}
    title="Timer Configuration"
    onclick={() => onselect?.('timer')}
    aria-label="Timer settings"
  >
    <svg viewBox="0 0 20 20" width="18" height="18" xmlns="http://www.w3.org/2000/svg">
      <path fill="currentColor" d="M10,0C4.5,0,0,4.5,0,10s4.5,10,10,10c5.5,0,10-4.5,10-10S15.5,0,10,0z M10,18c-4.4,0-8-3.6-8-8s3.6-8,8-8s8,3.6,8,8S14.4,18,10,18z"/>
      <path fill="currentColor" d="M10.5,5H9v6l5.3,3.1l0.8-1.2l-4.5-2.7V5z"/>
    </svg>
  </button>

  <!-- Settings -->
  <button
    class="menu-btn"
    class:active={active === 'settings'}
    title="Settings"
    onclick={() => onselect?.('settings')}
    aria-label="Settings"
  >
    <svg viewBox="0 0 19.5 20" width="18" height="18" xmlns="http://www.w3.org/2000/svg">
      <path fill="currentColor" d="M17.2,11c0-0.3,0.1-0.6,0.1-1s0-0.7-0.1-1l2.1-1.6c0.2-0.1,0.2-0.4,0.1-0.6l-2-3.5C17.3,3.1,17,3,16.8,3.1l-2.5,1c-0.5-0.4-1.1-0.7-1.7-1l-0.4-2.7C12.2,0.2,12,0,11.7,0h-4C7.5,0,7.3,0.2,7.2,0.4L6.9,3.1c-0.6,0.3-1.2,0.6-1.7,1l-2.5-1C2.4,3,2.2,3.1,2.1,3.3l-2,3.5C-0.1,6.9,0,7.2,0.2,7.4L2.3,9c0,0.3-0.1,0.6-0.1,1s0,0.7,0.1,1l-2.1,1.6C0,12.8-0.1,13,0.1,13.3l2,3.5c0.1,0.2,0.4,0.3,0.6,0.2l2.5-1c0.5,0.4,1.1,0.7,1.7,1l0.4,2.6c0,0.2,0.2,0.4,0.5,0.4h4c0.3,0,0.5-0.2,0.5-0.4l0.4-2.6c0.6-0.3,1.2-0.6,1.7-1l2.5,1c0.2,0.1,0.5,0,0.6-0.2l2-3.5c0.1-0.2,0.1-0.5-0.1-0.6L17.2,11z M9.7,13.5c-1.9,0-3.5-1.6-3.5-3.5s1.6-3.5,3.5-3.5s3.5,1.6,3.5,3.5S11.7,13.5,9.7,13.5z"/>
    </svg>
  </button>

  <!-- Themes -->
  <button
    class="menu-btn"
    class:active={active === 'themes'}
    title="Themes"
    onclick={() => onselect?.('themes')}
    aria-label="Themes"
  >
    <svg viewBox="0 0 24 24" width="20" height="20" xmlns="http://www.w3.org/2000/svg">
      <path d="M0 0h24v24H0z" fill="none"/>
      <path fill="currentColor" d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/>
    </svg>
  </button>

  <!-- About -->
  <button
    class="menu-btn"
    class:active={active === 'about'}
    title="About"
    onclick={() => onselect?.('about')}
    aria-label="About"
  >
    <svg viewBox="0 0 24 24" width="20" height="20" xmlns="http://www.w3.org/2000/svg">
      <path fill="none" d="M0 0h24v24H0V0z"/>
      <path fill="currentColor" d="M11 7h2v2h-2zm0 4h2v6h-2zm1-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z"/>
    </svg>
  </button>
</nav>

<style>
  .menu {
    background: var(--color-background);
    display: flex;
    width: 100%;
    height: 36px;
    flex-shrink: 0;
  }

  .menu-btn {
    flex: 1 1 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    position: relative;
    transition: background 0.15s, color 0.15s;
    height: 100%;
  }

  .menu-btn:hover {
    background: var(--color-background-light);
    color: var(--color-foreground);
  }

  .menu-btn.active {
    color: var(--color-foreground);
  }

  /* Bottom underline indicator */
  .menu-btn::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    height: 2px;
    width: 0;
    background: var(--color-accent);
    transition: width 0.15s ease;
  }

  .menu-btn.active::after {
    width: 33%;
  }
</style>
