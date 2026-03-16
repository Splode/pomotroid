## 1. Rust — Settings

- [x] 1.1 In `src-tauri/src/settings/mod.rs`, add `pub short_breaks_enabled: bool` and `pub long_breaks_enabled: bool` fields to the `Settings` struct (after `long_break_interval`)
- [x] 1.2 In `src-tauri/src/settings/mod.rs`, add `short_breaks_enabled: true` and `long_breaks_enabled: true` to `Settings::default()`
- [x] 1.3 In `src-tauri/src/settings/mod.rs`, add both fields to the `load()` DB→struct mapping using `parse_bool`
- [x] 1.4 In `src-tauri/src/settings/defaults.rs`, add `("short_breaks_enabled", "true")` and `("long_breaks_enabled", "true")` to `DEFAULTS`

## 2. Rust — Database Migration

- [x] 2.1 In `src-tauri/src/db/migrations.rs`, add `MIGRATION_5` constant that seeds both new keys with `INSERT OR IGNORE` and increments schema version to 5
- [x] 2.2 In the `run()` function, add `if version < 5 { ... }` block applying `MIGRATION_5` with logging consistent with existing blocks
- [x] 2.3 Update the migration idempotency test assertion from `v == 4` to `v == 5`

## 3. Rust — Timer Sequence

- [x] 3.1 In `src-tauri/src/timer/sequence.rs`, update `advance()` to implement the four-combination break logic:
  - Work, before long-break point: if `short_breaks_enabled` → ShortBreak, else → Work with `work_round_number += 1`
  - Work, at long-break point: if `long_breaks_enabled` → LongBreak; elif `short_breaks_enabled` → ShortBreak with `work_round_number = 0` (resets to 1 after Short→Work); else → Work with `work_round_number = 1`
- [x] 3.2 Add unit tests covering all four flag combinations:
  - `short_breaks_disabled_chains_work_rounds` — verify Work→Work→...→LongBreak with `short_breaks_enabled=false`
  - `long_breaks_disabled_substitutes_short_break` — verify Work(N=total)→ShortBreak and counter reset with `long_breaks_enabled=false`
  - `both_breaks_disabled_pure_work_loop` — verify Work(1)→Work(2)→...→Work(total)→Work(1) with both false
  - `long_breaks_disabled_short_breaks_fire_normally` — verify ShortBreak still fires before the long-break point when only `long_breaks_enabled=false`

## 4. Frontend — Types and Store

- [x] 4.1 In `src/lib/types.ts`, add `short_breaks_enabled: boolean` and `long_breaks_enabled: boolean` to the `Settings` interface (after `long_break_interval`)
- [x] 4.2 In `src/lib/stores/settings.ts`, add `short_breaks_enabled: true` and `long_breaks_enabled: true` to the defaults object

## 5. Frontend — TimerSection UI

- [x] 5.1 In `src/lib/components/settings/sections/TimerSection.svelte`, add a `toggle` helper that calls `setSetting(dbKey, current ? 'false' : 'true')` and updates the settings store
- [x] 5.2 Add a `SettingsToggle` for `short_breaks_enabled` immediately above the Short Break duration slider row, using i18n keys `timer_toggle_short_breaks` and `timer_toggle_short_breaks_desc`
- [x] 5.3 Wrap the Short Break duration slider row in a `<div class="break-body" class:disabled={!$settings.short_breaks_enabled}>` that applies `opacity: 0.4; pointer-events: none` when disabled
- [x] 5.4 Add a `SettingsToggle` for `long_breaks_enabled` immediately above the Long Break duration slider row
- [x] 5.5 Wrap both the Long Break duration slider row and the Rounds until Long Break slider row together in a `<div class="break-body" class:disabled={!$settings.long_breaks_enabled}>` that applies `opacity: 0.4; pointer-events: none` when disabled
- [x] 5.6 Add `.break-body` and `.break-body.disabled` CSS rules to the component's `<style>` block (same values as the `.shortcuts-body.disabled` pattern)

## 6. Localisation

- [x] 6.1 Add `"timer_toggle_short_breaks"`, `"timer_toggle_short_breaks_desc"`, `"timer_toggle_long_breaks"`, and `"timer_toggle_long_breaks_desc"` to all 8 locale files in `src/messages/` with appropriate translations:
  - `en`: `"Enable Short Breaks"` / `"Take a short break between each work round."` / `"Enable Long Breaks"` / `"Take a longer break after every N work rounds."`
  - `de`: `"Kurze Pausen aktivieren"` / `"Nach jeder Arbeitsrunde eine kurze Pause einlegen."` / `"Lange Pausen aktivieren"` / `"Nach je N Arbeitsrunden eine längere Pause einlegen."`
  - `es`: `"Activar descansos cortos"` / `"Tomar un descanso corto entre cada ronda de trabajo."` / `"Activar descansos largos"` / `"Tomar un descanso más largo después de cada N rondas de trabajo."`
  - `fr`: `"Activer les pauses courtes"` / `"Prendre une courte pause entre chaque session de travail."` / `"Activer les pauses longues"` / `"Prendre une pause plus longue après chaque N sessions de travail."`
  - `ja`: `"短い休憩を有効にする"` / `"各作業ラウンドの後に短い休憩を取ります。"` / `"長い休憩を有効にする"` / `"N回の作業ラウンドごとに長い休憩を取ります。"`
  - `zh`: `"启用短休息"` / `"每个工作轮次之间进行短暂休息。"` / `"启用长休息"` / `"每完成 N 个工作轮次后进行较长休息。"`
  - `pt`: `"Ativar Pausas Curtas"` / `"Fazer uma pausa curta entre cada rodada de trabalho."` / `"Ativar Pausas Longas"` / `"Fazer uma pausa mais longa após cada N rodadas de trabalho."`
  - `tr`: `"Kısa Molaları Etkinleştir"` / `"Her çalışma turları arasında kısa bir mola ver."` / `"Uzun Molaları Etkinleştir"` / `"Her N çalışma turunda bir uzun mola ver."`

## 7. Verify

- [ ] 7.1 Both breaks enabled (default) — cycle matches existing behaviour exactly
- [ ] 7.2 Short breaks disabled — Work rounds chain directly; long break still fires at the right interval
- [ ] 7.3 Long breaks disabled — short break substituted at long-break point; `work_round_number` resets to 1 after it
- [ ] 7.4 Both breaks disabled — pure work loop; round counter increments and resets correctly
- [ ] 7.5 Disabling a break type dims its duration slider (and for long breaks, the rounds slider); enabling restores interaction
- [ ] 7.6 Reset All Settings restores both toggles to enabled
- [x] 7.7 Run `npm run check` — no type errors
- [x] 7.8 Run `cargo test` in `src-tauri/` — all tests pass
