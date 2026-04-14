<script lang="ts">
  import { tick, type Snippet } from "svelte";

  interface Props {
    text: string;
    delay?: number;
    placement?: "above" | "below";
    children: Snippet;
  }

  let { text, delay = 600, placement = "above", children }: Props = $props();

  let visible = $state(false);
  let positioned = $state(false);
  // Computed during updatePosition(); falls back to the placement prop when hidden.
  let flipped = $state(false);
  let actualPlacement = $derived(flipped ? "below" : placement);
  let tooltipStyle = $state("");
  let timer: ReturnType<typeof setTimeout> | undefined;
  let wrapper = $state<HTMLSpanElement | undefined>(undefined);
  let tooltipEl = $state<HTMLSpanElement | undefined>(undefined);
  const tooltipId = `tooltip-${Math.random().toString(36).slice(2, 9)}`;

  async function show() {
    // Already showing or timer running — nothing to do.
    if (visible || timer !== undefined) return;
    const run = async () => {
      timer = undefined;
      visible = true;
      await tick();
      updatePosition();
    };
    if (delay === 0) {
      await run();
    } else {
      timer = setTimeout(run, delay);
    }
  }

  function hide() {
    clearTimeout(timer);
    timer = undefined;
    visible = false;
    positioned = false;
    flipped = false;
    tooltipStyle = "";
  }

  function updatePosition() {
    if (!wrapper || !tooltipEl) return;
    const wRect = wrapper.getBoundingClientRect();
    const tRect = tooltipEl.getBoundingClientRect();
    const centerX = wRect.left + wRect.width / 2;
    const pad = 8;

    // Vertical: flip if too close to top edge.
    flipped = wRect.top < 70 && placement === "above";

    let top: number;
    if (actualPlacement === "below") {
      top = wRect.bottom + 8;
    } else {
      top = wRect.top - 8 - tRect.height;
    }

    // Horizontal: center on trigger, clamped within viewport.
    let left = centerX - tRect.width / 2;
    if (left < pad) left = pad;
    if (left + tRect.width > window.innerWidth - pad) {
      left = window.innerWidth - pad - tRect.width;
    }

    // Arrow offset: always points at the trigger's horizontal center.
    const arrowLeft = centerX - left;
    tooltipStyle = `top:${top}px;left:${left}px;--arrow-left:${arrowLeft}px`;
    positioned = true;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span
  class="tooltip-wrapper"
  bind:this={wrapper}
  onpointerenter={show}
  onpointermove={show}
  onpointerleave={hide}
  aria-describedby={visible ? tooltipId : undefined}
>
  {@render children()}
  {#if visible}
    <span
      bind:this={tooltipEl}
      class="tooltip"
      class:below={actualPlacement === "below"}
      class:positioned
      style={tooltipStyle}
      id={tooltipId}
      role="tooltip">{text}</span
    >
  {/if}
</span>

<style>
  .tooltip-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    aspect-ratio: 1;
  }

  .tooltip {
    --tooltip-bg: var(
      --color-background-light,
      color-mix(in oklch, var(--color-foreground) 10%, var(--color-background))
    );
    position: fixed;
    background: var(--tooltip-bg);
    color: var(--color-foreground);
    font-size: 0.72rem;
    line-height: 1.4;
    padding: 5px 9px;
    border-radius: 4px;
    width: max-content;
    max-width: 240px;
    white-space: normal;
    text-align: center;
    pointer-events: none;
    z-index: 9999;
    box-shadow: 0 2px 8px color-mix(in oklch, black 30%, transparent);
    border: 1px solid
      color-mix(in oklch, var(--color-foreground) 12%, transparent);
    /* Hidden until JS positions it to avoid a 1-frame flash at top-left. */
    visibility: hidden;
  }

  .tooltip.positioned {
    visibility: visible;
  }

  /* Arrow pointing down — tooltip is above the trigger */
  .tooltip::after {
    content: "";
    position: absolute;
    top: 100%;
    left: var(--arrow-left, 50%);
    transform: translateX(-50%);
    border: 5px solid transparent;
    border-top-color: var(--tooltip-bg);
  }

  /* Arrow pointing up — tooltip is below the trigger */
  .tooltip.below::after {
    top: auto;
    bottom: 100%;
    border-top-color: transparent;
    border-bottom-color: var(--tooltip-bg);
  }
</style>
