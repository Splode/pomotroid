## Context

The sessions table currently records round type, duration, start/end timestamps, and a completion flag — but no label or attribution. All stats are therefore anonymous counts. This change adds a sticky task label to the timer UI and plumbs it through to the DB and stats window.

The timer window is compact (minimum 300×300, can run much smaller in compact mode). The input must be non-intrusive: no border until focused, hidden in compact mode and during break rounds.

Three sources currently emit `timer:reset` to the frontend (reset button, settings value changes, reset-all-to-defaults). Only two of those should clear the label. A new `label:clear` event distinguishes them without altering the existing `timer:reset` contract.

## Goals / Non-Goals

**Goals:**
- One nullable `label` column on the `sessions` table (no schema redesign)
- Sticky label input in the timer UI, below the round type label, work rounds only, non-compact only
- Label cleared only on explicit user reset (button/shortcut) or "reset all to defaults"
- Label stored at session completion time (not insert time), so a mid-round change is captured
- Label breakdown in all three stats tabs (pie chart for daily, ranked horizontal bars for weekly and all-time)
- Sessions with `label = NULL` bucketed as "Unlabeled" in breakdown views

**Non-Goals:**
- Multiple labels or tags per session
- Label persistence across app restarts (in-memory only for v1)
- Retroactive label editing in the stats view
- Label filtering / searching in the stats view
- Label autocomplete or suggestion

## Decisions

### 1. Label stored at completion, not insert

**Decision**: Write the label to the DB row when `complete_session()` fires (at round end), not when the session row is first inserted (at `elapsed = 1`).

**Rationale**: The sticky model allows the user to update the label mid-round. The label that matters is what the user had set when the round ended — that is the authoritative record of what they worked on.

**Alternative considered**: Write at insert time and add a separate `UPDATE` at completion. This adds a round-trip and makes the completion write non-atomic. Rejected.

### 2. Label lives in `TimerController`, not frontend-only state

**Decision**: Store `current_label: Arc<Mutex<Option<String>>>` in `TimerController` on the Rust side. The frontend calls `timer_set_label(label)` to update it. When `complete_session()` runs (in the timer background thread), it reads `current_label` directly.

**Rationale**: `complete_session()` is called from the Rust timer event loop, which has no access to frontend state. The label must be in Rust state at completion time. Storing it in `TimerController` is consistent with how other timer-adjacent state (settings, db) is held.

**Alternative considered**: Have the frontend listen to `timer:round-change` and immediately call a `session_set_label(id, label)` IPC command after completion. This creates a race (session ID may not be stable across the event boundary) and requires exposing session IDs to the frontend. Rejected.

### 3. Separate `label:clear` event rather than a flag on `timer:reset`

**Decision**: Emit a new `label:clear` Tauri event from `timer_reset` and `settings_reset_defaults` commands. The frontend listens to it and clears the local label state.

**Rationale**: The `timer:reset` payload is currently a `TimerSnapshot`. Adding a `clear_label` boolean would require wrapping or versioning the payload type — a breaking change to the event contract. A separate event has zero impact on existing listeners and models the semantics cleanly.

**Alternative considered**: Have the frontend clear the label in the button `onclick` handler for the reset button. This misses the keyboard shortcut path (global shortcut fires through Rust, not the button). Rejected.

### 4. Label breakdown as a separate IPC command

**Decision**: New `stats_get_label_breakdown(period: String)` command returns `Vec<LabelStat { label: Option<String>, duration_mins: u32 }>`. Existing stats commands are not modified.

**Rationale**: Label breakdown is an additive, optional view. Folding it into `stats_get_detailed` or `stats_get_heatmap` would couple unrelated concerns and force all stats consumers to handle label data. The stats window can issue the label breakdown call independently per tab.

**Period values**: `"today"`, `"week"`, `"alltime"` — matching the three tab identifiers already in use.

### 5. Pie chart for daily, ranked horizontal bars for weekly and all-time

**Decision**: Daily view uses an SVG pie/donut chart; weekly and all-time use a ranked list with inline horizontal bar segments.

**Rationale**: Daily data is typically 2–6 distinct labels — a pie is readable and visually distinctive. Weekly and all-time data can have many labels across long periods; a ranked list with proportional bars scales better and allows scanning by duration.

**Unlabeled bucket**: Rendered last in all views as "(unlabeled)" in a muted color.

## Risks / Trade-offs

**[Risk] Label state lost on app restart** → Accepted for v1; the app is a single-session tool and users typically re-set their task at the start of a session anyway.

**[Risk] Round skipped at `elapsed = 0` — no session row, label not stored** → Acceptable; a skip before the first tick records nothing, consistent with existing behavior for all other session fields.

**[Risk] `complete_session()` called from background thread must read label under lock** → Mitigation: `current_label` is wrapped in `Arc<Mutex<>>` like all other shared state in `TimerController`.

**[Risk] Pie chart with many labels becomes unreadable** → Mitigation: cap the pie at top-N labels (e.g., 5) and group the rest into an "Other" slice for daily view; ranked list avoids this problem entirely for weekly/alltime.

## Migration Plan

1. Add DB migration `MIGRATION_7`: `ALTER TABLE sessions ADD COLUMN label TEXT` — backward-compatible, existing rows get `NULL`.
2. Update `complete_session()` query to accept `Option<&str>` for label and bind it.
3. Add `current_label` field to `TimerController`; initialize to `None`.
4. Add `timer_set_label` command; register in `tauri::Builder`.
5. Emit `label:clear` from `timer_reset` and `settings_reset_defaults` commands.
6. Add `stats_get_label_breakdown` command and backing query.
7. Add `onLabelClear` listener and `timerSetLabel` / `statsGetLabelBreakdown` wrappers to `ipc/index.ts`.
8. Add `TaskLabelInput` Svelte component; render in `+page.svelte` conditionally.
9. Add `LabelBreakdown` component; integrate into `DailyView`, `WeeklyView`, `YearlyView`.

Rollback: drop migration (SQLite `ALTER TABLE` can't remove columns in older SQLite; rollback means ship a new migration that leaves the column in place but unused, or restore from backup). For a desktop app, the practical rollback is a previous build.

## Open Questions

- Should the pie chart cap at 5 labels + "Other", or show all labels? (Recommend: cap at 5 for daily pie; ranked list shows all.)
- Should skipped sessions (completed=0) with a label appear in a future "abandoned" view? (Out of scope for v1, but the data is already there.)
