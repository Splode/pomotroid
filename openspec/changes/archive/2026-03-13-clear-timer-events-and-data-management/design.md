## Context

The `sessions` table records every timer round (work, short break, long break) with start time, end time, duration, and completion status. It grows indefinitely with no user-facing way to prune it. The existing "Reset All Settings" action in Settings → About intentionally does not touch `sessions` — settings and history are separate concerns. The proposal makes both destructive data actions visible and co-located in Settings → System.

## Goals / Non-Goals

**Goals:**

- Add a `sessions_clear` Rust command that runs `DELETE FROM sessions` inside a transaction.
- Add a `clearSessionHistory` IPC wrapper in `ipc/index.ts`.
- Add a Clear Session History row to `SystemSection.svelte` using the same two-step inline confirmation pattern already used by Reset All Settings.
- Move the Reset All Settings row (and its confirmation state) from `AboutSection.svelte` to `SystemSection.svelte`.
- Clean up `AboutSection.svelte` by removing the reset group and its associated styles.

**Non-Goals:**

- Selective deletion (by date range, round type, etc.) — bulk clear only.
- Exporting history before clearing — out of scope.
- Any change to what data the `sessions` table records or when rows are written.
- Clearing `custom_themes` — that is theme management, not session history.

## Decisions

**Use `DELETE FROM sessions` without `DROP TABLE`**
Dropping and recreating the table would require a migration version bump and risks schema drift. A plain `DELETE` keeps the table and its indexes intact; SQLite reclaims the space on the next `VACUUM` (which we do not need to trigger explicitly). Alternative (soft-delete flag) rejected — overkill for a bulk clear.

**Mirror the Reset All Settings confirmation pattern**
`AboutSection.svelte` implements a simple boolean `confirming` state: the row shows a button, clicking it flips `confirming = true`, revealing Cancel and Confirm controls inline. Using the same pattern for Clear Session History keeps both rows visually consistent and avoids introducing a new modal or dialog component.

**Move Reset All Settings rather than duplicate it**
The reset logic lives entirely in a single `onclick` handler calling `resetSettings()`. Moving it to `SystemSection.svelte` is a cut-and-paste; no new IPC or Rust changes are needed for the move. AboutSection keeps only static metadata content (version, links, log dir).

**No new Tauri event after clear**
`sessions_clear` only affects stored history; it has no effect on the running timer or any live frontend state. No broadcast event is needed — the command returns `Ok(())` and the frontend clears its confirmation state.

## Risks / Trade-offs

- **Irreversibility** — the two-step confirmation is the only safeguard; there is no undo. Mitigation: the confirmation pattern is established UX in this app; text should make the consequence clear ("This cannot be undone").
- **Sessions table shared with future statistics** — if a statistics feature is added later it will depend on this table. Clearing it is permanent. The action label and confirmation copy must be explicit about what is deleted.

## Migration Plan

No database migration required. The `sessions` table already exists (MIGRATION_1). The new command operates on existing schema.
