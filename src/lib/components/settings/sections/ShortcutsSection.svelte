<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting, reloadShortcuts } from '$lib/ipc';
  import ShortcutInput from '$lib/components/ShortcutInput.svelte';
  import * as m from '$paraglide/messages.js';

  async function setShortcut(dbKey: string, value: string) {
    const updated = await setSetting(dbKey, value);
    settings.set(updated);
    await reloadShortcuts();
  }
</script>

<div class="section">
  <p class="note">{m.shortcuts_note()}</p>

  <div class="row">
    <span class="label">{m.shortcuts_toggle_timer()}</span>
    <ShortcutInput
      value={$settings.shortcut_toggle}
      onchange={(v) => setShortcut('shortcut_toggle', v)}
    />
  </div>

  <div class="row">
    <span class="label">{m.shortcuts_reset_timer()}</span>
    <ShortcutInput
      value={$settings.shortcut_reset}
      onchange={(v) => setShortcut('shortcut_reset', v)}
    />
  </div>

  <div class="row">
    <span class="label">{m.shortcuts_skip_round()}</span>
    <ShortcutInput
      value={$settings.shortcut_skip}
      onchange={(v) => setShortcut('shortcut_skip', v)}
    />
  </div>

  <div class="row">
    <span class="label">{m.shortcuts_restart_round()}</span>
    <ShortcutInput
      value={$settings.shortcut_restart}
      onchange={(v) => setShortcut('shortcut_restart', v)}
    />
  </div>
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    padding: 8px 0;
  }

  .note {
    font-size: 0.75rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.65;
    padding: 8px 20px 16px;
    line-height: 1.5;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 20px;
    border-bottom: 1px solid var(--color-separator);
  }

  .label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }
</style>
