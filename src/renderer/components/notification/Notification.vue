<template>
</template>

<script>
import { EventBus } from '@/utils/event-bus'
const path = require('path')

export default {
  name: 'Notification',

  data() {
    return {
      notification: null
    }
  },

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
      this.notification = new Notification(opts.title, {
        body: opts.body,
        icon: opts.icon || path.join('static', 'icon.png'),
        silent: true
      })
    },

    notifyLongBreak() {
      this.callNotification({
        title: 'Work Round Complete',
        body: `Begin a ${this.timeLongBreak} minute long break.`,
        icon: path.join('static', 'icon--blue.png')
      })
    },

    notifyShortBreak() {
      this.callNotification({
        title: 'Work Round Complete',
        body: `Begin a ${this.timeShortBreak} minute short break.`,
        icon: path.join('static', 'icon--green.png')
      })
    },

    notifyWork() {
      this.callNotification({
        title: 'Break Finished',
        body: `Begin working for ${this.timeWork} minutes.`
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
