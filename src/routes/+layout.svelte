<script lang="ts">
  import { onMount } from 'svelte';

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
