
<a name="v0.8.0"></a>
## [v0.8.0](https://github.com/Splode/pomotroid/compare/v0.7.1...v0.8.0)

> 2020-05-09

### Chore

* cf857b6 add git-chglog config
* ef77bc9 configure travis-ci to build on macos

### Docs

* 856ab3e add CHANGELOG
* c508944 update screenshots, assets

### Fix

* 59a1ca8 capitalize app title


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

