## Context

The Settings window has six sections (Timer, Appearance, Notifications, Shortcuts, System, About). The "Reset to Defaults" button currently sits at the bottom of `TimerSection.svelte` and calls `resetSettings()`, which invokes the `settings_reset_defaults` Tauri command. That command deletes all 26 rows from the `settings` table and re-seeds factory defaults — affecting every section, not just timers. The About section is the natural home for a global administrative action: it already hosts external navigation (release notes, source code, log folder) and is understood by users as a meta/informational area rather than a settings-editing area.

## Goals / Non-Goals

**Goals:**
- Move the reset action to `AboutSection.svelte`, visually separated from the navigation links.
- Add inline two-step confirmation: first click reveals "Are you sure?" with Cancel and Reset buttons; second click fires the reset; Cancel restores the original button.
- Remove the reset UI from `TimerSection.svelte` entirely.
- Keep all changes frontend-only — no backend or IPC changes.

**Non-Goals:**
- Section-scoped resets (reset only timer settings, etc.) — out of scope for this change.
- Undo/restore after reset — out of scope.
- Any new Tauri dialog plugin usage.
- Changes to `settings_reset_defaults` command behavior.

## Decisions

### D1: About section as the destination

**Decision**: Place the reset row in `AboutSection.svelte`, below the existing links group and separated by a visual gap or separator.

**Alternatives considered**:
- *System section*: Already dense with WebSocket, language, logging, tray, and window groups. Adding a destructive action there adds cognitive weight to an area users visit frequently to adjust integrations.
- *Persistent footer*: A global footer always in view would give the action unwarranted prominence for something that should rarely be used.
- *Timer section* (status quo): Misleading scope — users infer it resets only timer settings.

About is visited infrequently (meta/administrative use), which matches the expected frequency of a factory reset.

### D2: Inline two-step confirmation

**Decision**: Use a `$state` boolean (`confirming`) in `AboutSection.svelte`. When `confirming` is false, show a single "Reset All Settings" button styled as a muted/danger row. When `confirming` is true, replace it with a "Are you sure?" label and two buttons: "Cancel" (sets `confirming = false`) and "Reset" (calls `resetSettings()`, then sets `confirming = false`).

**Alternatives considered**:
- *Native `confirm()` dialog from `tauri-plugin-dialog`*: Already installed. Rejected because the OS dialog is a stylistic mismatch — the app has a distinctive custom aesthetic and an OS modal breaks that context at exactly the moment the user should feel in control.
- *Countdown/cancel pattern*: Async timer state is more complexity than warranted for a settings panel. Inline two-step is simpler and equally legible.
- *Hold-to-confirm*: Unfamiliar on desktop, no existing pattern in the app.

### D3: Styling the reset row

**Decision**: Style the initial "Reset All Settings" button using `--color-foreground-darker` (muted) with a hover state that shifts toward a danger color. The confirmation state uses `--color-accent` for the Reset button to draw attention. Both states use the existing `.link-row` border/padding pattern from the About section for visual consistency.

**Rationale**: The action should be findable but not alarming at a glance. The danger signal should appear only when the user has already shown intent (the first click).

### D4: Localization

**Decision**: Add new i18n keys for the reset row label, confirmation prompt text, and the confirm/cancel button labels. Follow the existing `about_*` key namespace.

## Risks / Trade-offs

- **No undo**: Once confirmed, settings are gone. Mitigated by the two-step confirmation and the fact that re-customization is straightforward.
- **Confirmation state persists if user navigates away and back**: If `confirming = true` and the user switches sections and returns, the confirmation prompt would still be visible. Mitigation: reset `confirming` to false in an `$effect` that watches the active section, or simply accept it as harmless — the user would just see "Are you sure?" again and can Cancel.
- **Timer section cleanup**: Removing the reset button from TimerSection removes a feature that power users may have muscle-memoried. Low risk given the button was incorrectly placed.

## Migration Plan

No data migration. No backend changes. Pure frontend relocation. Rollout is a single PR merge.
