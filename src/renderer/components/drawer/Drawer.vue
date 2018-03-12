<template>
  <div class="Drawer">
    <app-settings-timer/>
    <app-drawer-menu/>
  </div>
</template>

<script>
import appDrawerMenu from '@/components/drawer/Drawer-menu'
import appSettingsTimer from '@/components/drawer/Settings-timer'

export default {
  components: {
    appDrawerMenu,
    appSettingsTimer
  }
}
</script>

<style lang="scss" scoped>
.Drawer {
  background-color: $colorLightNavy;
  // position: absolute;
  position: relative;
  width: 100%;
  // height: 100%;
  height: calc(100% - 68px);
  z-index: 1;
  -webkit-app-region: no-drag;
}
</style>
