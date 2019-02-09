<template>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

const notifier = require('node-notifier')
const path = require('path')

export default {
  name: 'Notification-win',

  computed: {
    // store getters
    timeLongBreak() {
      return this.$store.getters.timeLongBreak
    },
    timeShortBreak() {
      return this.$store.getters.timeShortBreak
    },
    timeWork() {
      return this.$store.getters.timeWork
    }
  },

  methods: {
    callNotification(opts) {
      notifier.notify(
        {
          appName: 'com.splode.pomotroid',
          title: opts.title || 'Work Round Complete',
          message: opts.message,
          icon: opts.icon || path.join(__static, 'icon.png'),
          sound: false
        },
        (err, res) => {
          if (err) {
            console.log(err)
          }
        }
      )
    },

    notifyLongBreak() {
      this.callNotification({
        message: `Begin a ${this.timeLongBreak} minute long break.`,
        icon: path.join(__static, 'icon--blue.png')
      })
    },

    notifyShortBreak() {
      this.callNotification({
        message: `Begin a ${this.timeShortBreak} minute short break.`,
        icon: path.join(__static, 'icon--green.png')
      })
    },

    notifyWork() {
      this.callNotification({
        title: 'Break Finished',
        message: `Begin working for ${this.timeWork} minutes.`
      })
    }
  },

  mounted() {
    EventBus.$on('ready-long-break', this.notifyLongBreak)
    EventBus.$on('ready-short-break', this.notifyShortBreak)
    EventBus.$on('ready-work', this.notifyWork)
  },

  beforeDestroy() {
    EventBus.$off('ready-long-break', this.notifyLongBreak)
    EventBus.$off('ready-short-break', this.notifyShortBreak)
    EventBus.$off('ready-work', this.notifyWork)
  }
}
</script>
