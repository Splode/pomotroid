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

const notifier = require('node-notifier')
const path = require('path')
console.log(__static)
notifier.notify(
  {
    appName: 'com.splode.pomotroid',
    title: 'Round Completed',
    message: 'Time to take a short break',
    icon: path.join(__static, '256x256.png'),
    sound: false
  },
  (err, res) => {
    if (err) {
      console.log(err)
    }
  }
)

let myNotification = new Notification('Title', {
  body: 'Lorem Ipsum Dolor Sit Amet'
})

myNotification.onclick = () => {
  console.log('Notification clicked')
}

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
  animation: fade-in 0.5s ease forwards;
  position: relative;
  overflow: hidden;
  height: 100vh;
}
</style>
