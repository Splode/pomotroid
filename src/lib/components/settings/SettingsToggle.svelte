<script lang="ts">
  // A labelled toggle-switch row. Click anywhere on the row to toggle.
  interface Props {
    label: string;
    checked: boolean;
    description?: string;
    onclick: () => void;
  }

  let { label, checked, description, onclick }: Props = $props();
</script>

<button class="row" {onclick}>
  <span class="text">
    <span class="label">{label}</span>
    {#if description}
      <span class="desc">{description}</span>
    {/if}
  </span>
  <span class="toggle" class:on={checked} aria-checked={checked} role="switch"></span>
</button>

<style>
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 20px;
    background: none;
    border: none;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    cursor: pointer;
    text-align: left;
    gap: 16px;
    transition: background 0.12s;
  }

  .row:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }

  .desc {
    font-size: 0.72rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    letter-spacing: 0.02em;
    opacity: 0.7;
  }

  /* Pill toggle */
  .toggle {
    position: relative;
    width: 34px;
    height: 18px;
    border-radius: 9px;
    /* Use a foreground-tinted track so it's visible on both dark and light themes
       (rgba(255,255,255,0.15) vanishes on light backgrounds). */
    background: color-mix(in oklch, var(--color-foreground) 22%, transparent);
    flex-shrink: 0;
    transition: background 0.2s;
  }

  .toggle::after {
    content: '';
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    /* Auto-contrast thumb: dark on light backgrounds, light on dark backgrounds. */
    background: oklch(from var(--color-background) clamp(0, (0.6 - l) * 9999, 1) 0 0);
    top: 3px;
    left: 3px;
    transition: transform 0.2s, background 0.2s;
  }

  .toggle.on {
    background: var(--color-accent);
  }

  .toggle.on::after {
    transform: translateX(16px);
    /* Auto-contrast thumb against the accent: dark thumb on bright accents
       (e.g. Gruvbox yellow #FABD2F, neon green), white on dark accents.
       oklch L≈0 is black, L≈1 is white; threshold 0.6 is the equal-contrast point. */
    background: oklch(from var(--color-accent) clamp(0, (0.6 - l) * 9999, 1) 0 0);
  }
</style>
