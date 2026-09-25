<script lang="ts">
  /**
   * Ranked horizontal bars — good for short, labelled comparisons such as
   * "focus by weekday" or "rounds by length".
   */
  interface Row {
    label: string;
    value: number;
    /** Pre-formatted value shown at the right edge. */
    display: string;
    /** Optional secondary line under the label. */
    sub?: string | null;
    color?: string;
    /** Render a muted bar even when the value is zero. */
    muted?: boolean;
  }

  interface Props {
    rows: Row[];
    /** Fixed maximum for the bar scale; defaults to the largest value. */
    max?: number | null;
  }

  let { rows, max = null }: Props = $props();

  let scale = $derived(max ?? Math.max(1, ...rows.map((r) => r.value)));
</script>

<ul class="hbar-list">
  {#each rows as row (row.label)}
    <li class="row">
      <span class="label">
        <span class="label-main">{row.label}</span>
        {#if row.sub}
          <span class="label-sub">{row.sub}</span>
        {/if}
      </span>
      <span class="track">
        <span
          class="fill"
          class:empty={row.value === 0}
          style="width: {Math.max(1, (row.value / scale) * 100)}%; background: {row.color ??
            'var(--color-focus-round)'}"
        ></span>
      </span>
      <span class="value">{row.display}</span>
    </li>
  {/each}
</ul>

<style>
  .hbar-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 7px;
  }

  .row {
    display: grid;
    grid-template-columns: minmax(58px, auto) 1fr minmax(46px, auto);
    align-items: center;
    gap: 9px;
  }

  .label {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .label-main {
    font-size: 0.68rem;
    color: var(--color-foreground);
    white-space: nowrap;
  }

  .label-sub {
    font-size: 0.56rem;
    color: var(--color-foreground-darker);
    white-space: nowrap;
  }

  .track {
    position: relative;
    height: 7px;
    border-radius: 4px;
    background: color-mix(in oklch, var(--color-foreground) 9%, transparent);
    overflow: hidden;
  }

  .fill {
    display: block;
    height: 100%;
    border-radius: 4px;
    transition: width 0.45s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fill.empty {
    background: color-mix(in oklch, var(--color-foreground) 9%, transparent) !important;
  }

  .value {
    font-size: 0.68rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    text-align: right;
    color: var(--color-foreground);
  }
</style>
