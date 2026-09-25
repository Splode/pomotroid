<script lang="ts">
  /**
   * SVG progress ring with a big centred value.
   *
   * Purely presentational — pass an already-computed 0–1 fraction.
   */
  interface Props {
    /** Fill fraction, 0–1 (values outside the range are clamped). */
    value: number;
    /** Primary text in the middle of the ring. */
    display: string | number;
    /** Optional context line rendered under the display value. */
    caption?: string | null;
    size?: number;
    stroke?: number;
    color?: string;
  }

  let {
    value,
    display,
    caption = null,
    size = 128,
    stroke = 9,
    color = 'var(--color-focus-round)',
  }: Props = $props();

  let clamped = $derived(Math.max(0, Math.min(1, Number.isFinite(value) ? value : 0)));

  let radius = $derived((size - stroke) / 2);
  let circumference = $derived(2 * Math.PI * radius);
  /** Dash offset: full circumference hides the arc entirely. */
  let dashOffset = $derived(circumference * (1 - clamped));
</script>

<div class="ring-wrap" style="width: {size}px; height: {size}px">
  <svg width={size} height={size} viewBox="0 0 {size} {size}" role="img" aria-label="{display} {caption ?? ''}">
    <!-- Track -->
    <circle
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke="color-mix(in oklch, var(--color-foreground) 12%, transparent)"
      stroke-width={stroke}
    />
    <!-- Progress arc, rotated so it starts at 12 o'clock -->
    <circle
      class="arc"
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke={color}
      stroke-width={stroke}
      stroke-linecap="round"
      stroke-dasharray={circumference}
      stroke-dashoffset={dashOffset}
      transform="rotate(-90 {size / 2} {size / 2})"
    />
  </svg>
  <div class="ring-center">
    <span class="ring-value">{display}</span>
    {#if caption}
      <span class="ring-caption">{caption}</span>
    {/if}
  </div>
</div>

<style>
  .ring-wrap {
    position: relative;
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .arc {
    transition: stroke-dashoffset 0.6s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .ring-center {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    pointer-events: none;
  }

  .ring-value {
    font-size: 1.55rem;
    font-weight: 700;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    color: var(--color-foreground);
  }

  .ring-caption {
    font-size: 0.58rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
    text-align: center;
    max-width: 80%;
    line-height: 1.2;
  }
</style>
