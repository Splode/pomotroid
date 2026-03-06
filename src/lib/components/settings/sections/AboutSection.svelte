<script lang="ts">
  import { onMount } from 'svelte';
  import { openUrl } from '@tauri-apps/plugin-opener';

  import { info, warn, error as logError } from '@tauri-apps/plugin-log';
  import { openLogDir, appVersion, resetSettings, checkUpdate, installUpdate } from '$lib/ipc';
  import { settings } from '$lib/stores/settings';
  import type { UpdateInfo } from '$lib/types';
  import * as m from '$paraglide/messages.js';

  const BASE_VERSION = '1.0.0';
  const REPO = 'https://github.com/Splode/pomotroid';

  let version = $state('...');
  let confirming = $state(false);

  type UpdateState = 'idle' | 'checking' | 'up-to-date' | 'available' | 'installing' | 'error';
  let updateState = $state<UpdateState>('idle');
  let availableUpdate = $state<UpdateInfo | null>(null);
  let updateError = $state('');

  // Strip pre-release and build metadata to get the bare X.Y.Z for the release tag URL.
  function baseOnly(v: string): string {
    return v.split('-')[0].split('+')[0];
  }

  let releaseUrl = $derived(`${REPO}/releases/tag/v${baseOnly(version === '...' ? BASE_VERSION : version)}`);

  onMount(async () => {
    try {
      version = await appVersion();
    } catch {
      version = BASE_VERSION;
    }

    if ($settings.check_for_updates) {
      updateState = 'checking';
      await info('[about] checking for updates');
      try {
        const update = await checkUpdate();
        if (update) {
          availableUpdate = update;
          updateState = 'available';
          await info(`[about] update available: v${update.version}`);
        } else {
          updateState = 'up-to-date';
          await info('[about] already up to date');
        }
      } catch (e) {
        updateError = String(e);
        updateState = 'error';
        await warn(`[about] update check failed: ${e}`);
      }
    }
  });

  async function handleInstall() {
    updateState = 'installing';
    await info(`[about] installing update v${availableUpdate?.version}`);
    try {
      await installUpdate();
    } catch (e) {
      updateError = String(e);
      updateState = 'error';
      await logError(`[about] update install failed: ${e}`);
    }
  }

  async function handleReset() {
    const updated = await resetSettings();
    settings.set(updated);
    confirming = false;
  }
</script>

