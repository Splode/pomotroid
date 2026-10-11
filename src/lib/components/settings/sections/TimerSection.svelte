<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';
  import * as m from '$paraglide/messages.js';
  import { parseDuration, formatDuration as formatMMSS } from '$lib/utils/duration';

  const MIN_SECS = 60; // 1:00
  const MAX_SECS = 5400; // 90:00
  const MAX_ROUNDS = 12;

  // Slider positions (whole minutes) derived from stored seconds.
  let workMins = $derived(Math.round($settings.time_work_secs / 60));
  let rounds = $derived($settings.long_break_interval);

  // Per-row edit state: the raw text the user is currently typing.
  let workEdit = $state<string | null>(null);
  let shortEdit = $state<string | null>(null);
  let longEdit = $state<string | null>(null);

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

  async function toggle(dbKey: string, current: boolean) {
    const updated = await setSetting(dbKey, current ? 'false' : 'true');
    settings.set(updated);
  }

  /** Commit an edited badge value: parse, clamp, save. Reverts on invalid input. */
  async function commitBadge(
    raw: string | null,
    currentSecs: number,
    dbKey: string,
    el: HTMLInputElement
  ): Promise<void> {
    if (raw === null) {
      el.value = formatMMSS(currentSecs);
      return;
    }
    const parsed = parseDuration(raw);
    if (parsed === null) {
      el.value = formatMMSS(currentSecs);
      return;
    }
    const minimum = dbKey === 'time_work_secs' ? MIN_SECS : 0;
    const clamped = Math.max(minimum, Math.min(MAX_SECS, parsed));
    await handleChange(dbKey, clamped);
    el.value = formatMMSS(clamped);
  }
</script>

