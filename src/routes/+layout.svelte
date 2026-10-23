<script lang="ts">
  import { onMount } from 'svelte';
  // Dev-only Tauri IPC stand-in so a plain `npm run dev` browser session can
  // render the UI. No-ops inside the real Tauri shell and is tree-shaken out of
  // production builds (see the module's `import.meta.env.DEV` guard).
  import '$lib/dev/mockTauri.js';

  onMount(() => {
    const disableContextMenu = (event: MouseEvent) => {
      event.preventDefault();
    };

    document.addEventListener('contextmenu', disableContextMenu, { capture: true });

    return () => {
      document.removeEventListener('contextmenu', disableContextMenu, { capture: true });
    };
  });
</script>

<slot />
