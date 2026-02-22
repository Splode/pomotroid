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
    <audio
      id="audio-tick"
      ref="audio-tick"
      :src="'./static/audio/tick.mp3'"
    ></audio>
  </div>
</template>

<script>
import { EventBus } from '@/utils/EventBus'

export default {
  data() {
    return {
      audioLongBreak: null,
      audioTick: null,
      audioShortBreak: null,
      audioWork: null
    }
  },

  computed: {
    // store getters
    currentRound() {
      return this.$store.getters.currentRound
    },
    tickSounds() {
      return this.$store.getters.tickSounds
    },
    tickSoundsDuringBreak() {
      return this.$store.getters.tickSoundsDuringBreak
    },
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

    EventBus.$on('timer-tick', () => {
      this.$refs['audio-tick'].volume = this.volume
      const isBreak = this.currentRound === 'short-break' || this.currentRound === 'long-break'
      if (isBreak && !this.tickSoundsDuringBreak) return
      if (!isBreak && !this.tickSounds) return
      this.$refs['audio-tick'].play()
    })

    EventBus.$on('ready-work', () => {
      this.$refs['audio-work'].volume = this.volume
      this.$refs['audio-work'].play()
    })
  }
}
</script>
