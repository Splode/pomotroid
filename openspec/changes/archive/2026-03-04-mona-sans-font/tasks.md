## 1. Font Assets

- [x] 1.1 Create `static/fonts/` directory
- [x] 1.2 Download `MonaSansVF[wdth,wght,opsz,ital].woff2` from `github/mona-sans` (`fonts/webfonts/variable/`) and place in `static/fonts/`
- [x] 1.3 Download `MonaSansMonoVF[wght].woff2` from `github/mona-sans` (`fonts/webfonts/variable/`) and place in `static/fonts/`

## 2. Global CSS (`src/app.css`)

- [x] 2.1 Add `@font-face` block for Mona Sans: `font-family: 'Mona Sans'`, `src: url('/fonts/MonaSansVF[wdth,wght,opsz,ital].woff2') format('woff2')`, `font-weight: 200 900`, `font-stretch: 75% 125%`, `font-style: normal oblique 0deg 10deg`, `font-display: block`
- [x] 2.2 Add `@font-face` block for Mona Sans Mono: `font-family: 'Mona Sans Mono'`, `src: url('/fonts/MonaSansMonoVF[wght].woff2') format('woff2')`, `font-weight: 200 900`, `font-display: block`
- [x] 2.3 Update `body` rule: replace `font-family: system-ui, -apple-system, sans-serif` with `font-family: 'Mona Sans', system-ui, sans-serif`
- [x] 2.4 Add `font-optical-sizing: auto` to the `:root` block

## 3. Timer Display (`src/lib/components/TimerDisplay.svelte`)

- [x] 3.1 Add `isCompact?: boolean` to the `Props` interface (default `false`)
- [x] 3.2 Destructure `isCompact = false` from `$props()`
- [x] 3.3 Add a `$derived` value for the dynamic font-stretch: `85%` when `isCompact`, `95%` when `state.is_running`, otherwise `103%`
- [x] 3.4 Bind the derived font-stretch to the `.time` element via `style="font-stretch: {stretchValue}"`
- [x] 3.5 Update `.time` CSS: change `font-weight` from `300` to `350`; add `transition: font-stretch 400ms ease`; remove any static `font-stretch` if present

## 4. Thread `isCompact` into TimerDisplay (`src/lib/components/Timer.svelte`)

- [x] 4.1 Pass `isCompact` to the `<TimerDisplay>` component: `<TimerDisplay {state} {isCompact} />`

## 5. Shortcut Input (`src/lib/components/ShortcutInput.svelte`)

- [x] 5.1 Update the monospace `font-family` declaration from `monospace` to `'Mona Sans Mono', monospace`

## 6. Verification

- [x] 6.1 Run `npm run check` — confirm zero TypeScript/Svelte errors
- [x] 6.2 Visual check in `npm run tauri dev`: confirm Mona Sans renders in the main window (timer digits, round label, titlebar)
- [x] 6.3 Visual check: start and pause the timer — confirm font-stretch animates smoothly between 95% and 103%
- [x] 6.4 Visual check: resize window below compact threshold — confirm font-stretch snaps to 85%
- [x] 6.5 Visual check: open Stats window — confirm chart labels and numbers render in Mona Sans
- [x] 6.6 Visual check: open Settings → Shortcuts — confirm shortcut keys render in Mona Sans Mono
