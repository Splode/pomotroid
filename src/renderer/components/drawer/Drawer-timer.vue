<template>
  <div class="Container">
    <p class="Drawer-heading">Timer</p>
    <div class="Setting-wrapper">
      <p class="Setting-title">Focus</p>
      <p class="Setting-value">{{ localTimeWork + ':00' }}</p>
      <div class="Slider-wrapper">
        <input
          type="range"
          min="1"
          :max="maxTime"
          step="1"
          class="Slider Slider--red"
          v-model.number="localTimeWork"
          @change="setTimeWork($event, 'work')"
        />
        <div
          class="Slider-bar Slider-bar--red"
          :style="{ width: calcPercentage(localTimeWork, maxTime) + '%' }"
        ></div>
      </div>
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Short Break</p>
      <p class="Setting-value">{{ localTimeShortBreak + ':00' }}</p>
      <div class="Slider-wrapper">
        <input
          type="range"
          min="1"
          :max="maxTime"
          step="1"
          class="Slider Slider--green"
          v-model.number="localTimeShortBreak"
          @change="setTimeShortBreak($event, 'short-break')"
        />
        <div
          class="Slider-bar Slider-bar--green"
          :style="{ width: calcPercentage(localTimeShortBreak, maxTime) + '%' }"
        ></div>
      </div>
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Long Break</p>
      <p class="Setting-value">{{ localTimeLongBreak + ':00' }}</p>
      <div class="Slider-wrapper">
        <input
          type="range"
          min="1"
          :max="maxTime"
          step="1"
          class="Slider Slider--blue"
          v-model.number="localTimeLongBreak"
          @change="setTimeLongBreak($event, 'long-break')"
        />
        <div
          class="Slider-bar Slider-bar--blue"
          :style="{ width: calcPercentage(localTimeLongBreak, maxTime) + '%' }"
        ></div>
      </div>
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Rounds</p>
      <p class="Setting-value">{{ localWorkRounds }}</p>
      <div class="Slider-wrapper">
        <input
          type="range"
          min="1"
          :max="maxRounds"
          step="1"
          class="Slider"
          v-model.number="localWorkRounds"
          @change="setWorkRounds"
        />
        <div
          class="Slider-bar Slider-bar--blueGrey"
          :style="{
            width: calcRoundPercentage(localWorkRounds, maxRounds) + '%'
          }"
        ></div>
      </div>
    </div>

    <div class="Setting-wrapper">
      <p class="TextButton" @click="resetDefaults">Reset Defaults</p>
    </div>
  </div>
</template>

<script>
import { EventBus } from '@/utils/EventBus'

export default {
  name: 'Drawer-timer',

  data() {
    return {
      localTimeLongBreak: 0,
      localTimeShortBreak: 0,
      localTimeWork: 0,
      localWorkRounds: 0,
      maxTime: 90,
      maxRounds: 12
    }
  },

  computed: {
    // store getters
    currentRound() {
      return this.$store.getters.currentRound
    },

    timeLongBreak() {
      return this.$store.getters.timeLongBreak
    },

    timeShortBreak() {
      return this.$store.getters.timeShortBreak
    },

    timeWork() {
      return this.$store.getters.timeWork
    },

    workRounds() {
      return this.$store.getters.workRounds
    }
  },

  methods: {
    calcPercentage(val, max) {
      return (val / max) * 100
    },

    // complex conditional to correctly position slider-bar for round slider
    calcRoundPercentage(val, max) {
      const percentage = (val / max) * 100
      if (percentage > 25 && percentage < 34) {
        return percentage - 6
      } else if (percentage > 33 && percentage < 66) {
        return percentage - 5.5
      } else if (percentage > 50) {
        return percentage - 4
      } else {
        return percentage - 7
      }
    },

    checkToResetTimer(setting) {
      if (this.currentRound === setting) {
        EventBus.$emit('timer-init', {
          auto: false
        })
        EventBus.$emit('call-timer-reset')
      }
    },

    initTimes() {
      this.localTimeLongBreak = this.timeLongBreak
      this.localTimeShortBreak = this.timeShortBreak
      this.localTimeWork = this.timeWork
      this.localWorkRounds = this.workRounds
    },

    resetDefaults() {
      this.$store.dispatch('resetDefaults')
      this.initTimes()
      EventBus.$emit('timer-init', {
        auto: false
      })
      EventBus.$emit('call-timer-reset')
    },

    setTimeLongBreak(e, setting) {
      this.$store.dispatch('setTimeLongBreak', parseInt(e.target.value))
      this.checkToResetTimer(setting)
    },

    setTimeShortBreak(e, setting) {
      this.$store.dispatch('setTimeShortBreak', parseInt(e.target.value))
      this.checkToResetTimer(setting)
    },

    setTimeWork(e, setting) {
      this.$store.dispatch('setTimeWork', parseInt(e.target.value))
      this.checkToResetTimer(setting)
    },

    setWorkRounds(e, setting) {
      this.$store.dispatch('setWorkRounds', parseInt(e.target.value))
    }
  },

  mounted() {
    this.initTimes()
  }
}
</script>

<style lang="scss" scoped>
.Setting-wrapper {
  margin: 10px 0;
  text-align: center;
}

.Setting-title {
  color: var(--color-foreground-darkest);
  font-size: 14px;
  letter-spacing: 0.05em;
  margin-bottom: 8px;
}

.Setting-value {
  background-color: var(--color-background);
  border-radius: 4px;
  display: inline-block;
  font-family: 'RobotoMono', monospace;
  font-size: 12px;
  padding: 2px 6px;
}

.TextButton {
  color: var(--color-foreground-darker);
}
</style>
