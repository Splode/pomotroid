## Context

The Timer settings section displays each duration as a static badge (`{workMins}:00`) next to a range slider. Sliders are configured with `step="1"` and save whole-minute values under the DB keys `time_work_mins`, `time_short_break_mins`, and `time_long_break_mins`. The Rust `load()` function converts these to seconds on read (`* 60`). The `Settings` struct already exposes `time_work_secs`, `time_short_break_secs`, and `time_long_break_secs` — second-resolution fields — but only whole-minute values ever reach them.

This design adds an editable MM:SS input in place of the static badge, and migrates the DB storage to whole-seconds so that arbitrary values like 5:39 survive a restart.

## Goals / Non-Goals

**Goals:**
- Allow entering any duration from 1:00 to 90:00 with second-level precision in Settings → Timer
- Persist sub-minute values across restarts (requires DB schema change)
- Keep the slider for coarse (whole-minute) adjustments; badge for precise entry
- Migrate existing installations non-destructively

**Non-Goals:**
- Sub-minute slider precision (slider remains at 1-minute steps; keyboard entry covers fine-grained use)
- Changing how the timer engine consumes seconds (already operates in seconds)
- Localising the MM:SS format (fixed for now)

## Decisions

### 1. DB keys: rename from `*_mins` to `*_secs`, storing total seconds

**Alternatives considered:**
- **Keep `*_mins` keys, store decimal minutes** (e.g. `"5.65"`) — confusing key name, floating-point parsing risk.
- **New keys alongside old keys, load falls back** — load logic becomes ambiguous; two sources of truth.
- **In-place update, same key names** — `time_work_mins` containing `"339"` is deeply confusing.

**Decision:** MIGRATION_2 reads existing `*_mins` rows, multiplies by 60, inserts `*_secs` rows, then deletes the old `*_mins` rows. `load()` switches to reading from `*_secs` keys. Key names match the `Settings` struct field names, eliminating the awkward impedance mismatch.

`defaults.rs` switches from `("time_work_mins", "25")` to `("time_work_secs", "1500")` (and equivalent for short/long break). The Rust `load()` drops the `* 60` multiplication. The frontend IPC call switches from the `*_mins` key names to `*_secs`.

### 2. Interaction model: editable badge, slider handles coarse adjustment

The `.slider-value` badge becomes an `<input type="text">` that is styled identically to the current badge when not focused. Clicking or tabbing into the badge activates edit mode (selects all text). The slider retains its 1-minute step and remains the primary interaction; the badge provides precision override.

**Alternatives considered:**
- **Replace slider with a pure text input** — loses the fast drag-to-adjust experience.
- **Always-visible editable input next to the slider** — clutters the layout; most users never need sub-minute precision.
- **Separate "advanced" toggle to reveal seconds field** — unnecessary complexity for a rare but legitimate use case.

### 3. Input parsing

Accepted formats on commit (Enter / Tab / blur):
- `MM:SS` — canonical form (e.g. `5:39`, `25:00`, `1:05`)
- Bare integer `M` or `MM` — interpreted as whole minutes (e.g. `25` → `25:00`)

Any value that does not match these patterns, or that resolves to fewer than 60 s or more than 5400 s after clamping, is snapped to the nearest valid boundary. The input reverts to the formatted display value (`MM:SS`) after commit.

Seconds component: clamped to `0–59`. Minutes component: clamped so that `MM * 60 + SS` stays within 60–5400.

### 4. Slider ↔ badge synchronisation

- **Slider move → badge updates** to nearest whole-minute display, saves whole-minute × 60 seconds to DB.
- **Badge edit → slider updates** to `Math.round(totalSeconds / 60)` (nearest minute visual); saves exact total seconds to DB.
- No bidirectional loop risk because slider `oninput` and badge `oncommit` each fire independently.

## Risks / Trade-offs

- **Existing reset-defaults test** asserts `time_work_secs == 25 * 60` after a reset by reading `time_work_mins`. After migration the test must use `time_work_secs` as the DB key in both seed and assertion. This is a straightforward update.
- **Migration irreversibility** → If a user downgrades to a pre-migration version the app falls back to `Settings::default()` for missing `*_mins` keys, which is safe (defaults are sane), but their custom values are lost. Acceptable for a desktop app; documented in migration comment.
- **MM:SS parsing edge cases** (e.g. `1:60`, `0:30`, `90:01`) → handled by clamping; no user-visible error state needed.

## Migration Plan

1. Add `MIGRATION_2` to `migrations.rs`:
   ```sql
   INSERT OR IGNORE INTO settings (key, value)
     SELECT 'time_work_secs',        CAST(CAST(value AS INTEGER) * 60 AS TEXT)
       FROM settings WHERE key = 'time_work_mins';
   INSERT OR IGNORE INTO settings (key, value)
     SELECT 'time_short_break_secs', CAST(CAST(value AS INTEGER) * 60 AS TEXT)
       FROM settings WHERE key = 'time_short_break_mins';
   INSERT OR IGNORE INTO settings (key, value)
     SELECT 'time_long_break_secs',  CAST(CAST(value AS INTEGER) * 60 AS TEXT)
       FROM settings WHERE key = 'time_long_break_mins';
   DELETE FROM settings WHERE key IN
     ('time_work_mins', 'time_short_break_mins', 'time_long_break_mins');
   INSERT INTO schema_version VALUES (2);
   ```
2. Update `run()` with `if version < 2 { ... }` block.
3. Update `defaults.rs` to seed `*_secs` keys (values in seconds).
4. Update `load()` to read `*_secs` keys without `* 60`.
5. Update `TimerSection.svelte` to call `settings_set` with `*_secs` keys.
6. Update Rust tests to reflect new key names.

Rollback: not applicable for a local SQLite DB. Users who need to downgrade manually clear the DB.
