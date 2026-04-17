## Why

The Appearance section currently renders the full theme list twice — once for light mode and once for dark — which means scrolling through 18+ cards twice to configure themes. This makes the settings panel feel long and repetitive, especially since only one picker is active at a given time.

## What Changes

- Replace the two stacked, always-visible theme lists with a tabbed picker that shows one list at a time
- The tab strip displays "Light" and "Dark" tabs, each showing the currently selected theme name and swatch inline
- Selecting a tab reveals only that picker's list; the other is hidden
- The tab corresponding to the currently active theme (based on mode + OS) carries an "active" indicator
- The existing Auto / Light / Dark mode selector is retained above the picker

## Capabilities

### New Capabilities

- `theme-selection-ui`: Tabbed theme picker component — tab strip with inline theme preview, single visible list, active-mode indicator on the relevant tab.

### Modified Capabilities

- `theme-mode`: The "Independent light and dark theme pickers" requirement is unchanged in behavior but the UI mechanism changes from two simultaneous lists to a single tabbed list. No functional regression; deferred-preview and active-resolution logic are preserved.

## Impact

- `src/lib/components/settings/sections/AppearanceSection.svelte` — primary change; tab state replaces the two `<div class="theme-list">` blocks
- No Rust, IPC, settings schema, or DB changes required
- i18n: two new message keys for the tab labels (light / dark)
- No new dependencies
