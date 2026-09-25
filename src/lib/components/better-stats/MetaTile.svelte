<script lang="ts">
  /** Small labelled figure used in the dense grid panels. */
  interface Props {
    label: string;
    value: string;
    sub?: string | null;
    /** Optional emphasis colour for the value. */
    tone?: 'default' | 'accent' | 'warn' | 'ok';
  }

  let { label, value, sub = null, tone = 'default' }: Props = $props();
</script>

<div class="tile">
  <span class="tile-label">{label}</span>
  <span class="tile-value" class:accent={tone === 'accent'} class:warn={tone === 'warn'} class:ok={tone === 'ok'}>
    {value}
  </span>
  {#if sub}
    <span class="tile-sub">{sub}</span>
  {/if}
</div>

<style>
  .tile {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 9px 11px;
    border-radius: 5px;
    background: color-mix(in oklch, var(--color-foreground) 5%, transparent);
    min-width: 0;
  }

  .tile-label {
    font-size: 0.58rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tile-value {
    font-size: 1.02rem;
    font-weight: 700;
    line-height: 1.1;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.01em;
    color: var(--color-foreground);
  }

  .tile-value.accent {
    color: var(--color-focus-round);
  }

  .tile-value.ok {
    color: var(--color-short-round);
  }

  .tile-value.warn {
    color: var(--color-long-round);
  }

  .tile-sub {
    font-size: 0.6rem;
    line-height: 1.3;
    color: color-mix(in oklch, var(--color-foreground-darker) 85%, transparent);
  }
</style>
