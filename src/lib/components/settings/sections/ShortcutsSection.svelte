<script lang="ts">
  import { onMount } from 'svelte';
  import { settings } from '$lib/stores/settings';
  import { setSetting, reloadShortcuts, accessibilityTrusted } from '$lib/ipc';
  import ShortcutInput from '$lib/components/ShortcutInput.svelte';
  import * as m from '$paraglide/messages.js';
  import { isMac } from '$lib/utils/platform';
  import { openUrl } from '@tauri-apps/plugin-opener';

  const ACCESSIBILITY_URL = 'x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility';

  let trusted = $state(true);

  async function checkTrusted() {
    trusted = await accessibilityTrusted();
  }

  onMount(() => {
    if (!isMac) return;

    checkTrusted();

    function onFocus() {
      if (!trusted) checkTrusted();
    }

    window.addEventListener('focus', onFocus);
    return () => window.removeEventListener('focus', onFocus);
  });

  async function setShortcut(dbKey: string, value: string) {
    const updated = await setSetting(dbKey, value);
    settings.set(updated);
    await reloadShortcuts();
  }
</script>

<div class="section">
  {#if isMac && !trusted}
    <div class="accessibility-notice">
      <p class="notice-text">{m.shortcuts_accessibility_notice()}</p>
      <button class="notice-btn" onclick={() => openUrl(ACCESSIBILITY_URL)}>
        {m.shortcuts_accessibility_open()}
      </button>
    </div>
  {/if}

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

  .accessibility-notice {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin: 8px 16px;
    padding: 10px 14px;
    border: 1px solid color-mix(in oklch, var(--color-foreground-darker) 35%, transparent);
    border-radius: 6px;
    background: color-mix(in oklch, var(--color-foreground-darker) 8%, transparent);
  }

  .notice-text {
    font-size: 0.75rem;
    color: var(--color-foreground-darker);
    line-height: 1.5;
    margin: 0;
  }

  .notice-btn {
    align-self: flex-start;
    padding: 4px 10px;
    font-size: 0.75rem;
    color: var(--color-foreground);
    background: color-mix(in oklch, var(--color-foreground) 10%, transparent);
    border: 1px solid color-mix(in oklch, var(--color-foreground) 20%, transparent);
    border-radius: 4px;
    cursor: pointer;
    transition: background var(--transition-default);
  }

  .notice-btn:hover {
    background: color-mix(in oklch, var(--color-foreground) 18%, transparent);
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
