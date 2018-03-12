<template>
</template>

<script>
import { EventBus } from '@/utils/event-bus'
const path = require('path')

export default {
  name: 'Notification',

  data () {
    return {
      notification: null
    }
  },

  methods: {
    callNotification (opts) {
      this.notification = new Notification(opts.title, {
        body: opts.body,
        icon: path.join('static', 'icon.png')
      })
    }
  },

  mounted () {
    EventBus.$on('ready-long-break', () => {
      this.callNotification({
        title: 'Work Round Complete',
        body: 'Begin a long break.'
      })
    })

    EventBus.$on('ready-short-break', () => {
      this.callNotification({
        title: 'Work Round Complete',
        body: 'Begin a short break.'
      })
    })

    EventBus.$on('ready-work', () => {
      this.callNotification({
        title: 'Break Finished',
        body: 'Begin working.'
      })
    })
  }
}
</script>
