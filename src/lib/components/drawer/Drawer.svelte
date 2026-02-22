<script lang="ts">
  // Drawer shell — slides in from the right side.
  // Contains DrawerMenu (bottom tab bar) and the active panel content.
  import { fade } from 'svelte/transition';
  import type { DrawerPanel } from '$lib/types';
  import DrawerMenu from './DrawerMenu.svelte';
  import DrawerTimer from './DrawerTimer.svelte';
  import DrawerSettings from './DrawerSettings.svelte';
  import DrawerTheme from './DrawerTheme.svelte';
  import DrawerAbout from './DrawerAbout.svelte';

  interface Props {
    open?: boolean;
  }

  let { open = false }: Props = $props();

  let activePanel = $state<DrawerPanel>('timer');
</script>

<div class="drawer" class:open aria-hidden={!open}>
  <!-- Panel content fades when switching tabs -->
  {#key activePanel}
    <div class="panel-content" in:fade={{ duration: 140 }}>
      {#if activePanel === 'timer'}
        <DrawerTimer />
      {:else if activePanel === 'settings'}
        <DrawerSettings />
      {:else if activePanel === 'themes'}
        <DrawerTheme />
      {:else}
        <DrawerAbout />
      {/if}
    </div>
  {/key}

  <DrawerMenu active={activePanel} onselect={(p) => (activePanel = p)} />
</div>

<style>
  .drawer {
    position: fixed;
    top: 0;
    right: 0;
    height: 100%;
    width: 260px;
    transform: translateX(100%);
    transition: transform var(--transition-slow);
    background: var(--color-background-light);
    z-index: 50;
    display: flex;
    flex-direction: column;
  }

  .drawer.open {
    transform: translateX(0);
  }

  .panel-content {
    flex: 1;
    overflow: hidden;
  }
</style>
