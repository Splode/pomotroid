
[Unreleased]
-----------

### Bug Fixes

* **White screen flash on startup** — the Settings and Statistics windows no longer display a blank white frame before styles load. Windows are now created hidden and shown only after the theme has been applied.

### Audio

* **Sleep/wake audio recovery** — audio no longer produces a flood of "buffer underrun/overrun" errors after the system wakes from standby. The audio device is now opened fresh for each sound rather than held open indefinitely, so it reconnects to the OS audio subsystem automatically after a sleep/wake cycle without requiring an app restart.

### Typography

* **Mona Sans variable font** — the app now embeds [Mona Sans](https://github.com/github/mona-sans) as a variable font (weight 200–900, width 75–125%), replacing the platform system font. All UI text renders consistently across operating systems with no network dependency. Optical sizing (`font-optical-sizing: auto`) is enabled globally so letterform contrast and spacing adapt automatically to each element's rendered size. Shortcut keys in Settings → Shortcuts render in Mona Sans Mono.

### Settings

* **Keyboard entry for timer durations** — the time badge next to each timer slider in Settings → Timer is now an editable field. Enter any duration in `MM:SS` format (e.g. `5:39`) or as a bare integer in minutes (e.g. `25`). Values are clamped to the supported range of `1:00`–`90:00`. Commit with Enter or blur; Escape reverts. The slider continues to work for whole-minute adjustments.

### Timer

* **Sub-minute timer durations** — timer durations are now stored in whole seconds, enabling any duration within the 1:00–90:00 range rather than whole minutes only. Existing settings are migrated automatically.

### Statistics

* **Focus time rounds to nearest minute** — daily focus time in the Statistics view is now rounded to the nearest minute rather than truncated (e.g. a 5:39 session counts as 6 m, not 5 m).

### Appearance

* **Updated app icon** — refreshed application icon across all platforms and sizes; removed unused legacy icon variants

### System Tray

* **Timer controls in tray menu** — the system tray context menu now includes Start/Pause/Resume, Skip, and Reset Round actions. The toggle item label updates dynamically ("Start" → "Pause" → "Resume") as the timer state changes. Skip and Reset Round are disabled when the timer is idle. Reset Round restarts only the current round without advancing the work/break cycle.
* **Pause icon contrast** — the tray pause icon (two vertical bars) now uses the active round's color (red for work, teal for short break, blue for long break) instead of the theme foreground color, ensuring visibility on dark panels regardless of theme.

### Linux

* **RPM package** — `.rpm` bundle now built and distributed alongside `.deb` and `.AppImage`, targeting Fedora, RHEL, openSUSE, and derivatives
* **Release artifact signing** — `.deb`, `.rpm`, and `.AppImage` packages are now GPG-signed in CI. Each release includes a detached `.asc` signature file alongside every Linux package. Import `public.asc` from the repository root and run `gpg --verify <file>.asc <file>` to verify any download. See `SECURITY.md` for full instructions.

---

[v1.0.0] - 2026-03-02
-----------

### Complete Rewrite — Electron → Tauri 2 + Rust + Svelte 5

Pomotroid has been rebuilt from the ground up. The Electron + Vue.js stack has been replaced with a fully native Tauri 2 application backed by Rust and a Svelte 5 frontend, resulting in a drastically smaller footprint, faster startup, and no Chromium dependency.

### Timer

* **Drift-correcting engine** — Rust timer thread uses `std::time::Instant` with a fixed tick schedule; eliminates the cumulative drift that plagued the web-worker approach
* **Sleep / wake handling** — timer pauses automatically on OS sleep and resumes from the correct position on wake; no missed rounds
* **Round sequencing** — configurable work / short-break / long-break cycle with independent auto-start per round type
* **Skip and round reset** — skip to the next round or restart only the current round at any time, regardless of timer state
* **Session recording** — every completed round is written to SQLite with type, duration, and completion status

### Statistics

* **Dedicated stats window** with three tabs:
  * **Today** — work and break round counts, total focus time, hourly breakdown chart
  * **This Week** — daily bar chart for the past seven days, current consecutive-day streak
  * **All Time** — 52-week heatmap (GitHub-style contribution calendar), lifetime session totals
* Charts use pure SVG; no external charting library
* Chart and heatmap colours are derived from the active theme
* Stats update live when a round completes — no manual refresh needed

### Themes & Appearance

* **37 bundled themes** including Pomotroid (dark), Pomotroid Light, Dracula, Nord, Tokyo Night, Gruvbox, Solarized, GitHub, One Dark, Rose Piné (3 variants), Catppuccin (4 variants), Synthwave, Ayu, Everforest, Kanagawa, Monokai, Night Owl, and more
* **Auto light / dark mode** — follows the OS `prefers-color-scheme` preference automatically; separate theme pickers for light and dark
* **Custom theme hot-reload** — drop a JSON theme file into the user themes folder and it appears instantly without a restart
* Theme colours propagate throughout the full UI and into the tray icon arc

### Localization

* **7 languages** — English, Spanish, French, German, Japanese, Chinese (Simplified), and Portuguese
* Auto-detects OS language on first launch; can be overridden manually in Settings
* All UI strings, settings labels, and notification messages are translated
* Locale-aware date and time formatting throughout (Intl.DateTimeFormat, reactive to language changes)
* Built with Paraglide JS v2 — compile-time, type-safe, tree-shakable message functions

### Audio

* **Configurable alert cues** for work rounds, short breaks, and long breaks
* **Custom audio** — replace any of the three alert cues with your own file via a file picker
* **Tick sounds** — independently toggleable for work and break rounds
* **Volume control** — global volume slider (0–100)
* All audio runs on a dedicated thread via `rodio`; playback is guaranteed even when the window is hidden to the tray

### System Tray

* **Dynamic tray icon** — a progress arc rendered with `tiny-skia` sweeps clockwise from 12 o'clock, coloured by round type (work / short break / long break)
* **Countdown mode** — optional inverted arc that fills from full to empty instead of empty to full
* **Pause indicator** — two vertical bars drawn over the icon when the timer is paused
* **Tray menu** — Show and Exit actions; left-clicking the icon toggles window visibility
* **Minimise to tray** and **close to tray** are independently configurable

### Global Shortcuts

* **Four actions** — toggle (start / pause / resume), reset, skip, restart round
* **Platform defaults** — `Control+F1–F4` on Windows and Linux; `Command+Shift+1–4` on macOS
* Fully rebindable in Settings; supports modifier combinations and function keys
* Shortcuts work when the window is hidden to the tray

### macOS

* **Global shortcuts via Accessibility** — shortcuts use the macOS Accessibility API; a contextual notice in Settings guides through granting permission with a direct link to System Settings
* **Native window style** — overlay titlebar with macOS-native traffic-light controls
* **Platform-aware defaults** — tray settings hidden on macOS where they are not applicable; shortcut defaults use Command instead of Control

### Desktop Notifications

* Native OS notifications on round completion via `tauri-plugin-notification`
* Linux uses `notify-send` (libnotify) to avoid D-Bus session bus conflicts
* Notifications include round type and duration

### WebSocket Integration

* **Opt-in local WebSocket server** (default off) for stream overlays and external integrations
* Listens on `ws://127.0.0.1:<port>` (localhost only); default port 1314
* Broadcasts `roundChange` events with full timer state; clients can query current state with `getState`
* Port is configurable; enable / disable at runtime from Settings → Advanced

### Diagnostics & Logging

* **Rotating log file** written to the OS log directory (5 MB max, keeps one backup)
* Structured log entries for all major events — timer state changes, settings loads, audio playback, shortcut registration, round completions
* Panics are captured to the log before the process terminates
* **Verbose logging** toggle in Settings → Advanced switches between INFO and DEBUG level
* **Open Log Folder** button in Settings → About for quick access

### Settings

26 configurable values across seven sections — Timer, Appearance, Notifications, Audio, Shortcuts, Advanced, and About. All settings are persisted to SQLite and take effect immediately without a restart (except WebSocket port).

### Breaking Changes

* Settings are no longer stored in `user-preferences.json`. All preferences reset to defaults on first launch after upgrading from v0.x.
* The minimum OS requirement follows Tauri 2 system minimums: Windows 10+, macOS 10.13+, Linux with GTK 3.24+.
* Custom theme JSON files from v0.x are fully compatible — no changes needed.

---

<a name="v0.13.0"></a>
## [v0.13.0](https://github.com/Splode/pomotroid/compare/v0.12.0...v0.13.0)

> 2021-01-14

### Bug Fixes

* 2695b5c [#108](https://github.com/Splode/pomotroid/issues/108) Visual feedback for settings checkbox
* a26fc10 tick sounds correctly during break
* dfca313 disable menu so Ctrl+W doesn't close app
* 8f54ba7 scoop manifest hash for v0.12.0

### Chores

* b4eeef1 update various deps
* 6888de6 update various deps
* 8e34a5f add codeql github action
* 561073e update scoop manifest for v0.12.0

### Continuous Integration

* ba5c617 add electron-builder workflow

### Docs

* 4c36ec6 add contributing guide

### Features

* 432a118 default to hardware acceleration disabled
* 1919001 adjust shortcut input styling
* adf449d set tick sounds during break default to true
* 351d14a Add option to disable tick sounds during break


<a name="v0.12.0"></a>
## [v0.12.0](https://github.com/Splode/pomotroid/compare/v0.11.1...v0.12.0)

> 2020-09-06

### Bug Fixes

* d09e775 update tray icon state on timer-reset event

### Chores

* a79e03e update mini-css-extract-plugin
* badd6b5 update various deps
* 350dd44 update scoop manifest for v0.11.1


<a name="v0.11.1"></a>
## [v0.11.1](https://github.com/Splode/pomotroid/compare/v0.11.0...v0.11.1)

> 2020-07-01

### Chores

* 0a1eae9 update pomotroid scoop manifest for v0.11.0

### Features

* ca0908b update macOS runtime, build icons


<a name="v0.11.0"></a>
## [v0.11.0](https://github.com/Splode/pomotroid/compare/v0.10.0...v0.11.0)

> 2020-06-28

### Chores

* 9059f08 update scoop manifest to v0.10.0

### Docs

* f4ec338 widen theme preview image
* c7869d0 add theme preview img to README
* e06381f add AppGet install instructions to README

### Features

* dfef070 add setting to minimize to tray on close
* abf604e update macos runtime and build icons
* ae348c5 Separate 'autoStartTimer' into 'autoStartBreakTimer' and 'autoStartWorkTimer'
* c7c4d75 enhance tick sound effect


<a name="v0.10.0"></a>
## [v0.10.0](https://github.com/Splode/pomotroid/compare/v0.9.0...v0.10.0)

> 2020-06-01

### Bug Fixes

* c5ddf9f set default theme if not present in config
* 423fc19 adjust various theme accent colors
* 1386dc3 timer play and pause button using css variables
* 750c917 contrast of pomotroid theme color
* 203d3b5 theme asset pathing in production
* 87e119d color timer footer icons using theme colors
* 8e85bfa add css variables to logo in about view
* e334eab check for existence of config, create if non-existing

### Chores

* 6fe7d1b update scoop manifest for v0.9.0

### Code Refactoring

* fb5350f use userDir func in LocalStore
* ef4d51b Themer exports a singleton
* 3c57dac implement color scss variables as css variables

### Docs

* f17069d update to theme docs
* 67109a1 update README, add theme docs

### Features

* 60b5278 load custom user themes
* d82ce60 add tooltips to setting tabs
* 8242a55 tweaks to Dracula theme
* 528ac60 refine color themes
* 7b25273 refine various theme colors
* 9914bac switch Tokyo Night theme to Storm varient
* 013a521 add Synthwave theme
* 14ee6b8 add Nord theme
* 7edfdb9  add GitHub theme
* 34e3e59 add Spandex color theme
* 826c68e add Graphite theme
* 8f68133 use accent color for UI elements, increasing visual consistency
* 2104aaf add accent color to themes
* f301084 refine Dracula theme
* 0be6783 add Ayu theme
* 16019f4 refine D.Va theme
* b8eee59 add Gruvbox theme
* a9a24d3 add Andromeda theme
* 8516a6d add Solarized Light theme
* e0ebc55 add Tokyo Night theme
* b744d78 add scrolling to support many theme options
* 2ec071a add City Lights theme
* d9edcee add Popping and Locking theme
* d9fc11e add meta data to theme
* 3893368 add theme-specific styling in theme drawer
* b491721 style themes in Drawer based on themes' own style
* 1ad836c add utility methods on Themer
* 6ed3340 set theme preference
* 0914810 add pomotroid theme file
* bce8180 set tray icon color based on current theme
* fdf223a add theme support
* 36ea81a add timer tick audio setting
* 210dcba add daily log rotation for activity logs
* d19f177 log application activity to file


<a name="v0.9.0"></a>
## [v0.9.0](https://github.com/Splode/pomotroid/compare/v0.8.0...v0.9.0)

> 2020-05-25

### Bug Fixes

* 59a1ca8 capitalize app title

### Chores

* 6fc784b update scoop manifest for v0.8.0
* cf857b6 add git-chglog config

### Docs

* cc7b6e9 update CHANGELOG
* 856ab3e add CHANGELOG

### Features

* 20a56b7 improve accessibility of text elements
* 5bd03d6 add tooltips to various UI components


<a name="v0.8.0"></a>
## [v0.8.0](https://github.com/Splode/pomotroid/compare/v0.7.1...v0.8.0)

> 2020-05-09

### Chores

* ef77bc9 configure travis-ci to build on macos

### Docs

* fb4dbb8 add TravisCI build tag to README
* c508944 update screenshots, assets


<a name="v0.7.1"></a>
## [v0.7.1](https://github.com/Splode/pomotroid/compare/v0.7.0...v0.7.1)

> 2020-04-03

### Bug Fixes

* b7a2419 time display issue during round transition

### Docs

* 269cabb minor edit to README
* f0ec86e add scoop install instruction to README

### Features

* b6a7159 change "work" display to "focus"
* 010a941 add scoop manifest


<a name="v0.7.0"></a>
## [v0.7.0](https://github.com/Splode/pomotroid/compare/v0.6.2...v0.7.0)

> 2020-01-05

### Bug Fixes

* 1bcc01a TrayIcon EventBus listener uses new 'timer-tick' event
* f29a5da [#57](https://github.com/Splode/pomotroid/issues/57) Hotkey to start/stop timer

### Chores

* bbfcd35 explicitly define artifact names per build target
* bfe563b add tar archive as a linux build target
* 3730a0a add snap, deb build targets

### Code Refactoring

* 85e795a change timer tick event payload structure, key name
* 9691b95 rename local-store filename to LocalStore
* e282f74 change event-bus filename to EventBus

### Features

* 1388945 offload timer to web worker

### Style

* 51b1fbe correct linting error


<a name="v0.6.2"></a>
## [v0.6.2](https://github.com/Splode/pomotroid/compare/v0.6.1...v0.6.2)

> 2019-09-01

### Bug Fixes

* b76b045 revert to using interval in timer

### Chores

* e4b17fc build win portable and nsis versions


<a name="v0.6.1"></a>
## [v0.6.1](https://github.com/Splode/pomotroid/compare/v0.6.0...v0.6.1)

> 2019-05-17

### Chores

* eda9ed7 update node version 8 -> 10 CI configs
* 6001cb8 upgrade various deps
* 2902d64 upgrade electron builder, debug
* fcd559b upgrade electron v4 -> v5
* 742f011 update deps
* 3e87b57 update electron-debug
* 1fe34ac upgrade electron to v4
* 14c5c2c upgrade vuex to v3
* 2690bdf patch vue and vue-template compiler
* fd2ec62 major upgrade webpack to v4
* edf893c minor upgrade eslint
* c35b9a7 minor and patch dep upgrades
* 85312af minor and  patch dep upgrades

### Docs

* f130ef6 add documentation for Timer


<a name="v0.6.0"></a>
## [v0.6.0](https://github.com/Splode/pomotroid/compare/v0.5.0...v0.6.0)

> 2019-02-09

### Bug Fixes

* 83c0681 ensure timer minute values are stored and passed as numbers (int)
* 58f13b6 ensure only 1 instance of dial is animating, destroy/recreate dial
* eeabedc set timer dial length on window focus, sync RAF and timer

### Docs

* ab258cb add documentation to timer-dial methods, props validation

### Features

* fa61e23 display timer durations in notifications
* 2a8151e display color-coded icons in notifications
* 93302b6 auto-hide the volume slider after a set time, unless in use
* 6cdec04 set volume via slider, remove explicit reference to isMuted prop

### Style

* d5a4b31 format stylesheets


<a name="v0.5.0"></a>
## [v0.5.0](https://github.com/Splode/pomotroid/compare/v0.4.1...v0.5.0)

> 2019-02-04

### Chores

* a042945 use product name for artifact generation, adjust about spacing
* adf86a1 use electron-build macros for artifact name generation

### Features

* c8303ca style and structure, native open project links in about drawer
* 6aeeef3 set icon in window constructor
* 5267be0 add metadata to package.json, about drawer for project info


<a name="v0.4.1"></a>
## [v0.4.1](https://github.com/Splode/pomotroid/compare/v0.4.0...v0.4.1)

> 2019-02-02

### Bug Fixes

* 9f72b03 also set alwaysOnTop after window ready per ubuntu 18.04 bug
* b42e5c2 create single localstore instance to guard against invalidation

### Chores

* 3a037a5 add build script for ia32 arch
* 8efe921 minor dep upgrade
* 0f9d049 upgrade eslint ^5, babel ^7
* f4b839c upgrade anime major v3
* d322b7f upgrade minor and patch dep versions

### Code Refactoring

* 0573107 generate new default settings object instead of using assign
* 2c8fe5f simplify and document local-store utility

### Style

* 345b14a format consistently using prettier


<a name="v0.4.0"></a>
## [v0.4.0](https://github.com/Splode/pomotroid/compare/v0.3.0...v0.4.0)

> 2018-09-05

### Docs

* 6444525 add mini-mode to feature roadmap

### Features

* 45dbf80 add tray icon fixes and features from [@letmaik](https://github.com/letmaik)


<a name="v0.3.0"></a>
## [v0.3.0](https://github.com/Splode/pomotroid/compare/v0.2.0...v0.3.0)

> 2018-04-05


<a name="v0.2.0"></a>
## [v0.2.0](https://github.com/Splode/pomotroid/compare/v0.1.2...v0.2.0)

> 2018-03-13


<a name="v0.1.2"></a>
## [v0.1.2](https://github.com/Splode/pomotroid/compare/v0.1.1...v0.1.2)

> 2018-03-08


<a name="v0.1.1"></a>
## [v0.1.1](https://github.com/Splode/pomotroid/compare/v0.1.0...v0.1.1)

> 2018-01-31


<a name="v0.1.0"></a>
## v0.1.0

> 2018-01-30
