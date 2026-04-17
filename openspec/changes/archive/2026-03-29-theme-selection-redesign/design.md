## Context

`AppearanceSection.svelte` renders two independent, always-expanded theme lists. With 18+ bundled themes plus any user-created custom themes, both lists are fully visible at all times — requiring the user to scroll through 36+ cards to review or change theme settings. The two lists are structurally identical: same card layout, same swatch rendering, differing only in which settings key they write to (`theme_light` vs `theme_dark`).

All theme logic (mode resolution, active/highlighted card state, deferred-preview behavior) lives in `AppearanceSection.svelte`. No Rust or IPC changes are needed.

## Goals / Non-Goals

**Goals:**

- Reduce visible theme cards from two full lists to one at a time
- Preserve all existing behavior: deferred preview, active-mode indicator, custom badge, checkmark
- Default the visible tab to the currently active picker so the most relevant list is shown on open
- Keep the change contained to `AppearanceSection.svelte` (and i18n messages if new keys are needed)

**Non-Goals:**

- Redesigning the card layout or swatch rendering
- Changing how themes are stored, loaded, or resolved
- Modifying the Auto / Light / Dark mode selector
- Animated transitions on tab switch (out of scope for this change)

## Decisions

### Tabbed picker over collapsible/accordion rows

**Decision**: Replace the two stacked lists with a two-tab control ("Light" / "Dark") that shows one list at a time.

**Rationale**: A collapsible accordion still requires expanding both rows to compare or configure themes in both modes; the user ends up with both lists open simultaneously — no reduction in clutter. A tabbed control enforces "one list visible at a time" structurally. It also maps naturally to the existing light/dark mental model and requires no disclosure triangle affordance.

**Alternative considered**: Single-mode-aware picker (hide the inactive list entirely, only show the one list relevant to the current mode). Rejected because users in Auto mode frequently want to configure both light and dark themes, and forcing them to toggle the mode selector just to access the other picker is a worse UX.

### Default tab follows the active picker

**Decision**: On mount, `activeTab` is initialized to `'light'` when `lightIsActive`, otherwise `'dark'`.

**Rationale**: Opens the settings window showing the picker that's currently driving the visible theme — the most relevant starting point. After initial mount, tab state is local to the component session; switching mode selector does not auto-switch the tab (avoids surprising the user mid-configuration).

### Reuse existing i18n keys for tab labels

**Decision**: Tab labels reuse `appearance_mode_light` and `appearance_mode_dark` message keys.

**Rationale**: The labels "Light" and "Dark" carry the same meaning in both contexts. Introducing duplicate keys would diverge translations unnecessarily.

### Active-mode indicator on the tab, not the list header

**Decision**: The "active" badge moves from the group-label `<div>` above each list to the corresponding tab button. The group-label headers above the list are removed.

**Rationale**: With a tab control the list header is redundant — the tab already names the picker. Consolidating the active indicator onto the tab reduces label repetition.

## Risks / Trade-offs

- **Tab state is ephemeral**: Closing and reopening settings resets to the default-active tab. Users who prefer to always configure the dark picker first may find this mildly annoying. → Acceptable; the default is the most contextually relevant option.
- **Auto mode ambiguity on tab label**: In Auto mode, neither tab label says "Auto" — users must understand that "Light" / "Dark" refer to the picker for each OS scheme. → The mode selector above the tabs already establishes this context; the active badge on the tab reinforces which one is live.
