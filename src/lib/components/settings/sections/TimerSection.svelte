<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import { setSetting, timerResetIncrement } from '$lib/ipc';
  import SettingsToggle from '$lib/components/settings/SettingsToggle.svelte';
  import * as m from '$paraglide/messages.js';

  const MIN_SECS = 60; // 1:00
  const MAX_SECS = 5400; // 90:00
  const MAX_ROUNDS = 12;

  // Incremental focus mode bounds.
  const MIN_INCREMENT_MINS = 1;
  const MAX_INCREMENT_MINS = 30;
  const MAX_CAP_MINS = 180;

  // Slider positions (whole minutes) derived from stored seconds.
  let workMins = $derived(Math.round($settings.time_work_secs / 60));
  let shortMins = $derived(Math.round($settings.time_short_break_secs / 60));
  let longMins = $derived(Math.round($settings.time_long_break_secs / 60));
  let rounds = $derived($settings.long_break_interval);

  let incrementMins = $derived(Math.round($settings.time_work_increment_secs / 60));
  let capMins = $derived(Math.round($settings.time_work_max_secs / 60));

  // Per-row edit state: the raw text the user is currently typing.
  let workEdit = $state<string | null>(null);
  let shortEdit = $state<string | null>(null);
  let longEdit = $state<string | null>(null);
  let incrementEdit = $state<string | null>(null);

  /** Parse MM:SS or bare integer minutes. Returns total seconds, or null on failure. */
  function parseMMSS(input: string): number | null {
    const trimmed = input.trim();
    const colonIdx = trimmed.indexOf(':');
    if (colonIdx === -1) {
      const mins = parseInt(trimmed, 10);
      if (isNaN(mins) || trimmed === '') return null;
      return mins * 60;
    }
    const mm = parseInt(trimmed.slice(0, colonIdx), 10);
    const ss = parseInt(trimmed.slice(colonIdx + 1), 10);
    if (isNaN(mm) || isNaN(ss) || ss < 0 || ss > 59) return null;
    return mm * 60 + ss;
  }

  /** Format total seconds as M:SS or MM:SS. */
  function formatMMSS(totalSecs: number): string {
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    return `${mins}:${String(secs).padStart(2, '0')}`;
  }

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
    const parsed = parseMMSS(raw);
    if (parsed === null) {
      el.value = formatMMSS(currentSecs);
      return;
    }
    const clamped = Math.max(MIN_SECS, Math.min(MAX_SECS, parsed));
    await handleChange(dbKey, clamped);
    el.value = formatMMSS(clamped);
  }

  // ── Incremental focus mode ──────────────────────────────────────────────
  //
  // The ladder preview mirrors the Rust `SequenceState::work_duration_secs`
  // calculation so the user can see exactly what each round will be.

  const STEP_COUNT = 6;

  /** Work duration (minutes) for round `n` (1-based) of the ladder. */
  function ladderStepMins(n: number): number {
    const base = workMins;
    const inc = incrementMins;
    return Math.min(base + inc * (n - 1), Math.max(capMins, base));
  }

  let ladder = $derived(
    Array.from({ length: STEP_COUNT }, (_, i) => ladderStepMins(i + 1))
  );
  /** True once the ladder stops growing before STEP_COUNT rounds. */
  let ladderCapped = $derived(
    ladder.length > 1 && ladder[ladder.length - 1] === ladder[ladder.length - 2]
  );

  /** Cap slider floor: never below the base work duration. */
  let capMin = $derived(Math.max(workMins, 5));

  /**
   * Where the ladder restarts from, as configured by the two reset triggers.
   * Both default to on, which is the behaviour of every build before the
   * triggers became configurable.
   */
  let ladderHint = $derived.by(() => {
    const onLongBreak = $settings.incremental_reset_on_long_break;
    const daily = $settings.incremental_reset_daily;
    if (onLongBreak && daily) return m.timer_increment_ladder_hint();
    if (daily) return m.timer_increment_hint_daily();
    if (onLongBreak) return m.timer_increment_hint_long_break();
    return m.timer_increment_hint_never();
  });

  /** Transient confirmation shown after a manual ladder reset. */
  let ladderResetDone = $state(false);
  let ladderResetTimer: ReturnType<typeof setTimeout> | null = null;

  /** Manual reset: back to the base duration without touching round counters. */
  async function resetLadder(): Promise<void> {
    await timerResetIncrement();
    ladderResetDone = true;
    if (ladderResetTimer) clearTimeout(ladderResetTimer);
    ladderResetTimer = setTimeout(() => {
      ladderResetDone = false;
      ladderResetTimer = null;
    }, 2500);
  }

  /** Commit the increment badge (minutes). */
  async function commitIncrement(raw: string | null, el: HTMLInputElement): Promise<void> {
    const parsed = raw === null ? null : parseMMSS(raw);
    if (parsed === null) {
      el.value = formatMMSS($settings.time_work_increment_secs);
      return;
    }
    const mins = Math.max(
      MIN_INCREMENT_MINS,
      Math.min(MAX_INCREMENT_MINS, Math.round(parsed / 60))
    );
    await handleChange('time_work_increment_secs', mins * 60);
    el.value = formatMMSS(mins * 60);
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
          min="1"
          max="90"
          step="1"
          value={shortMins}
          class="slider"
          oninput={(e) =>
            handleChange(
              'time_short_break_secs',
              (e.target as HTMLInputElement).valueAsNumber * 60
            )}
        />
        <div class="bar bar--short" style="width: {barWidth(shortMins, 1, 90)}"></div>
      </div>
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
          min="1"
          max="90"
          step="1"
          value={longMins}
          class="slider"
          oninput={(e) =>
            handleChange('time_long_break_secs', (e.target as HTMLInputElement).valueAsNumber * 60)}
        />
        <div class="bar bar--long" style="width: {barWidth(longMins, 1, 90)}"></div>
      </div>
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

  <!-- Incremental focus: each work round gets longer than the last -->
  <SettingsToggle
    label={m.timer_toggle_incremental()}
    description={m.timer_toggle_incremental_desc()}
    checked={$settings.incremental_work_enabled}
    onclick={() => toggle('incremental_work_enabled', $settings.incremental_work_enabled)}
  />
  <div class="break-body" class:disabled={!$settings.incremental_work_enabled}>
    <!-- Add per round -->
    <div class="slider-row">
      <div class="slider-meta">
        <span class="slider-label">{m.timer_slider_increment()}</span>
        <input
          class="slider-value"
          type="text"
          value={incrementEdit ?? formatMMSS($settings.time_work_increment_secs)}
          onfocus={(e) => {
            incrementEdit = (e.target as HTMLInputElement).value;
            (e.target as HTMLInputElement).select();
          }}
          oninput={(e) => {
            incrementEdit = (e.target as HTMLInputElement).value;
          }}
          onblur={async (e) => {
            await commitIncrement(incrementEdit, e.target as HTMLInputElement);
            incrementEdit = null;
          }}
          onkeydown={async (e) => {
            if (e.key === 'Enter') {
              await commitIncrement(incrementEdit, e.target as HTMLInputElement);
              incrementEdit = null;
              (e.target as HTMLInputElement).blur();
            } else if (e.key === 'Escape') {
              incrementEdit = null;
              (e.target as HTMLInputElement).value = formatMMSS(
                $settings.time_work_increment_secs
              );
              (e.target as HTMLInputElement).blur();
            }
          }}
        />
      </div>
      <div class="slider-wrap">
        <input
          type="range"
          min={MIN_INCREMENT_MINS}
          max={MAX_INCREMENT_MINS}
          step="1"
          value={incrementMins}
          class="slider"
          oninput={(e) =>
            handleChange(
              'time_work_increment_secs',
              (e.target as HTMLInputElement).valueAsNumber * 60
            )}
        />
        <div
          class="bar bar--focus"
          style="width: {barWidth(incrementMins, MIN_INCREMENT_MINS, MAX_INCREMENT_MINS)}"
        ></div>
      </div>
    </div>

    <!-- Cap -->
    <div class="slider-row">
      <div class="slider-meta">
        <span class="slider-label">{m.timer_slider_increment_cap()}</span>
        <span class="slider-value slider-value--static">{capMins}m</span>
      </div>
      <div class="slider-wrap">
        <input
          type="range"
          min={capMin}
          max={MAX_CAP_MINS}
          step="5"
          value={capMins}
          class="slider"
          oninput={(e) =>
            handleChange('time_work_max_secs', (e.target as HTMLInputElement).valueAsNumber * 60)}
        />
        <div class="bar bar--long" style="width: {barWidth(capMins, capMin, MAX_CAP_MINS)}"></div>
      </div>
    </div>

    <!-- Reset triggers -->
    <SettingsToggle
      label={m.timer_toggle_increment_reset_long_break()}
      description={m.timer_toggle_increment_reset_long_break_desc()}
      checked={$settings.incremental_reset_on_long_break}
      onclick={() =>
        toggle('incremental_reset_on_long_break', $settings.incremental_reset_on_long_break)}
    />
    <SettingsToggle
      label={m.timer_toggle_increment_reset_daily()}
      description={m.timer_toggle_increment_reset_daily_desc()}
      checked={$settings.incremental_reset_daily}
      onclick={() => toggle('incremental_reset_daily', $settings.incremental_reset_daily)}
    />

    <!-- Ladder preview -->
    <div class="ladder">
      <span class="ladder-title">{m.timer_increment_ladder()}</span>
      <div class="ladder-steps">
        {#each ladder as mins, i (i)}
          <span class="ladder-step" class:capped={ladderCapped && i >= 1 && mins === ladder[i - 1]}>
            {mins}m
          </span>
          {#if i < ladder.length - 1}
            <span class="ladder-arrow">›</span>
          {/if}
        {/each}
        <span class="ladder-arrow">…</span>
      </div>
      <span class="ladder-hint">
        {ladderCapped ? m.timer_increment_capped() : ladderHint}
      </span>
    </div>

    <!-- Manual reset -->
    <button class="action-row" onclick={resetLadder}>
      <span class="action-text">
        <span class="action-label">{m.timer_increment_reset_now()}</span>
        <span class="action-desc" class:done={ladderResetDone}>
          {ladderResetDone ? m.timer_increment_reset_done() : m.timer_increment_reset_now_desc()}
        </span>
      </span>
      <span class="action-icon" aria-hidden="true">↺</span>
    </button>
  </div>
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

  /* ── Incremental ladder preview ─────────────────────────── */
  .ladder {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px 20px 16px;
    border-bottom: 1px solid var(--color-separator);
  }

  .ladder-title {
    font-size: 0.62rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-foreground-darker);
  }

  .ladder-steps {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 6px;
  }

  .ladder-step {
    font-size: 0.72rem;
    font-family: monospace;
    font-variant-numeric: tabular-nums;
    padding: 3px 8px;
    border-radius: 3px;
    background: var(--color-hover);
    color: var(--color-foreground);
    border: 1px solid transparent;
  }

  .ladder-step.capped {
    border-color: color-mix(in oklch, var(--color-long-round) 55%, transparent);
    color: var(--color-long-round);
  }

  .ladder-arrow {
    font-size: 0.72rem;
    color: var(--color-foreground-darker);
  }

  .ladder-hint {
    font-size: 0.68rem;
    font-style: italic;
    color: color-mix(in oklch, var(--color-foreground-darker) 70%, transparent);
  }

  /* ── Manual ladder reset ────────────────────────────────── */
  .action-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 20px;
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-separator);
    cursor: pointer;
    text-align: left;
    gap: 16px;
    transition: background 0.12s;
  }

  .action-row:hover {
    background: var(--color-hover);
  }

  .action-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .action-label {
    font-size: 0.85rem;
    color: var(--color-foreground);
    letter-spacing: 0.02em;
  }

  .action-desc {
    font-size: 0.72rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    letter-spacing: 0.02em;
    opacity: 0.7;
  }

  .action-desc.done {
    color: var(--color-accent);
    opacity: 1;
  }

  .action-icon {
    font-size: 0.95rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    flex-shrink: 0;
  }
</style>
