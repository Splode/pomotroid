## Context

The timer sequence is owned entirely by `SequenceState::advance()` in `src-tauri/src/timer/sequence.rs`. This function already receives `&Settings` and determines the next `RoundType` and duration. All four combinations of break enable/disable flags are handled purely inside this function — no other layer of the timer stack needs to change.

The current branching logic in `advance()`:
```
Work → if work_round_number >= work_rounds_total → LongBreak, else → ShortBreak
ShortBreak → work_round_number += 1; → Work
LongBreak  → work_round_number = 1;  → Work
```

The new logic introduces two guard checks:

```
Work →
  if at_long_break_point (work_round_number >= work_rounds_total):
    if long_breaks_enabled  → LongBreak
    elif short_breaks_enabled → ShortBreak, work_round_number = 0  ← resets to 1 after Short→Work
    else → Work, work_round_number = 1  ← immediate reset, pure work loop
  else:
    if short_breaks_enabled → ShortBreak
    else → Work, work_round_number += 1  ← skip short, keep counting

ShortBreak → work_round_number += 1; → Work   (unchanged)
LongBreak  → work_round_number = 1;  → Work   (unchanged)
```

Setting `work_round_number = 0` before a substituted ShortBreak is the key detail: the existing `ShortBreak → Work` arm increments unconditionally, so setting it to 0 means it arrives at Work as 1 — preserving the invariant that `work_round_number` is always 1 at the start of a fresh cycle.

## Goals / Non-Goals

**Goals:**
- Independent enable/disable for short and long breaks.
- All four combinations produce correct, well-defined cycle behaviour.
- `work_round_number` always resets cleanly at the natural cycle boundary (the long-break point), regardless of whether a long break actually fires.
- UI dims (not hides) dependent controls when a break type is disabled.

**Non-Goals:**
- Per-round skip (transient — the existing Skip Round button covers this).
- Different behaviour per cycle (e.g., disable only the first short break).
- Changes to auto-start logic (it simply has nothing to do when a break is skipped).

## Decisions

**Substitute rather than skip at the long-break point when long breaks are disabled.** If long breaks are disabled and short breaks are enabled, taking a short break at the long-break boundary is more natural than jumping straight back to Work(1). It also reuses existing Short→Work transition logic for the counter reset.

**`work_round_number = 0` trick rather than a new reset path.** The existing `ShortBreak → Work` arm increments `work_round_number` unconditionally. Zeroing it before entering ShortBreak as a long-break substitute means no new arm is needed — the existing arm does the right thing.

**Both settings default `true`.** Existing behaviour is fully preserved for all current users. No breaking change; no migration needed to maintain the existing sequence. (Migration 5 still seeds the rows via `INSERT OR IGNORE` for correctness, but the default value matches prior implicit behaviour.)

**"Disable" toggle framing, not "Enable".** Since both break types are on by default, the toggles are labelled "Disable Short Breaks" / "Disable Long Breaks". The knob is OFF (unchecked) in the normal state and turns ON when the user opts out. This means the `checked` prop is bound to `!breaks_enabled` — the inverse of the stored setting — so the visual state matches the label.

**Dim dependent controls, don't hide them.** Consistent with the global shortcuts pattern: controls are always in the DOM, just `opacity: 0.4; pointer-events: none` when their parent toggle is off. When long breaks are disabled, both the Long Break duration slider and the Rounds until Long Break slider are wrapped together in a single `disabled` container.

**Session counter replaces cycle counter when long breaks are disabled.** The `X / Y` round counter (current work round / total before long break) has no meaning when long breaks are off — there is no natural cycle boundary for Y. Instead, `SequenceState` carries a `session_work_count` field: a monotonically-increasing count of Work rounds entered since the last reset, which never resets at cycle boundaries. `TimerFooter` switches between `X / Y` (long breaks enabled) and a localised `"round N"` label (long breaks disabled). The `timer_session_round` i18n key carries an `{n}` parameter so each locale can position the number naturally.

## Risks / Trade-offs

**Risk: `work_round_number = 0` is a surprising intermediate state.** If a snapshot is taken between the Work→ShortBreak transition and the ShortBreak→Work transition, the round number will read as 0.
→ Mitigation: Snapshots are only emitted at round boundaries after `advance()` completes; the 0 state is never observable externally.

**Risk: Both breaks disabled with `work_rounds_total = 1` produces an immediate tight loop: Work(1) → Work(1) → ...** This is correct by the spec, but a user who accidentally ends up here may wonder why nothing seems to happen between rounds.
→ Mitigation: No action needed — the auto-start settings control whether transitions are immediate or require a manual start, so the experience is no worse than the current single-round long-break loop.

## Migration Plan

Migration 5 in `db/migrations.rs`:
```sql
INSERT OR IGNORE INTO settings (key, value) VALUES ('short_breaks_enabled', 'true');
INSERT OR IGNORE INTO settings (key, value) VALUES ('long_breaks_enabled', 'true');
INSERT INTO schema_version VALUES (5);
```

`INSERT OR IGNORE` means existing installs are unaffected. Fresh installs get both keys from `seed_defaults` instead.
