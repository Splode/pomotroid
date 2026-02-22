<template>
  <div class="Container">
    <p class="Drawer-heading">Settings</p>
    <div class="Setting-wrapper">
      <p class="Setting-title">Always On Top</p>
      <div
        class="Checkbox"
        @click="selectAlwaysOnTop"
        :class="alwaysOnTop ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <transition name="slide-left" mode="in-out">
      <div class="Setting-wrapper" v-show="alwaysOnTop">
        <p class="Setting-title">Deactivate Always On Top on Breaks</p>
        <div
          class="Checkbox"
          @click="selectBreakAlwaysOnTop"
          :class="breakAlwaysOnTop ? 'is-active' : 'is-inactive'"
          ref="breakAlwaysOnTopCheckbox"
        ></div>
      </div>
    </transition>
    <div class="Setting-wrapper">
      <p class="Setting-title">Auto-start Work Timer</p>
      <div
        class="Checkbox"
        @click="selectAutoStartWorkTimer"
        :class="autoStartWorkTimer ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Auto-start Break Timer</p>
      <div
        class="Checkbox"
        @click="selectAutoStartBreakTimer"
        :class="autoStartBreakTimer ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Tick Sounds - Work</p>
      <div
        class="Checkbox"
        @click="selectTickSounds"
        :class="tickSounds ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Tick Sounds - Break</p>
      <div
        class="Checkbox"
        @click="selectTickSoundsDuringBreak"
        :class="tickSoundsDuringBreak ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Desktop Notifications</p>
      <div
        class="Checkbox"
        @click="selectNotifications"
        :class="notifications ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Minimize to Tray</p>
      <div
        class="Checkbox"
        @click="selectMinToTray"
        :class="minToTray ? 'is-active' : 'is-inactive'"
      ></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Minimize to Tray on Close</p>
      <div
        class="Checkbox"
        @click="selectMinToTrayOnClose"
        :class="minToTrayOnClose ? 'is-active' : 'is-inactive'"
      ></div>
    </div>

    <p class="Drawer-heading">Global Shortcuts</p>

    <div class="Setting-wrapper">
      <p class="Setting-title">Toggle Timer</p>
      <shortcut-input
        :value="globalShortcuts['call-timer-toggle']"
        @input="shortcut => setGlobalShortcut('call-timer-toggle', shortcut)"
      />
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Reset Timer</p>
      <shortcut-input
        :value="globalShortcuts['call-timer-reset']"
        @input="shortcut => setGlobalShortcut('call-timer-reset', shortcut)"
      />
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Skip Round</p>
      <shortcut-input
        :value="globalShortcuts['call-timer-skip']"
        @input="shortcut => setGlobalShortcut('call-timer-skip', shortcut)"
      />
    </div>
  </div>
</template>

<script>
import { ipcRenderer } from 'electron'
import ShortcutInput from '../ShortcutInput'

export default {
  name: 'Drawer-settings',

  computed: {
    alwaysOnTop() {
      return this.$store.getters.alwaysOnTop
    },

    breakAlwaysOnTop() {
      return this.$store.getters.breakAlwaysOnTop
    },

    autoStartWorkTimer() {
      return this.$store.getters.autoStartWorkTimer
    },

    autoStartBreakTimer() {
      return this.$store.getters.autoStartBreakTimer
    },

    minToTray() {
      return this.$store.getters.minToTray
    },

    minToTrayOnClose() {
      return this.$store.getters.minToTrayOnClose
    },

    notifications() {
      return this.$store.getters.notifications
    },

    os() {
      return this.$store.getters.os
    },

    tickSounds() {
      return this.$store.getters.tickSounds
    },

    tickSoundsDuringBreak() {
      return this.$store.getters.tickSoundsDuringBreak
    },

    globalShortcuts() {
      return this.$store.getters.globalShortcuts
    }
  },

  methods: {
    selectAlwaysOnTop() {
      const payload = {
        key: 'alwaysOnTop',
        val: !this.alwaysOnTop
      }
      ipcRenderer.send('toggle-alwaysOnTop', !this.alwaysOnTop)
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)

      if (this.alwaysOnTop === false && this.breakAlwaysOnTop === true) {
        this.$refs.breakAlwaysOnTopCheckbox.click()
      }
    },

    selectBreakAlwaysOnTop() {
      const payload = {
        key: 'breakAlwaysOnTop',
        val: !this.breakAlwaysOnTop
      }

      ipcRenderer.send('toggle-breakAlwaysOnTop', !this.breakAlwaysOnTop)
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectAutoStartWorkTimer() {
      const payload = {
        key: 'autoStartWorkTimer',
        val: !this.autoStartWorkTimer
      }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectAutoStartBreakTimer() {
      const payload = {
        key: 'autoStartBreakTimer',
        val: !this.autoStartBreakTimer
      }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectMinToTray() {
      const payload = {
        key: 'minToTray',
        val: !this.minToTray
      }
      ipcRenderer.send('toggle-minToTray', !this.minToTray)
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectMinToTrayOnClose() {
      const payload = {
        key: 'minToTrayOnClose',
        val: !this.minToTrayOnClose
      }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectNotifications() {
      const payload = {
        key: 'notifications',
        val: !this.notifications
      }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectTickSounds() {
      this.$store.dispatch('setTickSounds', !this.tickSounds)
    },

    selectTickSoundsDuringBreak() {
      this.$store.dispatch(
        'setTickSoundsDuringBreak',
        !this.tickSoundsDuringBreak
      )
    },

    setGlobalShortcut(event, shortcut) {
      const newShortcut = JSON.parse(JSON.stringify(this.globalShortcuts))
      newShortcut[event] = shortcut
      this.$store.dispatch('setGlobalShortcuts', newShortcut)
      ipcRenderer.send('reload-global-shortcuts', newShortcut)
    }
  },
  components: {
    ShortcutInput
  }
}
</script>

<style lang="scss" scoped>
.Checkbox {
  background-color: var(--color-background);
  border: 2px solid var(--color-background-lightest);
  border-radius: 100%;
  display: inline-block;
  transition: $transitionDefault;
  width: 16px;
  height: 16px;
  &:hover {
    border-color: var(--color-accent);
  }
  &.is-active {
    background-color: var(--color-accent);
    border-color: var(--color-background);
    &:hover {
      background-color: var(--color-accent);
      border-color: var(--color-background-lightest);
    }
  }
}

.Container {
  max-height: calc(100% - 36px);
  overflow-y: auto;
}

.Setting-wrapper {
  background-color: var(--color-background);
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  margin: 12px 0;
  padding: 12px;
}

.Setting-title {
  color: var(--color-foreground-darker);
  font-size: 14px;
  letter-spacing: 0.05em;
}
</style>
