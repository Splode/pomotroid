<script lang="ts">
  // Captures a keyboard combination and formats it as a shortcut string
  // matching Rust's parse_shortcut format (e.g. "Control+F1", "Shift+Alt+A").

  interface Props {
    value?: string;
    onchange?: (value: string) => void;
  }

  let { value = '', onchange }: Props = $props();

  let listening = $state(false);

  function codeToKey(code: string): string | null {
    if (code.startsWith('Key')) return code.slice(3); // "KeyA" → "A"
    if (code.startsWith('Digit')) return code.slice(5); // "Digit1" → "1"
    if (/^F([1-9]|1[0-2])$/.test(code)) return code; // "F1"–"F12"
    if (code === 'Space') return 'Space';
    if (code === 'Enter') return 'Enter';
    if (code === 'Escape') return 'Escape';
    if (code === 'ArrowLeft') return 'Left';
    if (code === 'ArrowRight') return 'Right';
    if (code === 'ArrowUp') return 'Up';
    if (code === 'ArrowDown') return 'Down';
    return null;
  }

  function onKeydown(e: KeyboardEvent) {
    if (!listening) return;
    e.preventDefault();
    e.stopPropagation();

    // Ignore bare modifier keys
    if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) return;

    const key = codeToKey(e.code);
    if (!key) return;

    const parts: string[] = [];
    if (e.ctrlKey) parts.push('Control');
    if (e.shiftKey) parts.push('Shift');
    if (e.altKey) parts.push('Alt');
    if (e.metaKey) parts.push('Super');
    parts.push(key);

    const shortcut = parts.join('+');
    onchange?.(shortcut);
    listening = false;
  }

  function onFocus() {
    listening = true;
  }

  function onBlur() {
    listening = false;
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<input
  class="shortcut-input"
  class:listening
  readonly
  {value}
  placeholder="Click and press keys…"
  onfocus={onFocus}
  onblur={onBlur}
  onkeydown={onKeydown}
  role="textbox"
  tabindex="0"
/>

<style>
  .shortcut-input {
    background: var(--color-background);
    border: 1px solid transparent;
    border-radius: 4px;
    color: var(--color-foreground);
    cursor: pointer;
    font-size: 0.75rem;
    font-family: 'Mona Sans Mono', monospace;
    padding: 4px 8px;
    text-align: center;
    outline: none;
    width: 140px;
    transition: border-color 0.15s;
  }

  .shortcut-input.listening {
    border-color: var(--color-accent);
    cursor: text;
  }

  .shortcut-input::placeholder {
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.5;
    font-family: system-ui, sans-serif;
    font-size: 0.7rem;
  }
</style>
