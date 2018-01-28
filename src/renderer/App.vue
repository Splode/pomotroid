<template>
  <div id="app">
    <app-titlebar/>
    <app-timer/>
  </div>
</template>

<script>
  import appTimer from '@/components/Timer'
  import appTitlebar from '@/components/Titlebar'

  export default {
    name: 'pomotroid',
    components: {
      appTimer,
      appTitlebar
    }
  }
</script>

<style lang="scss">
#app {
  height: 100vh;
}
</style>
