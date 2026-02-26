<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';

  let localPort = $state(String($settings.websocket_port));
  $effect(() => { localPort = String($settings.websocket_port); });

  async function toggle(dbKey: string, current: boolean) {
    const updated = await setSetting(dbKey, current ? 'false' : 'true');
    settings.set(updated);
  }

  async function handlePortBlur() {
    const port = parseInt(localPort, 10);
    if (!isNaN(port) && port >= 1024 && port <= 65535) {
      const updated = await setSetting('websocket_port', String(port));
      settings.set(updated);
    } else {
      localPort = String($settings.websocket_port);
    }
  }
</script>

<div class="section">
  <div class="group-heading">Window</div>

  <SettingsToggle
    label="Always on Top"
    description="Keep the timer window above all other windows."
    checked={$settings.always_on_top}
    onclick={() => toggle('always_on_top', $settings.always_on_top)}
  />
  {#if $settings.always_on_top}
    <SettingsToggle
      label="Lower Priority During Breaks"
      description="Disable always-on-top while a break is running."
      checked={$settings.break_always_on_top}
      onclick={() => toggle('break_always_on_top', $settings.break_always_on_top)}
    />
  {/if}

  <div class="group-heading">System Tray</div>

  <SettingsToggle
    label="Minimize to Tray"
    description="Show the tray icon and hide the window when minimized."
    checked={$settings.min_to_tray}
    onclick={() => toggle('min_to_tray', $settings.min_to_tray)}
  />
  <SettingsToggle
    label="Close to Tray"
    description="Hide to the tray instead of quitting when the window is closed."
    checked={$settings.min_to_tray_on_close}
    onclick={() => toggle('min_to_tray_on_close', $settings.min_to_tray_on_close)}
  />

  <div class="group-heading">Integrations</div>

  <SettingsToggle
    label="WebSocket Server"
    description="Expose a local WebSocket endpoint for external integrations (port {$settings.websocket_port})."
    checked={$settings.websocket_enabled}
    onclick={() => toggle('websocket_enabled', $settings.websocket_enabled)}
  />

  {#if $settings.websocket_enabled}
    <div class="row">
      <span class="label">Port</span>
      <input
        class="port-input"
        type="number"
        min="1024"
        max="65535"
        bind:value={localPort}
        onblur={handlePortBlur}
      />
    </div>
    <p class="note">
      Listens on <code>ws://127.0.0.1:{$settings.websocket_port}/ws</code>.
      Send <code>{"{ \"type\": \"getState\" }"}</code> to query the current timer state.
      Round changes are broadcast automatically.
    </p>
  {/if}
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .group-heading {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    margin: 0;
    padding: 16px 20px 6px;
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

  .port-input {
    background: var(--color-hover);
    border: 1px solid color-mix(in oklch, var(--color-foreground) 18%, transparent);
    border-radius: 4px;
    color: var(--color-foreground);
    font-size: 0.85rem;
    font-family: monospace;
    padding: 4px 10px;
    width: 90px;
    text-align: right;
    outline: none;
    transition: border-color 0.15s;
  }

  .port-input:focus {
    border-color: var(--color-accent);
  }

  .note {
    font-size: 0.75rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.65;
    padding: 16px 20px;
    line-height: 1.6;
  }

  code {
    font-family: monospace;
    font-size: 0.75em;
    background: color-mix(in oklch, var(--color-foreground) 12%, transparent);
    padding: 1px 5px;
    border-radius: 3px;
  }
</style>
