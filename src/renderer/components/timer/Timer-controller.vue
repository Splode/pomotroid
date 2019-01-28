<template>

</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  computed: {
    // store getters
    autoStartTimer() {
      return this.$store.getters.autoStartTimer
    },

    currentRound() {
      return this.$store.getters.currentRound
    },

    round() {
      return this.$store.getters.round
    },

    workRounds() {
      return this.$store.getters.workRounds
    }
  },

  methods: {
    checkRound() {
      if (this.currentRound === 'work' && this.round >= this.workRounds) {
        this.$store.dispatch('setCurrentRound', 'long-break')
        EventBus.$emit('ready-long-break')
        console.log('long-break ready')
      } else if (this.currentRound === 'work') {
        this.$store.dispatch('setCurrentRound', 'short-break')
        EventBus.$emit('ready-short-break')
        console.log('short-break ready')
      } else if (this.currentRound === 'short-break') {
        this.$store.dispatch('setCurrentRound', 'work')
        this.$store.dispatch('incrementRound')
        EventBus.$emit('ready-work')
        console.log('short-break finished, work ready, increment round')
      } else if (this.currentRound === 'long-break') {
        this.$store.dispatch('setCurrentRound', 'work')
        this.$store.dispatch('resetRound')
        EventBus.$emit('ready-work')
        console.log('long-break finished, reset round')
      }
      this.dispatchTimer()
    },
    dispatchTimer() {
      EventBus.$emit('timer-init', {
        auto: this.autoStartTimer
      })
    }
  },

  mounted() {
    EventBus.$on('timer-completed', () => {
      this.checkRound()
    })
  }
}
</script>
