## 1. Tooltip Component

- [x] 1.1 Create `src/lib/components/Tooltip.svelte` with `text`, `delay`, and `placement` props; slot for trigger; show/hide on mouseenter/mouseleave with configurable delay timer
- [x] 1.2 Implement viewport-edge detection: on show, check `getBoundingClientRect()` and flip placement if tooltip would overflow
- [x] 1.3 Wire `aria-describedby` on the trigger element when tooltip is visible; remove when hidden
- [x] 1.4 Style the tooltip: theme-aware CSS custom properties, max-width 240px, arrow pointer, box shadow

## 2. Info Icon Component

- [x] 2.1 Create `src/lib/components/TooltipInfo.svelte` wrapping `Tooltip` with `delay={0}` and a styled `ⓘ` icon; accepts `text` prop

## 3. i18n Keys

- [x] 3.1 Add all 14 tooltip keys to `src/messages/en.json`
- [x] 3.2 Add translated versions of all 14 keys to `fr.json`, `de.json`, `es.json`, `pt.json`, `zh.json`, `ja.json`, `tr.json`

## 4. Timer Window — Tooltips

- [x] 4.1 Wrap Restart Round button in `Titlebar.svelte` or `Timer.svelte` with `<Tooltip text={m.tooltip_restart_round()}>`
- [x] 4.2 Wrap Skip button with `<Tooltip text={m.tooltip_skip()}>`
- [x] 4.3 Wrap Reset button in `TimerFooter.svelte` with `<Tooltip text={m.tooltip_reset()}>`
- [x] 4.4 Wrap Mute/Unmute button in `TimerFooter.svelte` with state-conditional tooltip (`tooltip_mute` / `tooltip_unmute`)
- [x] 4.5 Wrap round indicator `<span>` in `TimerFooter.svelte` with mode-conditional tooltip (`tooltip_round_counter` when long breaks enabled, `tooltip_round_counter_session` when disabled)
- [x] 4.6 Wrap Settings button in `Titlebar.svelte` with `<Tooltip text={m.tooltip_settings()}>`
- [x] 4.7 Wrap Statistics button in `Titlebar.svelte` with `<Tooltip text={m.tooltip_statistics()}>`

## 5. Settings Window — Info Icons

- [x] 5.1 Add `<TooltipInfo text={m.system_tray_gnome_hint()} />` next to the System Tray toggle label in `SystemSection.svelte` (Linux only, guarded by `isLinux`); remove the existing `.note` paragraph
- [x] 5.2 Add `<TooltipInfo text={m.tooltip_verbose_logging()} />` next to the Verbose Logging toggle
- [x] 5.3 Add `<TooltipInfo text={m.tooltip_websocket()} />` next to the WebSocket Server toggle
- [x] 5.4 Add `<TooltipInfo text={m.tooltip_dial_countdown()} />` next to the Dial Countdown toggle in `TimerSection.svelte`
- [x] 5.5 Add `<TooltipInfo text={m.tooltip_auto_start_work()} />` next to the Auto-start Work toggle
- [x] 5.6 Add `<TooltipInfo text={m.tooltip_auto_start_break()} />` next to the Auto-start Break toggle

## 6. Verification

- [x] 6.1 Verify all tooltips appear and dismiss correctly in both windows
- [x] 6.2 Verify viewport-edge flip: resize window small and confirm tooltips near the top flip to below
- [x] 6.3 Verify GNOME hint tooltip appears on Linux build and is absent on macOS/Windows
- [x] 6.4 Verify `npm run check` passes (no TypeScript errors)
