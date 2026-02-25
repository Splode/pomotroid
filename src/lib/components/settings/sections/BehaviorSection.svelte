<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';

  async function toggle(dbKey: string, current: boolean) {
    const updated = await setSetting(dbKey, current ? 'false' : 'true');
    settings.set(updated);
  }
</script>

<div class="section">
  <p class="group-heading">Timer</p>

  <SettingsToggle
    label="Auto-start Work"
    description="Automatically begin the next work session after a break ends."
    checked={$settings.auto_start_work}
    onclick={() => toggle('auto_start_work', $settings.auto_start_work)}
  />
  <SettingsToggle
    label="Auto-start Breaks"
    description="Automatically begin a break when a work session ends."
    checked={$settings.auto_start_break}
    onclick={() => toggle('auto_start_break', $settings.auto_start_break)}
  />

  <p class="group-heading">Window</p>

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

  <p class="group-heading">System Tray</p>

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

  <p class="group-heading">Notifications</p>

  <SettingsToggle
    label="Desktop Notifications"
    description="Show a system notification when each round ends."
    checked={$settings.notifications_enabled}
    onclick={() => toggle('notifications', $settings.notifications_enabled)}
  />
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .group-heading {
    font-size: 0.68rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    padding: 16px 20px 6px;
  }
</style>
