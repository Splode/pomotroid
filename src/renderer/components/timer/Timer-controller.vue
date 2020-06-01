<template></template>

<script>
import { EventBus } from '@/utils/EventBus'
import { logger } from '@/utils/logger'

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
        this.$store.dispatch('incrementTotalWorkRounds')
        EventBus.$emit('ready-long-break')
        logger.info('focus round completed')
      } else if (this.currentRound === 'work') {
        this.$store.dispatch('setCurrentRound', 'short-break')
        this.$store.dispatch('incrementTotalWorkRounds')
        EventBus.$emit('ready-short-break')
        logger.info('focus round completed')
      } else if (this.currentRound === 'short-break') {
        this.$store.dispatch('setCurrentRound', 'work')
        this.$store.dispatch('incrementRound')
        EventBus.$emit('ready-work')
        logger.info('short break completed')
      } else if (this.currentRound === 'long-break') {
        this.$store.dispatch('setCurrentRound', 'work')
        this.$store.dispatch('resetRound')
        EventBus.$emit('ready-work')
        logger.info('long break completed')
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
