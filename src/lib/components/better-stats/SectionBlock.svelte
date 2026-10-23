<script lang="ts">
  /** Titled panel used to group a chart or a set of cards. */
  interface Props {
    title: string;
    /** Optional hint rendered on the right of the header. */
    hint?: string | null;
    /** Optional slot content in the header (e.g. a toggle). */
    children?: import('svelte').Snippet;
    headerExtra?: import('svelte').Snippet;
  }

  let { title, hint = null, children, headerExtra }: Props = $props();
</script>

<section class="block">
  <header class="head">
    <h2 class="title">{title}</h2>
    {#if headerExtra}
      {@render headerExtra()}
    {:else if hint}
      <span class="hint">{hint}</span>
    {/if}
  </header>
  {@render children?.()}
</section>

<style>
  .block {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
  }

  .title {
    margin: 0;
    font-size: 0.68rem;
    font-weight: 700;
    letter-spacing: 0.11em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .hint {
    font-size: 0.62rem;
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
  }
</style>
