<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting, resetSettings } from '$lib/ipc';

  const MAX_MINS = 90;
  const MAX_ROUNDS = 12;

  let workMins = $derived(Math.round($settings.time_work_secs / 60));
  let shortMins = $derived(Math.round($settings.time_short_break_secs / 60));
  let longMins = $derived(Math.round($settings.time_long_break_secs / 60));
  let rounds = $derived($settings.long_break_interval);

  // Returns a CSS width value that matches the browser's native thumb center
  // position for a range input with the given min/max and a 14 px thumb.
  function barWidth(val: number, min: number, max: number): string {
    const frac = (val - min) / (max - min);
    return `calc(${frac} * (100% - 14px) + 7px)`;
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

<div class="section">
  <div class="slider-row">
    <div class="slider-meta">
      <span class="slider-label">Focus</span>
      <span class="slider-value">{workMins}:00</span>
    </div>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={workMins}
        class="slider"
        oninput={(e) => handleChange('time_work_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--focus" style="width: {barWidth(workMins, 1, MAX_MINS)}"></div>
    </div>
  </div>

  <div class="slider-row">
    <div class="slider-meta">
      <span class="slider-label">Short Break</span>
      <span class="slider-value">{shortMins}:00</span>
    </div>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={shortMins}
        class="slider"
        oninput={(e) => handleChange('time_short_break_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--short" style="width: {barWidth(shortMins, 1, MAX_MINS)}"></div>
    </div>
  </div>

  <div class="slider-row">
    <div class="slider-meta">
      <span class="slider-label">Long Break</span>
      <span class="slider-value">{longMins}:00</span>
    </div>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_MINS} step="1"
        value={longMins}
        class="slider"
        oninput={(e) => handleChange('time_long_break_mins', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--long" style="width: {barWidth(longMins, 1, MAX_MINS)}"></div>
    </div>
  </div>

  <div class="slider-row">
    <div class="slider-meta">
      <span class="slider-label">Rounds until Long Break</span>
      <span class="slider-value">{rounds}</span>
    </div>
    <div class="slider-wrap">
      <input
        type="range" min="1" max={MAX_ROUNDS} step="1"
        value={rounds}
        class="slider"
        oninput={(e) => handleChange('work_rounds', (e.target as HTMLInputElement).valueAsNumber)}
      />
      <div class="bar bar--rounds" style="width: {barWidth(rounds, 1, MAX_ROUNDS)}"></div>
    </div>
  </div>

  <div class="reset-row">
    <button class="reset-btn" onclick={handleReset}>Reset to Defaults</button>
  </div>
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .slider-row {
    padding: 14px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .slider-meta {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 10px;
  }

  .slider-label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }

  .slider-value {
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
    transition: width 0.05s;
  }

  .bar--focus  { background: var(--color-focus-round); }
  .bar--short  { background: var(--color-short-round); }
  .bar--long   { background: var(--color-long-round); }
  .bar--rounds { background: var(--color-foreground-darker, var(--color-foreground)); }

  .reset-row {
    padding: 16px 20px;
    display: flex;
    justify-content: flex-end;
  }

  .reset-btn {
    background: none;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    padding: 6px 14px;
    cursor: pointer;
    transition: border-color 0.15s, color 0.15s;
  }

  .reset-btn:hover {
    border-color: var(--color-accent);
    color: var(--color-accent);
  }
</style>
