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
    border-bottom: 1px solid var(--color-separator);
    cursor: pointer;
    text-align: left;
    gap: 16px;
    transition: background 0.12s;
  }

  .row:hover {
    background: var(--color-hover);
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
    /* OFF state: use a muted foreground tone so the thumb is visible against
       the subtle foreground-tinted track without being overly bright. */
    background: var(--color-foreground-darker, var(--color-foreground));
    top: 3px;
    left: 3px;
    transition: transform 0.2s, background 0.2s;
  }

  .toggle.on {
    background: var(--color-accent);
  }

  .toggle.on::after {
    transform: translateX(16px);
    /* ON state: the background color is always chosen by the theme designer to
       contrast with the accent, so it reads cleanly and stays within the palette.
       e.g. dark navy on teal (Pomotroid), dark charcoal on yellow (Gruvbox). */
    background: var(--color-background);
  }
</style>
