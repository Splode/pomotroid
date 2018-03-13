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
    },
    notifyLongBreak () {
      this.callNotification({
        message: 'Begin a long break.'
      })
    },
    notifyShortBreak () {
      this.callNotification({
        message: 'Begin a short break.'
      })
    },
    notifyWork () {
      this.callNotification({
        title: 'Break Finished',
        message: 'Begin working.'
      })
    }
  },

  mounted () {
    EventBus.$on('ready-long-break', this.notifyLongBreak)
    EventBus.$on('ready-short-break', this.notifyShortBreak)
    EventBus.$on('ready-work', this.notifyWork)
  },

  beforeDestroy () {
    EventBus.$off('ready-long-break', this.notifyLongBreak)
    EventBus.$off('ready-short-break', this.notifyShortBreak)
    EventBus.$off('ready-work', this.notifyWork)
  }
}
</script>
