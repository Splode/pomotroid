## 1. Page Layout Adjustment

- [x] 1.1 In `src/routes/+page.svelte`, increase `COMPACT_BOTTOM_PAD` from `20` to `48`
- [x] 1.2 In the same file, change `main.compact { padding-bottom: 20px }` to `padding-bottom: 8px`

## 2. MiniControls Component

- [x] 2.1 Create `src/lib/components/MiniControls.svelte`
- [x] 2.2 Import `timerToggle`, `timerRestartRound`, `timerSkip` from `$lib/ipc`
- [x] 2.3 Import `timerState` from `$lib/stores/timer` for play/pause icon state
- [x] 2.4 Render three icon-only buttons in a row: restart (back icon), play/pause (fade-switched icon matching timer state), skip (forward icon)
- [x] 2.5 Style buttons: 24×24px, icon sizes 10px (side) / 12px (play/pause), gap 14px, colors `--color-foreground-darker` at rest → `--color-foreground` on hover; play/pause has a thin 1.5px circular border

## 3. Timer Component Restructure

- [x] 3.1 In `Timer.svelte`, wrap the existing `<div class="timer" style="zoom: {uiScale}">` in a new `<div class="timer-outer" class:compact={isCompact}>` element
- [x] 3.2 Import `MiniControls` from `./MiniControls.svelte`
- [x] 3.3 After the zoom div, conditionally render `{#if isCompact}<MiniControls />{/if}`
- [x] 3.4 Add `.timer-outer` CSS: `display: flex; flex-direction: column; align-items: center; gap: 8px`

## 4. Verification

- [x] 4.1 Run `npm run check` — no type errors
- [x] 4.2 Resize window below 300px — mini controls appear below dial
- [x] 4.3 Resize window above 300px — mini controls disappear, full controls visible
- [x] 4.4 Click each mini button and confirm correct action fires
- [x] 4.5 Confirm play/pause icon switches correctly between running/paused states
