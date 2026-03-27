<script lang="ts">
  import Tooltip from './Tooltip.svelte';

  interface Props {
    text: string;
  }

  let { text }: Props = $props();
</script>

<Tooltip {text} delay={0}>
  <!-- stopPropagation prevents the info icon from firing a parent button's onclick.
       Must be a <span> (not <button>) to be valid phrasing content inside <button>. -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <span
    class="info-icon"
    role="img"
    aria-label="More information"
    onclick={(e) => e.stopPropagation()}
  >
    <svg width="13" height="13" viewBox="0 0 13 13" fill="none" aria-hidden="true">
      <circle cx="6.5" cy="6.5" r="5.75" stroke="currentColor" stroke-width="1.5"/>
      <circle cx="6.5" cy="4" r="0.9" fill="currentColor"/>
      <line x1="6.5" y1="6" x2="6.5" y2="10" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
    </svg>
  </span>
</Tooltip>

<style>
  .info-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--color-foreground);
    cursor: default;
    margin-left: 5px;
    opacity: 0.55;
    transition: opacity 0.15s;
    user-select: none;
    flex-shrink: 0;
  }

  .info-icon:hover {
    opacity: 1;
  }
</style>
