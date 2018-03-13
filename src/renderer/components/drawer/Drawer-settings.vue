<template>
  <div class="Container">
    <p class="Drawer-heading">Settings</p>
    <div class="Setting-wrapper">
      <p class="Setting-title">Always On Top</p>
      <div class="Checkbox" @click="selectAlwaysOnTop" :class="alwaysOnTop ? 'is-active' : 'is-inactive'"></div>
    </div>
    <div class="Setting-wrapper">
      <p class="Setting-title">Desktop Notifications</p>
      <div class="Checkbox" @click="selectNotifications" :class="notifications ? 'is-active' : 'is-inactive'"></div>
    </div>
  </div>
</template>

<script>
import { ipcRenderer } from 'electron'

export default {
  name: 'Drawer-settings',

  computed: {
    alwaysOnTop () {
      return this.$store.getters.alwaysOnTop
    },

    notifications () {
      return this.$store.getters.notifications
    }
  },

  methods: {
    selectAlwaysOnTop () {
      const payload = {
        key: 'alwaysOnTop',
        val: !this.alwaysOnTop
      }
      ipcRenderer.send('toggle-alwaysOnTop', !this.alwaysOnTop)
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    },

    selectNotifications () {
      const payload = {
        key: 'notifications',
        val: !this.notifications
      }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
    }
  }
}
</script>

<style lang="scss" scoped>
.Checkbox {
  background-color: $colorNavy;
  border: 2px solid $colorBlueGrey;
  border-radius: 100%;
  display: inline-block;
  transition: $transitionDefault;
  width: 16px;
  height: 16px;
  &:hover {
    border-color: $colorRed;
  }
  &.is-active {
    background-color: $colorRed;
    border-color: $colorNavy;
    &:hover {
      background-color: $colorNavy;
      border-color: $colorRed;
    }
  }
}

.Setting-wrapper {
  background-color: $colorNavy;
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  margin: 12px 0;
  padding: 12px;
}

.Setting-title {
  color: $colorBlueGrey;
  font-size: 14px;
  letter-spacing: .05em;
}
</style>
