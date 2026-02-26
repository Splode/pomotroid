<script lang="ts">
  // Timer configuration: work/break durations and long-break interval.
  import { settings } from '$lib/stores/settings';
  import { setSetting, resetSettings } from '$lib/ipc';

  const MAX_MINS = 90;
  const MAX_ROUNDS = 12;

  // Local slider values in minutes (derived from the store).
  let workMins = $derived(Math.round($settings.time_work_secs / 60));
  let shortMins = $derived(Math.round($settings.time_short_break_secs / 60));
  let longMins = $derived(Math.round($settings.time_long_break_secs / 60));
  let rounds = $derived($settings.long_break_interval);

  function pct(val: number, max: number) {
    return (val / max) * 100;
  }

  async function handleChange(dbKey: string, rawValue: number) {
    const updated = await setSetting(dbKey, String(rawValue));
    settings.set(updated);
  }

  async function handleReset() {
    const updated = await resetSettings();
    settings.set(updated);
  }
</script>

<div class="panel">
  <p class="heading">Timer</p>

  <!-- Focus -->
  <div class="row">
    <p class="label">Focus</p>
    <p class="value">{workMins}:00</p>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={workMins}
        class="slider"
        oninput={(e) => handleChange('time_work_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--focus" style="width: {pct(workMins, MAX_MINS)}%"></div>
    </div>
  </div>

  <!-- Short Break -->
  <div class="row">
    <p class="label">Short Break</p>
    <p class="value">{shortMins}:00</p>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={shortMins}
        class="slider"
        oninput={(e) => handleChange('time_short_break_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--short" style="width: {pct(shortMins, MAX_MINS)}%"></div>
    </div>
  </div>

  <!-- Long Break -->
  <div class="row">
    <p class="label">Long Break</p>
    <p class="value">{longMins}:00</p>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={longMins}
        class="slider"
        oninput={(e) => handleChange('time_long_break_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--long" style="width: {pct(longMins, MAX_MINS)}%"></div>
    </div>
  </div>

  <!-- Rounds -->
  <div class="row">
    <p class="label">Rounds</p>
    <p class="value">{rounds}</p>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_ROUNDS} step="1"
        value={rounds}
        class="slider"
        oninput={(e) => handleChange('work_rounds', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--rounds" style="width: {pct(rounds, MAX_ROUNDS)}%"></div>
    </div>
  </div>

  <!-- Reset -->
  <div class="row row--center">
    <button class="text-btn" onclick={handleReset}>Reset Defaults</button>
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
    margin: 10px 0;
    text-align: center;
  }

  .row--center {
    margin-top: 16px;
  }

  .label {
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    letter-spacing: 0.05em;
    margin-bottom: 4px;
  }

  .value {
    background: var(--color-background);
    border-radius: 4px;
    display: inline-block;
    font-family: monospace;
    font-size: 0.75rem;
    padding: 2px 8px;
    margin-bottom: 6px;
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
    background: transparent;
    outline: none;
    cursor: pointer;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
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
    transition: width 0.05s;
  }

  .bar--focus  { background: var(--color-focus-round); }
  .bar--short  { background: var(--color-short-round); }
  .bar--long   { background: var(--color-long-round); }
  .bar--rounds { background: var(--color-foreground-darker, var(--color-foreground)); }

  .text-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    padding: 4px 8px;
    border-radius: 4px;
    transition: color 0.15s;
  }

  .text-btn:hover {
    color: var(--color-accent);
  }
</style>
