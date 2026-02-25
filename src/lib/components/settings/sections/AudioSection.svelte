<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';

  let localVolume = $state($settings.volume);
  $effect(() => { localVolume = $settings.volume; });

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
</style>
