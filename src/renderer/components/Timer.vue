<template>
  <div>
    <button @click="startTimer">Start</button>
    <button @click="pauseTimer">Pause</button>
    <button @click="resumeTimer">Resume</button>
    <button @click="resetTimer">Reset</button>
    <p v-if="!timerStarted">{{ prettyMinutes }}</p>
    <p v-else>{{ prettyTime }}</p>
    <app-timer-controller/>
  </div>
</template>

<script>
import Timer from './../utils/timer'
import appTimerController from '@/components/Timer-controller'
import { EventBus } from '../utils/event-bus'

export default {
  components: {
    appTimerController
  },

  data () {
    return {
      minutes: 1,
      timer: null,
      timerActive: false,
      timerStarted: false
    }
  },

  computed: {
    // store getters
    currentRound () {
      return this.$store.getters.currentRound
    },

    timeLongBreak () {
      return this.$store.getters.timeLongBreak
    },

    timeShortBreak () {
      return this.$store.getters.timeShortBreak
    },

    timeWork () {
      return this.$store.getters.timeWork
    },

    // local
    prettyMinutes () {
      return this.minutes + ':00'
    },

    prettyTime () {
      return `${this.timeRemaining.remainingMinutes}:${this.timeRemaining.remainingSeconds}`
    },

    timeElapsed () {
      if (this.timer.time !== null) {
        const time = this.timer.time
        const minutes = Math.floor(time / 60)
        const seconds = time - (minutes * 60)
        return {
          minutes,
          seconds
        }
      }
    },

    timeRemaining () {
      if (this.timer.time !== null) {
        const minutes = this.minutes
        const time = this.timer.time
        const elapsedMinutes = Math.floor(time / 60)
        const elapsedSeconds = time - (elapsedMinutes * 60)
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

    createTimer (min) {
      this.timer = new Timer(min)
    },

    initTimer () {
      switch (this.currentRound) {
        case 'work':
          this.minutes = this.timeWork
          this.createTimer(this.timeWork)
          break
        case 'short-break':
          this.minutes = this.timeShortBreak
          this.createTimer(this.timeShortBreak)
          break
        case 'long-break':
          this.minutes = this.timeLongBreak
          this.createTimer(this.timeShortBreak)
          break
        default:
          this.createTimer(25)
          break
      }
    },

    pauseTimer () {
      this.timer.pause()
      this.timerActive = !this.timerActive
    },

    resetTimer () {
      this.timer.reset()
      this.timerActive = !this.timerActive
      this.timerStarted = false
    },

    resumeTimer () {
      this.timer.resume()
      this.timerActive = !this.timerActive
    },

    startTimer () {
      this.timer.start()
      this.timerActive = !this.timerActive
      this.timerStarted = true
    }
  },

  mounted () {
    this.initTimer()
    EventBus.$on('timer-init', () => {
      this.initTimer()
      setTimeout(() => {
        this.startTimer()
      }, 1500)
    })
  }
}
</script>

<style>

</style>
