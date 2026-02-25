<script lang="ts">
  import { onMount } from 'svelte';
  import { settings } from '$lib/stores/settings';
  import { setSetting, getCustomAudioInfo, setCustomAudio, clearCustomAudio, openAudioFilePicker } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';
  import type { CustomAudioInfo } from '$lib/types';

  type CueKey = keyof CustomAudioInfo;

  const CUE_LIST: { id: CueKey; label: string }[] = [
    { id: 'work_alert',       label: 'Work Alert' },
    { id: 'short_break_alert', label: 'Short Break Alert' },
    { id: 'long_break_alert',  label: 'Long Break Alert' },
  ];

  let localVolume = $state($settings.volume);
  $effect(() => { localVolume = $settings.volume; });

  // Custom audio file names (null = built-in default is active).
  let workAlert       = $state<string | null>(null);
  let shortBreakAlert = $state<string | null>(null);
  let longBreakAlert  = $state<string | null>(null);

  function getFileName(id: CueKey): string | null {
    if (id === 'work_alert') return workAlert;
    if (id === 'short_break_alert') return shortBreakAlert;
    return longBreakAlert;
  }

  function setFileName(id: CueKey, val: string | null) {
    if (id === 'work_alert') workAlert = val;
    else if (id === 'short_break_alert') shortBreakAlert = val;
    else longBreakAlert = val;
  }

  onMount(async () => {
    try {
      const info: CustomAudioInfo = await getCustomAudioInfo();
      workAlert       = info.work_alert;
      shortBreakAlert = info.short_break_alert;
      longBreakAlert  = info.long_break_alert;
    } catch (err) {
      console.warn('[audio] getCustomAudioInfo failed (audio unavailable?):', err);
    }
  });

  async function toggle(dbKey: string, current: boolean) {
    const updated = await setSetting(dbKey, current ? 'false' : 'true');
    settings.set(updated);
  }

  async function handleVolumeInput(e: Event) {
    const val = (e.target as HTMLInputElement).valueAsNumber;
    localVolume = val;
    const updated = await setSetting('volume', String(Math.round(val * 100)));
    settings.set(updated);
  }

  async function pickAudio(id: CueKey) {
    let path: string | null = null;
    try {
      path = await openAudioFilePicker();
    } catch (err) {
      console.error('[audio] file picker error:', err);
      return;
    }

    if (!path) return; // dialog was cancelled

    try {
      const displayName = await setCustomAudio(id, path);
      setFileName(id, displayName);
    } catch (err) {
      console.error('[audio] setCustomAudio failed:', err);
    }
  }

  async function restoreAudio(id: CueKey) {
    try {
      await clearCustomAudio(id);
      setFileName(id, null);
    } catch (err) {
      console.error('[audio] clearCustomAudio failed:', err);
    }
  }
</script>

<div class="section">
  <div class="volume-row">
    <div class="volume-meta">
      <span class="label">Volume</span>
      <span class="value">{Math.round(localVolume * 100)}%</span>
    </div>
    <div class="slider-wrap">
      <input
        type="range" min="0" max="1" step="0.01"
        value={localVolume}
        class="slider"
        oninput={handleVolumeInput}
      />
      <div class="bar" style="width: {localVolume * 100}%"></div>
    </div>
  </div>

  <SettingsToggle
    label="Tick Sounds — Work"
    description="Play a ticking sound during work sessions."
    checked={$settings.tick_sounds_during_work}
    onclick={() => toggle('tick_sounds_work', $settings.tick_sounds_during_work)}
  />
  <SettingsToggle
    label="Tick Sounds — Breaks"
    description="Play a ticking sound during break sessions."
    checked={$settings.tick_sounds_during_break}
    onclick={() => toggle('tick_sounds_break', $settings.tick_sounds_during_break)}
  />

  <div class="section-label">Alert Sounds</div>

  {#each CUE_LIST as { id, label } (id)}
    <div class="audio-row">
      <div class="audio-meta">
        <span class="label">{label}</span>
        <span class="file-name" class:custom={getFileName(id) !== null}>
          {getFileName(id) ?? 'Default'}
        </span>
      </div>
      <div class="audio-actions">
        {#if getFileName(id) !== null}
          <button class="btn-restore" onclick={() => restoreAudio(id)}>Restore</button>
        {/if}
        <button class="btn-choose" onclick={() => pickAudio(id)}>Choose File</button>
      </div>
    </div>
  {/each}
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .volume-row {
    padding: 14px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .volume-meta {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 10px;
  }

  .label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }

  .value {
    font-size: 0.8rem;
    font-family: monospace;
    color: var(--color-foreground-darker, var(--color-foreground));
    background: rgba(255, 255, 255, 0.06);
    padding: 2px 8px;
    border-radius: 3px;
  }

  .slider-wrap {
    position: relative;
    height: 20px;
    display: flex;
    align-items: center;
  }

  .slider {
    position: relative;
    z-index: 2;
    width: 100%;
    -webkit-appearance: none;
    appearance: none;
    height: 4px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    outline: none;
    cursor: pointer;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--color-foreground);
    cursor: pointer;
  }

  .slider::-moz-range-thumb {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--color-foreground);
    cursor: pointer;
    border: none;
  }

  .bar {
    position: absolute;
    left: 0;
    height: 4px;
    border-radius: 2px;
    pointer-events: none;
    z-index: 1;
    background: var(--color-accent);
    transition: width 0.05s;
  }

  /* Section label divider */
  .section-label {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.6;
    margin: 0;
    padding: 16px 20px 6px;
  }

  /* Alert sound rows */
  .audio-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .audio-meta {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .file-name {
    font-size: 0.72rem;
    font-family: monospace;
    color: var(--color-foreground-darker, rgba(255, 255, 255, 0.35));
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .file-name.custom {
    color: var(--color-accent);
  }

  .audio-actions {
    display: flex;
    gap: 6px;
    flex-shrink: 0;
  }

  .btn-choose,
  .btn-restore {
    font-size: 0.72rem;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(255, 255, 255, 0.07);
    color: var(--color-foreground);
    transition: background 0.15s;
    white-space: nowrap;
  }

  .btn-choose:hover {
    background: rgba(255, 255, 255, 0.13);
  }

  .btn-restore {
    color: var(--color-foreground-darker, rgba(255, 255, 255, 0.5));
  }

  .btn-restore:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--color-foreground);
  }
</style>
