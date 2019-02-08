<template>
  <div>
    <audio
      id="audio-long-break"
      ref="audio-long-break"
      :src="'./static/audio/alert-long-break.mp3'"
    ></audio>
    <audio
      id="audio-short-break"
      ref="audio-short-break"
      :src="'./static/audio/alert-short-break.mp3'"
    ></audio>
    <audio
      id="audio-work"
      ref="audio-work"
      :src="'./static/audio/alert-work.mp3'"
    ></audio>
  </div>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  data() {
    return {
      audioLongBreak: null,
      audioShortBreak: null,
      audioWork: null
    }
  },

  computed: {
    // store getters
    volume() {
      return this.$store.getters.volume * 0.01
    }
  },

  mounted() {
    // Volume attribute on audio is not supported
    // and must be set programmatically.
    EventBus.$on('ready-long-break', () => {
      this.$refs['audio-long-break'].volume = this.volume
      this.$refs['audio-long-break'].play()
    })

    EventBus.$on('ready-short-break', () => {
      this.$refs['audio-short-break'].volume = this.volume
      this.$refs['audio-short-break'].play()
    })

    EventBus.$on('ready-work', () => {
      this.$refs['audio-work'].volume = this.volume
      this.$refs['audio-work'].play()
    })
  }
}
</script>
