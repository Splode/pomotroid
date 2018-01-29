<template>
  <div id="app">
    <app-titlebar/>
    <transition name="slide-left" mode="out-in">
      <app-drawer v-if="drawerOpen"/>
    </transition>
    <app-timer/>
  </div>
</template>

<script>
import appDrawer from '@/components/drawer/Drawer'
import appTimer from '@/components/timer/Timer'
import appTitlebar from '@/components/Titlebar'

export default {
  name: 'pomotroid',
  components: {
    appDrawer,
    appTimer,
    appTitlebar
  },

  computed: {
    // store getters
    drawerOpen () {
      return this.$store.getters.drawerOpen
    }
  }
}
</script>

<style lang="scss">
#app {
  position: relative;
  overflow: hidden;
  height: 100vh;
}
</style>
