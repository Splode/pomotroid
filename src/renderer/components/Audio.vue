<template>
  <div>
    <audio id="audio-long-break" :src="'./static/audio/alert-long-break.mp3'"></audio>
    <audio id="audio-short-break" :src="'./static/audio/alert-short-break.mp3'"></audio>
    <audio id="audio-work" :src="'./static/audio/alert-work.mp3'"></audio>
  </div>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  data () {
    return {
      audioLongBreak: null,
      audioShortBreak: null,
      audioWork: null
    }
  },

  computed: {
    // store getters
    isMuted () {
      return this.$store.getters.isMuted
    }
  },

  mounted () {
    this.audioLongBreak = document.getElementById('audio-long-break')
    this.audioShortBreak = document.getElementById('audio-short-break')
    this.audioWork = document.getElementById('audio-work')

    EventBus.$on('ready-long-break', () => {
      if (!this.isMuted) {
        this.audioLongBreak.play()
      }
    })

    EventBus.$on('ready-short-break', () => {
      if (!this.isMuted) {
        this.audioShortBreak.play()
      }
    })

    EventBus.$on('ready-work', () => {
      if (!this.isMuted) {
        this.audioWork.play()
      }
    })
  }
}
</script>

<style>

</style>
