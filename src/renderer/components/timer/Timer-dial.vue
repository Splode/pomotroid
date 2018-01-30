<template>
  <div class="Dial-wrapper">
    <slot></slot>
    <p class="Dial-label">{{ currentRoundDisplay }}</p>
    <svg 
      version="1.2" 
      baseProfile="tiny" 
      id="Layer_1" 
      xmlns="http://www.w3.org/2000/svg" 
      xmlns:xlink="http://www.w3.org/1999/xlink"
      x="0px" y="0px" 
      viewBox="0 0 230 230" 
      xml:space="preserve"
      width="220"
      height="220"
      class="Dial-fill"
      :class="dialClass"
    >
      <path 
        fill="none"
        stroke-width="10" 
        stroke-linecap="round" 
        stroke-miterlimit="10"
        d="M115,5c60.8,0,110,49.2,110,110s-49.2,110-110,110S5,175.8,5,115S54.2,5,115,5"
      />
    </svg>
    <svg 
      version="1.2" 
      baseProfile="tiny" 
      id="Layer_1" 
      xmlns="http://www.w3.org/2000/svg" 
      xmlns:xlink="http://www.w3.org/1999/xlink"
      x="0px" y="0px" 
      viewBox="0 0 230 230" 
      xml:space="preserve"
      width="220"
      height="220"
      class="Dial-bg"
    >
      <path 
        fill="none" 
        stroke-width="2" 
        stroke-linecap="round" 
        stroke-miterlimit="10" 
        d="M115,5c60.8,0,110,49.2,110,110s-49.2,110-110,110S5,175.8,5,115S54.2,5,115,5"
      />
    </svg>
  </div>
</template>

<script>
import anime from 'animejs'
import { EventBus } from '@/utils/event-bus'

export default {
  props: ['minutes'],

  data () {
    return {
      dial: null
    }
  },

  computed: {
    // store getters
    currentRound () {
      return this.$store.getters.currentRound
    },

    timeLongBreak () {
      return this.$store.getters.timeLongBreak * 60 * 1000
    },

    timeShortBreak () {
      return this.$store.getters.timeShortBreak * 60 * 1000
    },

    timeWork () {
      return this.$store.getters.timeWork * 60 * 1000
    },

    currentRoundDisplay () {
      if (this.currentRound === 'work') {
        return 'Work'
      } else if (this.currentRound === 'short-break') {
        return 'Short Break'
      } else if (this.currentRound === 'long-break') {
        return 'Long Break'
      }
    },

    dialClass () {
      if (this.currentRound === 'work') {
        return 'Dial-fill--work'
      } else if (this.currentRound === 'short-break') {
        return 'Dial-fill--shortBreak'
      } else if (this.currentRound === 'long-break') {
        return 'Dial-fill--longBreak'
      }
    }
  },

  methods: {
    dialAnimation (duration) {
      this.dial = anime({
        targets: '.Dial-fill path',
        strokeDashoffset: [anime.setDashoffset, 0],
        easing: 'linear',
        duration: duration,
        direction: 'reverse',
        autoplay: false
      })
      this.dial.seek(this.dial.duration)
    }
  },

  mounted () {
    this.dialAnimation(this.timeWork)
    EventBus.$on('timer-started', () => {
      this.dial.play()
    })
    EventBus.$on('timer-paused', () => {
      this.dial.pause()
    })
    EventBus.$on('timer-resumed', () => {
      this.dial.play()
    })
    EventBus.$on('timer-reset', () => {
      this.dial.pause()
      this.dial.seek(this.dial.duration)
    })
    EventBus.$on('timer-init', () => {
      this.dial.pause()
      if (this.currentRound === 'work') {
        this.dialAnimation(this.timeWork)
      } else if (this.currentRound === 'short-break') {
        this.dialAnimation(this.timeShortBreak)
      } else if (this.currentRound === 'long-break') {
        this.dialAnimation(this.timeLongBreak)
      }
    })
  }
}
</script>

<style lang="scss" scoped>
.Dial-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 35px;
  position: relative;
}

.Dial-label {
  letter-spacing: .1em;
  position: absolute;
  top: 66%;
  text-transform: uppercase;
}

.Dial-bg {
  stroke: $colorBlueGrey;
}

.Dial-fill {
  position: absolute;
  transform-origin: center;
  -webkit-app-region: no-drag;
}

.Dial-fill--work {
  stroke: $colorRed;
}

.Dial-fill--shortBreak {
  stroke: $colorGreen;
}

.Dial-fill--longBreak {
  stroke: $colorBlue;
}
</style>
