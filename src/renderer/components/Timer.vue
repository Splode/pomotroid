<template>
  <div>
    <button @click="startTimer">Start</button>
    <button @click="pauseTimer">Pause</button>
    <button @click="resumeTimer">Resume</button>
    <button @click="resetTimer">Reset</button>
    <p v-if="!timerStarted">{{ prettyMinutes }}</p>
    <p v-else>{{ prettyTime }}</p>
  </div>
</template>

<script>
import Timer from './../utils/timer'

export default {
  data () {
    return {
      minutes: 1,
      workTimer: null,
      timerActive: false,
      timerStarted: false
    }
  },

  computed: {
    prettyMinutes () {
      return this.minutes + ':00'
    },

    prettyTime () {
      return `${this.timeRemaining.remainingMinutes}:${this.timeRemaining.remainingSeconds}`
    },

    timeElapsed () {
      if (this.workTimer.time !== null) {
        const time = this.workTimer.time
        const minutes = Math.floor(time / 60)
        const seconds = time - (minutes * 60)
        return {
          minutes,
          seconds
        }
      }
    },

    timeRemaining () {
      if (this.workTimer.time !== null) {
        const minutes = this.minutes
        const time = this.workTimer.time
        const elapsedMinutes = Math.floor(time / 60)
        const elapsedSeconds = time - (elapsedMinutes * 60)
        // const remainingMinutes = (minutes - elapsedMinutes) - 1
        // const remainingSeconds = 60 - elapsedSeconds
        const remainingSeconds = this.formatTimeDouble(60 - elapsedSeconds)
        let remainingMinutes = minutes - elapsedMinutes

        if (elapsedSeconds > 0) {
          remainingMinutes -= 1
        }

        return {
          remainingMinutes,
          remainingSeconds
        }
      }
    },

    seconds () {
      if (this.workTimer !== null) {
        if (this.workTimer.time < 60) {
          return 60 - this.workTimer.time
        } else {
          return this.workTimer.time / 60
        }
      }
    },

    time () {
      if (this.workTimer !== null) {
        return this.workTimer.time
      }
    }
  },

  methods: {
    formatTimeDouble (time) {
      if (time === 60) {
        return '00'
      } else if (time < 10) {
        return `0${time}`
      } else {
        return time
      }
    },

    pauseTimer () {
      this.workTimer.pause()
      this.timerActive = !this.timerActive
    },

    resetTimer () {
      this.workTimer.reset()
      this.timerActive = !this.timerActive
      this.timerStarted = false
    },

    resumeTimer () {
      this.workTimer.resume()
      this.timerActive = !this.timerActive
    },

    startTimer () {
      this.workTimer.start()
      this.timerActive = !this.timerActive
      this.timerStarted = true
    }
  },

  mounted () {
    this.workTimer = new Timer(this.minutes)
  }
}
</script>

<style>

</style>
