<template>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

const notifier = require('node-notifier')
const path = require('path')

export default {
  name: 'Notification-win',

  methods: {
    callNotification (opts) {
      notifier.notify({
        appName: 'com.splode.pomotroid',
        title: opts.title || 'Work Round Complete',
        message: opts.message,
        icon: path.join(__static, 'icon.png'),
        sound: false
      }, (err, res) => {
        if (err) {
          console.log(err)
        }
      })
    }
  },

  mounted () {
    EventBus.$on('ready-long-break', () => {
      this.callNotification({
        message: 'Begin a long break.'
      })
    })

    EventBus.$on('ready-short-break', () => {
      this.callNotification({
        message: 'Begin a short break.'
      })
    })

    EventBus.$on('ready-work', () => {
      this.callNotification({
        title: 'Break Finished',
        message: 'Begin working.'
      })
    })
  }
}
</script>
