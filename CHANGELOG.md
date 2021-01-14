
<a name="v0.13.0"></a>
## [v0.13.0](https://github.com/Splode/pomotroid/compare/v0.12.0...v0.13.0)

> 2021-01-14

### Chore

* b4eeef1 update various deps
* 6888de6 update various deps
* 8e34a5f add codeql github action
* 561073e update scoop manifest for v0.12.0

### Ci

* ba5c617 add electron-builder workflow

### Docs

* 4c36ec6 add contributing guide

### Feat

* 432a118 default to hardware acceleration disabled
* 1919001 adjust shortcut input styling
* adf449d set tick sounds during break default to true
* 351d14a Add option to disable tick sounds during break

### Fix

* 2695b5c [#108](https://github.com/Splode/pomotroid/issues/108) Visual feedback for settings checkbox
* a26fc10 tick sounds correctly during break
* dfca313 disable menu so Ctrl+W doesn't close app
* 8f54ba7 scoop manifest hash for v0.12.0


<a name="v0.12.0"></a>
## [v0.12.0](https://github.com/Splode/pomotroid/compare/v0.11.1...v0.12.0)

> 2020-09-06

### Chore

* a79e03e update mini-css-extract-plugin
* badd6b5 update various deps
* 350dd44 update scoop manifest for v0.11.1

### Fix

* d09e775 update tray icon state on timer-reset event


<a name="v0.11.1"></a>
## [v0.11.1](https://github.com/Splode/pomotroid/compare/v0.11.0...v0.11.1)

> 2020-07-01

### Chore

* 0a1eae9 update pomotroid scoop manifest for v0.11.0

### Feat

* ca0908b update macOS runtime, build icons


<a name="v0.11.0"></a>
## [v0.11.0](https://github.com/Splode/pomotroid/compare/v0.10.0...v0.11.0)

> 2020-06-28

### Chore

* 9059f08 update scoop manifest to v0.10.0

### Docs

* f4ec338 widen theme preview image
* c7869d0 add theme preview img to README
* e06381f add AppGet install instructions to README

### Feat

* dfef070 add setting to minimize to tray on close
* abf604e update macos runtime and build icons
* ae348c5 Separate 'autoStartTimer' into 'autoStartBreakTimer' and 'autoStartWorkTimer'
* c7c4d75 enhance tick sound effect


<a name="v0.10.0"></a>
## [v0.10.0](https://github.com/Splode/pomotroid/compare/v0.9.0...v0.10.0)

> 2020-06-01

### Chore

* 6fe7d1b update scoop manifest for v0.9.0

### Docs

* f17069d update to theme docs
* 67109a1 update README, add theme docs

### Feat

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

### Fix

* c5ddf9f set default theme if not present in config
* 423fc19 adjust various theme accent colors
* 1386dc3 timer play and pause button using css variables
* 750c917 contrast of pomotroid theme color
* 203d3b5 theme asset pathing in production
* 87e119d color timer footer icons using theme colors
* 8e85bfa add css variables to logo in about view
* e334eab check for existence of config, create if non-existing

### Refactor

* fb5350f use userDir func in LocalStore
* ef4d51b Themer exports a singleton
* 3c57dac implement color scss variables as css variables


<a name="v0.9.0"></a>
## [v0.9.0](https://github.com/Splode/pomotroid/compare/v0.8.0...v0.9.0)

> 2020-05-25

### Chore

* 6fc784b update scoop manifest for v0.8.0
* cf857b6 add git-chglog config

### Docs

* cc7b6e9 update CHANGELOG
* 856ab3e add CHANGELOG

### Feat

* 20a56b7 improve accessibility of text elements
* 5bd03d6 add tooltips to various UI components

### Fix

* 59a1ca8 capitalize app title


<a name="v0.8.0"></a>
## [v0.8.0](https://github.com/Splode/pomotroid/compare/v0.7.1...v0.8.0)

> 2020-05-09

### Chore

* ef77bc9 configure travis-ci to build on macos

### Docs

* fb4dbb8 add TravisCI build tag to README
* c508944 update screenshots, assets


<a name="v0.7.1"></a>
## [v0.7.1](https://github.com/Splode/pomotroid/compare/v0.7.0...v0.7.1)

> 2020-04-03

### Docs

* 269cabb minor edit to README
* f0ec86e add scoop install instruction to README

### Feat

* b6a7159 change "work" display to "focus"
* 010a941 add scoop manifest

### Fix

* b7a2419 time display issue during round transition


<a name="v0.7.0"></a>
## [v0.7.0](https://github.com/Splode/pomotroid/compare/v0.6.2...v0.7.0)

> 2020-01-05

### Chore

* bbfcd35 explicitly define artifact names per build target
* bfe563b add tar archive as a linux build target
* 3730a0a add snap, deb build targets

### Feat

* 1388945 offload timer to web worker

### Fix

* 1bcc01a TrayIcon EventBus listener uses new 'timer-tick' event
* f29a5da [#57](https://github.com/Splode/pomotroid/issues/57) Hotkey to start/stop timer

### Refactor

* 85e795a change timer tick event payload structure, key name
* 9691b95 rename local-store filename to LocalStore
* e282f74 change event-bus filename to EventBus

### Style

* 51b1fbe correct linting error


<a name="v0.6.2"></a>
## [v0.6.2](https://github.com/Splode/pomotroid/compare/v0.6.1...v0.6.2)

> 2019-09-01

### Chore

* e4b17fc build win portable and nsis versions

### Fix

* b76b045 revert to using interval in timer


<a name="v0.6.1"></a>
## [v0.6.1](https://github.com/Splode/pomotroid/compare/v0.6.0...v0.6.1)

> 2019-05-17

### Chore

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

### Docs

* ab258cb add documentation to timer-dial methods, props validation

### Feat

* fa61e23 display timer durations in notifications
* 2a8151e display color-coded icons in notifications
* 93302b6 auto-hide the volume slider after a set time, unless in use
* 6cdec04 set volume via slider, remove explicit reference to isMuted prop

### Fix

* 83c0681 ensure timer minute values are stored and passed as numbers (int)
* 58f13b6 ensure only 1 instance of dial is animating, destroy/recreate dial
* eeabedc set timer dial length on window focus, sync RAF and timer

### Style

* d5a4b31 format stylesheets


<a name="v0.5.0"></a>
## [v0.5.0](https://github.com/Splode/pomotroid/compare/v0.4.1...v0.5.0)

> 2019-02-04

### Chore

* a042945 use product name for artifact generation, adjust about spacing
* adf86a1 use electron-build macros for artifact name generation

### Feat

* c8303ca style and structure, native open project links in about drawer
* 6aeeef3 set icon in window constructor
* 5267be0 add metadata to package.json, about drawer for project info


<a name="v0.4.1"></a>
## [v0.4.1](https://github.com/Splode/pomotroid/compare/v0.4.0...v0.4.1)

> 2019-02-02

### Chore

* 3a037a5 add build script for ia32 arch
* 8efe921 minor dep upgrade
* 0f9d049 upgrade eslint ^5, babel ^7
* f4b839c upgrade anime major v3
* d322b7f upgrade minor and patch dep versions

### Fix

* 9f72b03 also set alwaysOnTop after window ready per ubuntu 18.04 bug
* b42e5c2 create single localstore instance to guard against invalidation

### Refactor

* 0573107 generate new default settings object instead of using assign
* 2c8fe5f simplify and document local-store utility

### Style

* 345b14a format consistently using prettier


<a name="v0.4.0"></a>
## [v0.4.0](https://github.com/Splode/pomotroid/compare/v0.3.0...v0.4.0)

> 2018-09-05

### Docs

* 6444525 add mini-mode to feature roadmap

### Feat

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

