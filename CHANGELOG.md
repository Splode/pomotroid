[Unreleased]
-----------

### Localization

* **Turkish** — UI translation corrections contributed by [@NandeMD](https://github.com/NandeMD).

[v1.5.0] - 2026-04-06
-----------

### Bug Fixes

* **App freeze / crash on launch when "Show in System Tray" is enabled on KDE Plasma 6 / Wayland** — enabling the system tray on a Linux system without `libayatana-appindicator3` or `libappindicator3` installed caused `libappindicator-sys` to panic inside an `extern "C"` function, which aborts the process. Because the setting is persisted before the crash, every subsequent launch would abort before the window could appear. The fix probes for the shared library via `dlopen` before calling `TrayIconBuilder::build()` and returns early with a warning when it is absent. The System Tray section in Settings → System is now hidden entirely on Linux systems where the library is not found, preventing the setting from being enabled in the first place. The required library can be installed on Arch / Manjaro with `sudo pacman -S libayatana-appindicator`.

### Shortcuts

* **Local keyboard shortcuts** — a set of keyboard shortcuts now activates while the main timer window has focus, with no system-wide registration required. Default bindings: Space (pause/resume), Left Arrow (reset current round), Right Arrow (skip round), Down Arrow (volume down), Up Arrow (volume up), M (mute toggle), F11 (fullscreen toggle). All seven bindings are re-mappable in Settings → Shortcuts under the new Local Shortcuts section, which appears above the existing Global Shortcuts section. Bindings persist across restarts and are restored to defaults when Reset All Settings is used.

### UI

* **Collapsible theme pickers** — the Light Theme and Dark Theme pickers in Settings → Appearance are now collapsible rows instead of two permanently expanded lists. Each row shows the configured theme name and a color chip (round-type swatches on the theme's own background color) when collapsed. Clicking a row expands its full theme list; opening one automatically closes the other. A checkmark marks the selected theme in either picker regardless of which mode is currently active.

[v1.4.0] - 2026-03-30
-----------

### Bug Fixes

* **System tray context menu showing "Pause" when timer is idle** — when auto-start work was disabled, the tray menu kept showing "Pause" after a break finished instead of resetting to "Start". The round-complete handler updated the tray icon but never updated the menu label when auto-start was suppressed; only the subsequent `Started` event (which never fired) would have corrected it. The menu is now explicitly reset to "Start" whenever a new round begins without auto-starting.

### UI

* **Tooltips** — hovering over timer controls (Restart Round, Skip, Reset, Mute/Unmute, round indicator) and titlebar buttons (Settings, Statistics) now shows a brief tooltip after 600 ms. Settings toggles for System Tray (Linux), Verbose Logging, and WebSocket Server gain an inline ⓘ icon with an instant tooltip. On Linux the System Tray tooltip explains the GNOME AppIndicator requirement. Tooltips use `position: fixed` to avoid clipping near window edges and flip below the trigger when too close to the top of the screen.

[v1.3.0] - 2026-03-23
-----------

### Themes

* **Crimson White** — new light theme with a white background and deep crimson accents. Contributed by [@Hungerdream](https://github.com/Hungerdream).

### Bug Fixes

* **Custom alert sound names resetting to generic filenames after restart** — after setting a custom alert sound, Settings would correctly display the original filename (e.g. `my_alert.mp3`) during the session, but after restarting the app the name reverted to the internal copy name (e.g. `custom_work_alert.mp3`). The original filename was returned to the frontend on first selection but never persisted; on startup the audio engine re-scanned the data directory and could only recover the generic copy name. The original display name is now saved to the settings database when a custom sound is set and removed when it is cleared, so the correct name is restored on every launch.
* **Settings and Statistics windows left orphaned when main window is closed** — closing the main window while Settings or Statistics was open left those windows running with no way to reopen the main window. Both child windows are now closed automatically whenever the main window is truly closed (not when hiding to tray).
* **macOS auto-update failing with "platform not found"** — the updater manifest (`latest.json`) used `darwin-universal` as the macOS platform key, which Tauri 2's updater does not recognise; it checks only for `darwin-aarch64` (Apple Silicon) and `darwin-x86_64` (Intel). Additionally, the release workflow pointed the updater at the `.dmg` distribution file rather than the `.app.tar.gz` bundle that the Tauri updater downloads and applies in-place. Both are fixed: the manifest now lists `darwin-aarch64` and `darwin-x86_64` entries, and the workflow uploads and references the `.app.tar.gz` artifact.

### Timer

* **Optional breaks** — short breaks and long breaks can now be independently disabled in Settings → Timer. Disabling short breaks chains work rounds back-to-back; disabling long breaks substitutes a short break at the cycle boundary so the round counter resets cleanly. When both are disabled the timer runs in a pure work loop. Duration sliders for disabled break types are dimmed to indicate they have no effect. When long breaks are disabled the round indicator switches from the `X / Y` cycle counter to a rolling "round N" session counter that increments continuously and resets only when the timer is reset.

[v1.2.0] - 2026-03-16
-----------

### Features

* **In-app auto-update** — Pomotroid now checks for new releases automatically when the Settings window opens. If an update is available, an "Install vX.Y.Z" button appears in Settings → About; clicking it downloads, verifies, and installs the update then relaunches immediately. Bundles are Ed25519-signed in CI and verified by the updater before installation. Automatic checking can be disabled via the new "Check for Updates Automatically" toggle in Settings → System. Linux users who installed via `.deb` or `.rpm` are not affected — in-app updates apply to AppImage only; package-manager installations should update through their respective package manager.

#### macOS

* **Native window management controls restored** — the macOS Window menu now includes the full set of standard window-management items (Fill, Center, Move & Resize with Halves/Quarters/Arrange submenus, Full Screen Tile, Bring All to Front) that were absent after the Electron → Tauri rewrite. The green traffic-light button also gains its tiling popup (arrange left/right/center). Both are enabled by setting `NSWindowCollectionBehaviorManaged` on the underlying `NSWindow` and registering the Window menu with `NSApplication.setWindowsMenu:` at startup.

### Settings

* **Data management in Settings → System** — a new Data section in Settings → System provides two actions: _Clear Session History_ permanently deletes all recorded session data, and _Reset All Settings_ restores every setting to its default value. Both actions require inline confirmation before executing. The Reset All Settings action has been moved here from Settings → About, and the Statistics window refreshes immediately when session history is cleared.
* **Global shortcuts are now opt-in** ⚠️ **BREAKING** — global keyboard shortcuts are disabled by default on all platforms. A new _Enable Global Shortcuts_ toggle in Settings → Shortcuts controls them as a unit; individual shortcut bindings are preserved and editable only while shortcuts are enabled. Existing users will find shortcuts disabled after upgrading and must re-enable them once in Settings → Shortcuts. This matches the behaviour of the WebSocket server, which is also off by default.

### Bug Fixes

* **Round count not updating while timer is active** — changing the number of work rounds in Settings while the timer is running or paused now immediately updates the round indicator in the main UI. Previously the frontend only received a state snapshot when the timer was idle, so the "X of Y" display required a manual Reset to reflect the new total. The snapshot is now always broadcast after a settings change; the active countdown is unaffected since the `timer:reset` event only updates the UI store, and any momentary `total_secs` discrepancy is corrected by the next `timer:tick` within one second.
* **Timer text jitter on macOS** — the countdown display no longer shifts horizontally as digits change while the timer is running. The root cause was that Mona Sans does not include tabular (equal-width) numeral variants, so digits like `1` and `0` have different widths. macOS Core Text measures and positions glyphs with subpixel fractional precision, making the width difference between digits visible as a layout shift on each tick. Other platforms round glyph advances to whole pixels, hiding the effect. The timer now uses Mona Sans Mono, which is inherently fixed-width and eliminates the width variation entirely.

[v1.1.0] - 2026-03-09
-----------

### Bug Fixes

* **Theme watcher infinite reload loop** — the custom theme hot-reload watcher no longer floods the log file with continuous theme-scan messages on startup. notify v8's inotify backend monitors `IN_OPEN` events by default; every `read_dir()` call inside the watcher's own reload triggered a new open event, causing a cycle that fired ~2 full theme scans per second indefinitely. Read-only access events (`IN_OPEN`, `IN_CLOSE_NOWRITE`) and pure metadata changes (`IN_ATTRIB` / atime updates) are now filtered out in the debounce loop so only actual file writes, creates, deletes, and renames trigger a theme reload.
* **White screen flash on startup** — the Settings and Statistics windows no longer display a blank white frame before styles load. Windows are now created hidden and shown only after the theme has been applied.

### Localization

* **Turkish language support** — Turkish (`tr`) is now available as a language option in Settings → System. All 109 UI strings, settings labels, and notification messages are translated.

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

### Integrations

* **WebSocket timer events** — the opt-in WebSocket server now broadcasts the full timer lifecycle to connected clients. New message types: `started` (timer begins from idle, carries `total_secs`), `paused` (carries `elapsed_secs`), `resumed` (carries `elapsed_secs`), and `reset`. The existing `roundChange` message is unchanged. A matching `timer:started` Tauri event is also emitted for internal listeners.

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
