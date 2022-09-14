<template>
  <div id="app">
    <app-titlebar />
    <transition name="slide-left" mode="out-in">
      <app-drawer v-if="drawerOpen" />
    </transition>
    <app-timer />
    <app-notification-win v-if="os === 'win32' && notifications" />
    <app-notification v-else-if="os !== 'win32' && notifications" />
  </div>
</template>

<script>
import appDrawer from '@/components/drawer/Drawer'
import appNotification from '@/components/notification/Notification'
import appNotificationWin from '@/components/notification/Notification-win'
import appTimer from '@/components/timer/Timer'
import appTitlebar from '@/components/Titlebar'
import themer from '@/utils/Themer'

export default {
  name: 'pomotroid',

  components: {
    appDrawer,
    appNotification,
    appNotificationWin,
    appTimer,
    appTitlebar
  },

  computed: {
    // store getters
    drawerOpen() {
      return this.$store.getters.drawerOpen
    },

    alwaysOnTop() {
      return this.$store.getters.alwaysOnTop
    },

    notifications() {
      return this.$store.getters.notifications
    },

    os() {
      return this.$store.getters.os
    },

    theme() {
      return this.$store.getters.theme
    }
  },

  created() {
    if (!this.theme || this.theme === 'pomotroid') return
    themer.apply(this.theme)
  }
}
</script>

<style lang="scss">
:root {
  font-size: 5vw;
}
#app {
  animation: fade-in 0.5s ease forwards;
  position: relative;
  overflow: hidden;
  height: 100vh;
}
</style>
