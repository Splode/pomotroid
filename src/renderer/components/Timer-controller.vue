<template>
  
</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  computed: {
    // store getters
    currentRound () {
      return this.$store.getters.currentRound
    },

    round () {
      return this.$store.getters.round
    },

    workRounds () {
      return this.$store.getters.workRounds
    }
  },

  methods: {
    // check round
    checkRound () {
      if (this.currentRound === 'work' && this.currentRound >= this.workRounds) {
        // set currentRound to long-break
        console.log('long-break ready')
      } else if (this.currentRound === 'work') {
        // set currentRound to short-break
        this.$store.dispatch('setCurrentRound', 'short-break')
        console.log('short-break ready')
      } else if (this.currentRound === 'short-break') {
        // set currentRound to work
        this.$store.dispatch('setCurrentRound', 'work')
        // increment round
        this.$store.dispatch('incrementRound')
        console.log('short-break finished, work ready, increment round')
      } else if (this.currentRound === 'long-break') {
        // set currentRound to work
        // reset round to 1
        console.log('long-break finished, reset round')
      }
      this.dispatchTimer()
    },
    // call for new timer
    dispatchTimer () {
      EventBus.$emit('timer-init')
    }
  },

  mounted () {
    EventBus.$on('timer-completed', () => {
      this.checkRound()
    })
  }
}
</script>
