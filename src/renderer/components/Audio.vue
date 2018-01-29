<template>
  <audio id="timer-audio" :src="'./static/audio/blop.mp3'"></audio>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  data () {
    return {
      timerAudio: null
    }
  },

  computed: {
    // store getters
    isMuted () {
      return this.$store.getters.isMuted
    }
  },

  mounted () {
    this.timerAudio = document.getElementById('timer-audio')
    EventBus.$on('timer-completed', () => {
      if (!this.isMuted) {
        this.timerAudio.play()
      }
    })
  }
}
</script>

<style>

</style>
