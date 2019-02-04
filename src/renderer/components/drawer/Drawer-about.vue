<template>
  <div class="Container">
    <p class="Drawer-heading">About</p>
    <h2>Pomotroid</h2>
    <p>Version: {{ version }}</p>
  </div>
</template>

<script>
import * as pckg from './../../../../package.json'
export default {
  name: 'DrawerAbout',
  data() {
    return {
      version: pckg.version
    }
  }
}
</script>

<style>

</style>
