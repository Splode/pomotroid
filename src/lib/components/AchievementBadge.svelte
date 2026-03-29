<script lang="ts">
  import { FLUENT_EMOJI } from '$lib/fluentEmoji';

  interface Props {
    color: string | null;
    emoji?: string;
    earned?: boolean;
    locked?: boolean;
    size?: number;
  }

  let { color, emoji = '', earned = true, locked = false, size = 28 }: Props = $props();

  const center = $derived(size / 2);
  const iconSize = $derived(Math.round(size * 0.45));
  const fluentSrc = $derived(emoji ? (FLUENT_EMOJI[emoji] ?? null) : null);
</script>

<span class="badge-wrapper" class:unearned={!earned}>
  {#if locked}
    <svg
      width={size}
      height={size}
      viewBox="0 0 {size} {size}"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <!-- Lock shackle -->
      <path
        d="M {center - iconSize * 0.18} {center - iconSize * 0.04}
           v -{iconSize * 0.22}
           a {iconSize * 0.18} {iconSize * 0.18} 0 0 1 {iconSize * 0.36} 0
           v {iconSize * 0.22}"
        fill="none"
        stroke="var(--color-foreground-darker, #888)"
        stroke-width={iconSize * 0.11}
        stroke-linecap="round"
      />
      <!-- Lock body -->
      <rect
        x={center - iconSize * 0.28}
        y={center - iconSize * 0.06}
        width={iconSize * 0.56}
        height={iconSize * 0.42}
        rx={iconSize * 0.08}
        fill="var(--color-foreground-darker, #888)"
      />
    </svg>
  {:else if fluentSrc}
    <img src={fluentSrc} width={size} height={size} alt={emoji} draggable="false" />
  {:else if emoji}
    <!-- Fallback: OS emoji for any unmapped characters -->
    <svg
      width={size}
      height={size}
      viewBox="0 0 {size} {size}"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <text
        x={center}
        y={center + iconSize * 0.35}
        text-anchor="middle"
        font-size={iconSize}
        dominant-baseline="auto"
      >{emoji}</text>
    </svg>
  {/if}
</span>

<style>
  .badge-wrapper {
    display: inline-flex;
    flex-shrink: 0;
  }

  .badge-wrapper.unearned {
    filter: grayscale(1) opacity(0.4);
  }

  img {
    display: block;
    pointer-events: none;
    user-select: none;
  }
</style>
