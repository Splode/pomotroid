<script lang="ts">
  interface Props {
    value: string;
    onchange: (value: string) => void;
    onblur?: () => void;
  }

  let { value, onchange, onblur }: Props = $props();

  let inputEl = $state<HTMLInputElement | null>(null);
</script>

<input
  bind:this={inputEl}
  class="task-label-input"
  type="text"
  maxlength="48"
  placeholder="what are you working on?"
  {value}
  oninput={(e) => onchange((e.target as HTMLInputElement).value)}
  onkeydown={(e) => { if (e.key === 'Enter') inputEl?.blur(); }}
  onblur={onblur}
  aria-label="Task label"
/>

<style>
  .task-label-input {
    background: none;
    border: none;
    border-bottom: 1px solid transparent;
    border-radius: 0;
    outline: none;
    width: 160px;
    text-align: center;
    font-size: 0.7rem;
    font-weight: 500;
    letter-spacing: 0.04em;
    color: var(--color-foreground);
    padding: 2px 4px;
    margin-top: -8px;
    transition: border-color var(--transition-default), color var(--transition-default);
    caret-color: var(--color-focus-round);
  }

  .task-label-input::placeholder {
    color: color-mix(in oklch, var(--color-foreground-darker) 50%, transparent);
    font-style: italic;
  }

  .task-label-input:focus {
    border-bottom-color: color-mix(in oklch, var(--color-foreground-darker) 40%, transparent);
  }
</style>
