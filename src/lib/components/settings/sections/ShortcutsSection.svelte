<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting, reloadShortcuts } from '$lib/ipc';
  import ShortcutInput from '$lib/components/ShortcutInput.svelte';

  async function setShortcut(dbKey: string, value: string) {
    const updated = await setSetting(dbKey, value);
    settings.set(updated);
    await reloadShortcuts();
  }
</script>

<div class="section">
  <p class="note">
    Global shortcuts work even when the window is not focused.
    Click a field and press your desired key combination to record it.
  </p>

  <div class="row">
    <span class="label">Toggle Timer</span>
    <ShortcutInput
      value={$settings.shortcut_toggle}
      onchange={(v) => setShortcut('shortcut_toggle', v)}
    />
  </div>

  <div class="row">
    <span class="label">Reset Timer</span>
    <ShortcutInput
      value={$settings.shortcut_reset}
      onchange={(v) => setShortcut('shortcut_reset', v)}
    />
  </div>

  <div class="row">
    <span class="label">Skip Round</span>
    <ShortcutInput
      value={$settings.shortcut_skip}
      onchange={(v) => setShortcut('shortcut_skip', v)}
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
    opacity: 0.7;
    padding: 8px 20px 16px;
    line-height: 1.5;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }
</style>
