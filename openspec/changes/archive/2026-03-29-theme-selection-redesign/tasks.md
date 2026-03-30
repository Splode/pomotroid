## 1. Component — Tab State

- [x] 1.1 Add `activeTab: 'light' | 'dark'` reactive state variable to `AppearanceSection.svelte`, initialized from `lightIsActive` on mount
- [x] 1.2 Ensure `activeTab` updates when `lightIsActive` / `darkIsActive` derived values change on mode-selector or OS-scheme change (only for initial default; user tab switches are not overridden after mount)

## 2. Component — Tab Strip UI

- [x] 2.1 Replace the two `.group-label` headers above the theme lists with a tab strip containing "Light" and "Dark" buttons
- [x] 2.2 Apply `.active` class to the selected tab button; apply `.is-active-picker` indicator (equivalent to the old `active-badge`) to the tab whose picker is currently driving the applied theme
- [x] 2.3 Add tab strip styles: segmented button group matching the existing `.mode-selector` visual pattern (border, border-radius, flex)

## 3. Component — Conditional List Rendering

- [x] 3.1 Wrap the light theme list in `{#if activeTab === 'light'}` and the dark theme list in `{#if activeTab === 'dark'}`
- [x] 3.2 Remove the `.group-label` divs that previously labeled each list (now redundant with tabs)
- [x] 3.3 Verify the existing card `.selected` / `.highlighted` logic still works correctly with the tabbed structure

## 4. Verification

- [x] 4.1 Confirm selecting a theme on the Light tab saves `theme_light` and applies immediately when light is active
- [x] 4.2 Confirm selecting a theme on the Dark tab saves `theme_dark` and applies immediately when dark is active
- [x] 4.3 Confirm deferred-preview behavior: selecting from the inactive tab saves the setting without changing the applied theme
- [x] 4.4 Confirm the active indicator appears on the correct tab across all three modes (Light, Dark, Auto with OS light, Auto with OS dark)
- [x] 4.5 Confirm switching tabs does not alter `theme_light`, `theme_dark`, `theme_mode`, or the applied theme