<div class="section">
  <div class="hero">
    <svg
      version="1.2"
      baseProfile="tiny"
      width="64"
      height="64"
      viewBox="0 0 256 256"
      xmlns="http://www.w3.org/2000/svg"
      aria-label="Pomotroid logo"
    >
      <circle fill="var(--color-background-light)" cx="128" cy="128" r="126.81"/>
      <circle
        fill="none"
        stroke="var(--color-focus-round)"
        stroke-width="40"
        stroke-linecap="round"
        cx="128"
        cy="128"
        r="73.31"
      />
    </svg>
    <div>
      <h2 class="name">Pomotroid</h2>
      <p class="version">Version {version}</p>
    </div>
  </div>

  <div class="links">
    <button class="link-row" onclick={() => openUrl(releaseUrl)}>
      <span>{m.about_release_notes()}</span>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2 10L10 2M10 2H4M10 2V8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button class="link-row" onclick={() => openUrl(REPO)}>
      <span>{m.about_source_code()}</span>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2 10L10 2M10 2H4M10 2V8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button class="link-row" onclick={openLogDir}>
      <span>{m.about_open_log_folder()}</span>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M1 3.5C1 2.67 1.67 2 2.5 2H5l1 1.5H9.5C10.33 3.5 11 4.17 11 5v4.5C11 10.33 10.33 11 9.5 11h-7C1.67 11 1 10.33 1 9.5V3.5Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>

  {#if $settings.check_for_updates || updateState !== 'idle'}
    <div class="update-group">
      {#if updateState === 'idle' || updateState === 'checking'}
        <div class="update-row update-row--muted">
          <span>{m.about_update_checking()}</span>
        </div>
      {:else if updateState === 'up-to-date'}
        <div class="update-row update-row--muted">
          <span>{m.about_update_up_to_date()}</span>
        </div>
      {:else if updateState === 'available' && availableUpdate}
        <button class="update-row update-row--action" onclick={handleInstall}>
          <span>{m.about_update_install({ version: availableUpdate.version })}</span>
        </button>
      {:else if updateState === 'installing'}
        <div class="update-row update-row--muted">
          <span>Installing…</span>
        </div>
      {:else if updateState === 'error'}
        <div class="update-row update-row--muted">
          <span>{m.about_update_error()}</span>
        </div>
      {/if}
    </div>
  {/if}

  <div class="reset-group">
    {#if !confirming}
      <button class="reset-row" onclick={() => (confirming = true)}>
        <span>{m.about_reset_all()}</span>
      </button>
    {:else}
      <div class="confirm-row">
        <span class="confirm-label">{m.about_reset_confirm()}</span>
        <div class="confirm-actions">
          <button class="confirm-cancel" onclick={() => (confirming = false)}>Cancel</button>
          <button class="confirm-reset" onclick={handleReset}>Reset</button>
        </div>
      </div>
    {/if}
  </div>

  <p class="credit">
    Built with Tauri, Svelte, and Rust.<br/>
    MIT License — Copyright &copy; 2017–2026 Christopher Murphy.
  </p>
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    padding: 32px 20px;
    gap: 24px;
  }

  .hero {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .name {
    font-size: 1.1rem;
    font-weight: 400;
    color: var(--color-short-round);
    letter-spacing: 0.04em;
    margin-bottom: 4px;
  }

  .version {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    letter-spacing: 0.04em;
  }

  .links {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .link-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-separator);
    cursor: pointer;
    color: var(--color-foreground);
    font-size: 0.85rem;
    letter-spacing: 0.02em;
    text-align: left;
    transition: background 0.12s;
  }

  .link-row:last-child {
    border-bottom: none;
  }

  .link-row:hover {
    background: var(--color-hover);
    color: var(--color-accent);
  }

  .update-group {
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .update-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    font-size: 0.85rem;
    letter-spacing: 0.02em;
  }

  .update-row--muted {
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.65;
  }

  .update-row--action {
    width: 100%;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-accent);
    text-align: left;
    transition: background 0.12s;
  }

  .update-row--action:hover {
    background: var(--color-hover);
  }

  .reset-group {
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .reset-row {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px 16px;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.85rem;
    letter-spacing: 0.02em;
    text-align: left;
    transition: background 0.12s, color 0.12s;
  }

  .reset-row:hover {
    background: var(--color-hover);
    color: color-mix(in oklch, var(--color-focus-round) 80%, var(--color-foreground));
  }

  .confirm-row {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px 16px;
  }

  .confirm-label {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.8;
  }

  .confirm-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .confirm-cancel,
  .confirm-reset {
    background: none;
    border: 1px solid color-mix(in oklch, var(--color-foreground) 18%, transparent);
    border-radius: 4px;
    font-size: 0.8rem;
    padding: 5px 14px;
    cursor: pointer;
    transition: border-color 0.15s, color 0.15s;
  }

  .confirm-cancel {
    color: var(--color-foreground-darker, var(--color-foreground));
  }

  .confirm-cancel:hover {
    border-color: color-mix(in oklch, var(--color-foreground) 40%, transparent);
    color: var(--color-foreground);
  }

  .confirm-reset {
    color: var(--color-accent);
    border-color: color-mix(in oklch, var(--color-accent) 40%, transparent);
  }

  .confirm-reset:hover {
    background: color-mix(in oklch, var(--color-accent) 10%, transparent);
    border-color: var(--color-accent);
  }

  .credit {
    font-size: 0.72rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.5;
    line-height: 1.6;
  }

</style>
