<script lang="ts">
  // App settings: boolean toggles and global shortcuts.
  import { settings } from '$lib/stores/settings';
  import { setSetting, reloadShortcuts } from '$lib/ipc';
  import ShortcutInput from '$lib/components/ShortcutInput.svelte';

  async function toggle(dbKey: string, current: boolean) {
    const updated = await setSetting(dbKey, current ? 'false' : 'true');
    settings.set(updated);
  }

  async function setShortcut(dbKey: string, value: string) {
    const updated = await setSetting(dbKey, value);
    settings.set(updated);
    await reloadShortcuts();
  }
</script>

<div class="panel">
  <p class="heading">Settings</p>

  <button class="row" onclick={() => toggle('always_on_top', $settings.always_on_top)}>
    <span class="label">Always On Top</span>
    <span class="checkbox" class:checked={$settings.always_on_top}></span>
  </button>

  {#if $settings.always_on_top}
    <button class="row" onclick={() => toggle('break_always_on_top', $settings.break_always_on_top)}>
      <span class="label">Disable On Top During Breaks</span>
      <span class="checkbox" class:checked={$settings.break_always_on_top}></span>
    </button>
  {/if}

  <button class="row" onclick={() => toggle('auto_start_work', $settings.auto_start_work)}>
    <span class="label">Auto-start Work Timer</span>
    <span class="checkbox" class:checked={$settings.auto_start_work}></span>
  </button>

  <button class="row" onclick={() => toggle('auto_start_break', $settings.auto_start_break)}>
    <span class="label">Auto-start Break Timer</span>
    <span class="checkbox" class:checked={$settings.auto_start_break}></span>
  </button>

  <button class="row" onclick={() => toggle('tick_sounds_work', $settings.tick_sounds_during_work)}>
    <span class="label">Tick Sounds — Work</span>
    <span class="checkbox" class:checked={$settings.tick_sounds_during_work}></span>
  </button>

  <button class="row" onclick={() => toggle('tick_sounds_break', $settings.tick_sounds_during_break)}>
    <span class="label">Tick Sounds — Break</span>
    <span class="checkbox" class:checked={$settings.tick_sounds_during_break}></span>
  </button>

  <button class="row" onclick={() => toggle('notifications', $settings.notifications_enabled)}>
    <span class="label">Desktop Notifications</span>
    <span class="checkbox" class:checked={$settings.notifications_enabled}></span>
  </button>

  <button class="row" onclick={() => toggle('min_to_tray', $settings.min_to_tray)}>
    <span class="label">Minimize to Tray</span>
    <span class="checkbox" class:checked={$settings.min_to_tray}></span>
  </button>

  <button class="row" onclick={() => toggle('min_to_tray_on_close', $settings.min_to_tray_on_close)}>
    <span class="label">Minimize to Tray on Close</span>
    <span class="checkbox" class:checked={$settings.min_to_tray_on_close}></span>
  </button>

  <p class="heading" style="margin-top: 16px;">Global Shortcuts</p>

  <div class="row row--shortcut">
    <span class="label">Toggle Timer</span>
    <ShortcutInput
      value={$settings.shortcut_toggle}
      onchange={(v) => setShortcut('shortcut_toggle', v)}
    />
  </div>

  <div class="row row--shortcut">
    <span class="label">Reset Timer</span>
    <ShortcutInput
      value={$settings.shortcut_reset}
      onchange={(v) => setShortcut('shortcut_reset', v)}
    />
  </div>

  <div class="row row--shortcut">
    <span class="label">Skip Round</span>
    <ShortcutInput
      value={$settings.shortcut_skip}
      onchange={(v) => setShortcut('shortcut_skip', v)}
    />
  </div>
</div>

<style>
  .panel {
    padding: 8px 12px;
    overflow-y: auto;
    height: 100%;
  }

  .heading {
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    margin-bottom: 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--color-separator);
  }

  .row {
    background: var(--color-background);
    border: none;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 6px 0;
    padding: 10px 12px;
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: background 0.15s;
  }

  .row:hover {
    background: color-mix(in oklch, var(--color-background) 88%, var(--color-foreground) 12%);
  }

  .row--shortcut {
    cursor: default;
  }

  .row--shortcut:hover {
    background: var(--color-background);
  }

  .label {
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    letter-spacing: 0.04em;
  }

  .checkbox {
    display: inline-block;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid color-mix(in oklch, var(--color-foreground) 25%, transparent);
    background: var(--color-background);
    flex-shrink: 0;
    transition: background 0.15s, border-color 0.15s;
  }

  .checkbox.checked {
    background: var(--color-accent);
    border-color: var(--color-background);
  }

  .row:hover .checkbox:not(.checked) {
    border-color: var(--color-accent);
  }
</style>