<div class="section">
  <!-- Focus -->
  <div class="slider-row">
    <div class="slider-meta">
      <span class="slider-label">{m.timer_slider_focus()}</span>
      <input
        class="slider-value"
        type="text"
        aria-label={m.timer_slider_focus()}
        value={workEdit ?? formatMMSS($settings.time_work_secs)}
        onfocus={(e) => {
          workEdit = (e.target as HTMLInputElement).value;
          (e.target as HTMLInputElement).select();
        }}
        oninput={(e) => {
          workEdit = (e.target as HTMLInputElement).value;
        }}
        onblur={async (e) => {
          await commitBadge(
            workEdit,
            $settings.time_work_secs,
            'time_work_secs',
            e.target as HTMLInputElement
          );
          workEdit = null;
        }}
        onkeydown={async (e) => {
          if (e.key === 'Enter') {
            await commitBadge(
              workEdit,
              $settings.time_work_secs,
              'time_work_secs',
              e.target as HTMLInputElement
            );
            workEdit = null;
            (e.target as HTMLInputElement).blur();
          } else if (e.key === 'Escape') {
            workEdit = null;
            (e.target as HTMLInputElement).value = formatMMSS($settings.time_work_secs);
            (e.target as HTMLInputElement).blur();
          }
        }}
      />
    </div>
    <div class="slider-wrap">
      <input
        type="range"
        min="1"
        max="90"
        step="1"
        value={workMins}
        class="slider"
        oninput={(e) =>
          handleChange('time_work_secs', (e.target as HTMLInputElement).valueAsNumber * 60)}
      />
      <div class="bar bar--focus" style="width: {barWidth(workMins, 1, 90)}"></div>
    </div>
  </div>

  <!-- Short Break toggle + slider -->
  <SettingsToggle
    label={m.timer_toggle_short_breaks()}
    description={m.timer_toggle_short_breaks_desc()}
    checked={!$settings.short_breaks_enabled}
    onclick={() => toggle('short_breaks_enabled', $settings.short_breaks_enabled)}
  />
  <div class="break-body" class:disabled={!$settings.short_breaks_enabled}>
    <div class="slider-row">
      <div class="slider-meta">
        <span class="slider-label">{m.timer_slider_short_break()}</span>
        <input
          class="slider-value"
          type="text"
          aria-label={m.timer_slider_short_break()}
          title={m.timer_break_duration_hint()}
          value={shortEdit ?? formatMMSS($settings.time_short_break_secs)}
          onfocus={(e) => {
            shortEdit = (e.target as HTMLInputElement).value;
            (e.target as HTMLInputElement).select();
          }}
          oninput={(e) => {
            shortEdit = (e.target as HTMLInputElement).value;
          }}
          onblur={async (e) => {
            await commitBadge(
              shortEdit,
              $settings.time_short_break_secs,
              'time_short_break_secs',
              e.target as HTMLInputElement
            );
            shortEdit = null;
          }}
          onkeydown={async (e) => {
            if (e.key === 'Enter') {
              await commitBadge(
                shortEdit,
                $settings.time_short_break_secs,
                'time_short_break_secs',
                e.target as HTMLInputElement
              );
              shortEdit = null;
              (e.target as HTMLInputElement).blur();
            } else if (e.key === 'Escape') {
              shortEdit = null;
              (e.target as HTMLInputElement).value = formatMMSS($settings.time_short_break_secs);
              (e.target as HTMLInputElement).blur();
            }
          }}
        />
      </div>
      <div class="slider-wrap">
        <input
          type="range"
          min="0"
          max={MAX_SECS}
          step="1"
          value={$settings.time_short_break_secs}
          class="slider"
          aria-label={m.timer_slider_short_break()}
          oninput={(e) =>
            handleChange('time_short_break_secs', (e.target as HTMLInputElement).valueAsNumber)}
        />
        <div
          class="bar bar--short"
          style="width: {barWidth($settings.time_short_break_secs, 0, MAX_SECS)}"
        ></div>
      </div>
      <p class="duration-hint">{m.timer_break_duration_hint()}</p>
    </div>
  </div>

  <!-- Long Break toggle + slider + rounds -->
  <SettingsToggle
    label={m.timer_toggle_long_breaks()}
    description={m.timer_toggle_long_breaks_desc()}
    checked={!$settings.long_breaks_enabled}
    onclick={() => toggle('long_breaks_enabled', $settings.long_breaks_enabled)}
  />
  <div class="break-body" class:disabled={!$settings.long_breaks_enabled}>
    <div class="slider-row">
      <div class="slider-meta">
        <span class="slider-label">{m.timer_slider_long_break()}</span>
        <input
          class="slider-value"
          type="text"
          aria-label={m.timer_slider_long_break()}
          title={m.timer_break_duration_hint()}
          value={longEdit ?? formatMMSS($settings.time_long_break_secs)}
          onfocus={(e) => {
            longEdit = (e.target as HTMLInputElement).value;
            (e.target as HTMLInputElement).select();
          }}
          oninput={(e) => {
            longEdit = (e.target as HTMLInputElement).value;
          }}
          onblur={async (e) => {
            await commitBadge(
              longEdit,
              $settings.time_long_break_secs,
              'time_long_break_secs',
              e.target as HTMLInputElement
            );
            longEdit = null;
          }}
          onkeydown={async (e) => {
            if (e.key === 'Enter') {
              await commitBadge(
                longEdit,
                $settings.time_long_break_secs,
                'time_long_break_secs',
                e.target as HTMLInputElement
              );
              longEdit = null;
              (e.target as HTMLInputElement).blur();
            } else if (e.key === 'Escape') {
              longEdit = null;
              (e.target as HTMLInputElement).value = formatMMSS($settings.time_long_break_secs);
              (e.target as HTMLInputElement).blur();
            }
          }}
        />
      </div>
      <div class="slider-wrap">
        <input
          type="range"
          min="0"
          max={MAX_SECS}
          step="1"
          value={$settings.time_long_break_secs}
          class="slider"
          aria-label={m.timer_slider_long_break()}
          oninput={(e) =>
            handleChange('time_long_break_secs', (e.target as HTMLInputElement).valueAsNumber)}
        />
        <div
          class="bar bar--long"
          style="width: {barWidth($settings.time_long_break_secs, 0, MAX_SECS)}"
        ></div>
      </div>
      <p class="duration-hint">{m.timer_break_duration_hint()}</p>
    </div>

    <!-- Rounds -->
    <div class="slider-row">
      <div class="slider-meta">
        <span class="slider-label">{m.timer_slider_rounds()}</span>
        <span class="slider-value slider-value--static">{rounds}</span>
      </div>
      <div class="slider-wrap">
        <input
          type="range"
          min="1"
          max={MAX_ROUNDS}
          step="1"
          value={rounds}
          class="slider"
          oninput={(e) => handleChange('work_rounds', (e.target as HTMLInputElement).valueAsNumber)}
        />
        <div class="bar bar--rounds" style="width: {barWidth(rounds, 1, MAX_ROUNDS)}"></div>
      </div>
    </div>
  </div>

  <SettingsToggle
    label={m.timer_toggle_auto_start_work()}
    description={m.timer_toggle_auto_start_work_desc()}
    checked={$settings.auto_start_work}
    onclick={() => toggle('auto_start_work', $settings.auto_start_work)}
  />
  <SettingsToggle
    label={m.timer_toggle_auto_start_break()}
    description={m.timer_toggle_auto_start_break_desc()}
    checked={$settings.auto_start_break}
    onclick={() => toggle('auto_start_break', $settings.auto_start_break)}
  />
  <SettingsToggle
    label={m.timer_toggle_countdown()}
    description={m.timer_toggle_countdown_desc()}
    checked={$settings.dial_countdown}
    onclick={() => toggle('dial_countdown', $settings.dial_countdown)}
  />
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
  }

  .slider-row {
    padding: 14px 20px;
    border-bottom: 1px solid var(--color-separator);
  }

  .slider-meta {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 10px;
  }

  .duration-hint {
    margin-top: 6px;
    font-size: 0.72rem;
    line-height: 1.5;
    color: var(--color-foreground-darker, var(--color-foreground));
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
    background: var(--color-hover);
    padding: 2px 8px;
    border-radius: 3px;
    /* Override the global border-box reset so width refers to content area only,
       matching how the original <span> badge was sized. */
    box-sizing: content-box;
    width: 5ch;
    border: 1px solid transparent;
    outline: none;
    text-align: right;
    cursor: text;
    transition:
      border-color 0.15s,
      background 0.15s;
  }

  .slider-value:focus {
    border-color: color-mix(in oklch, var(--color-foreground) 35%, transparent);
    background: color-mix(in oklch, var(--color-hover) 60%, var(--color-background));
  }

  /* Static variant used for the Rounds row (no keyboard entry). */
  .slider-value--static {
    cursor: default;
    pointer-events: none;
    width: auto;
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
    background: color-mix(in oklch, var(--color-foreground) 14%, transparent);
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

  .bar--focus {
    background: var(--color-focus-round);
  }
  .bar--short {
    background: var(--color-short-round);
  }
  .bar--long {
    background: var(--color-long-round);
  }
  .bar--rounds {
    background: var(--color-foreground-darker, var(--color-foreground));
  }

  .break-body {
    transition: opacity 0.15s;
  }
  .break-body.disabled {
    opacity: 0.4;
    pointer-events: none;
  }
</style>
