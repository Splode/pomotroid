<template>
  <div class="Container">
    <div class="Setting-wrapper">
      <p class="Setting-title">Work</p>
      <p class="Setting-value">{{ localTimeWork + ':00' }}</p>
      <input 
        type="range" 
        min="1" 
        max="60" 
        step="1" 
        class="Slider Slider--red" 
        v-model.number="localTimeWork" 
        @change="setTimeWork"
      >
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Short Break</p>
      <p class="Setting-value">{{ localTimeShortBreak + ':00' }}</p>
      <input 
        type="range" 
        min="1" 
        max="60" 
        step="1" 
        class="Slider Slider--green" 
        v-model.number="localTimeShortBreak"
        @change="setTimeShortBreak"
      >
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Long Break</p>
      <p class="Setting-value">{{ localTimeLongBreak + ':00' }}</p>
      <input 
        type="range" 
        min="1" 
        max="60" 
        step="1" 
        class="Slider Slider--blue" 
        v-model.number="localTimeLongBreak"
        @change="setTimeLongBreak"
      >
    </div>

    <div class="Setting-wrapper">
      <p class="Setting-title">Rounds</p>
      <p class="Setting-value">{{ localWorkRounds }}</p>
      <input 
        type="range" 
        min="1" 
        max="12" 
        step="1" 
        class="Slider" 
        v-model.number="localWorkRounds"
        @change="setWorkRounds"
      >
    </div>

    <div class="Setting-wrapper">
      <p class="TextButton">Reset Defaults</p>
    </div>
  </div>
</template>

<script>
export default {
  data () {
    return {
      localTimeLongBreak: 0,
      localTimeShortBreak: 0,
      localTimeWork: 0,
      localWorkRounds: 0
    }
  },

  computed: {
    // store getters
    timeLongBreak () {
      return this.$store.getters.timeLongBreak
    },

    timeShortBreak () {
      return this.$store.getters.timeShortBreak
    },

    timeWork () {
      return this.$store.getters.timeWork
    },

    workRounds () {
      return this.$store.getters.workRounds
    }
  },

  methods: {
    setTimeLongBreak (e) {
      this.$store.dispatch('setTimeLongBreak', e.target.value)
    },

    setTimeShortBreak (e) {
      this.$store.dispatch('setTimeShortBreak', e.target.value)
    },

    setTimeWork (e) {
      this.$store.dispatch('setTimeWork', e.target.value)
    },

    setWorkRounds (e) {
      this.$store.dispatch('setWorkRounds', e.target.value)
    }
  },

  mounted () {
    this.localTimeLongBreak = this.timeLongBreak
    this.localTimeShortBreak = this.timeShortBreak
    this.localTimeWork = this.timeWork
    this.localWorkRounds = this.workRounds
  }
}
</script>

<style lang="scss" scoped>
.Setting-wrapper {
  margin: 16px 0;
  text-align: center;
}

.Setting-title {
  color: $colorBlueGrey;
  font-size: 14px;
  letter-spacing: .05em;
  margin-bottom: 8px;
}

.Setting-value {
  background-color: $colorNavy;
  border-radius: 4px;
  display: inline-block;
  font-family: 'RobotoMono', monospace;
  font-size: 12px;
  padding: 2px 6px;
}

.Slider {
  position: relative;
  width: 100%;
  -webkit-appearance: none;
  &:focus {
    outline: 0;
  }
  &::-webkit-slider-runnable-track {
    background-color: $colorNavy;
    width: 100%;
    height: 3px;
  }
  &::-webkit-slider-thumb {
    background-color: $colorBlueGrey;
    border: 2px solid $colorBlueGrey;
    border-radius: 100%;
    margin-top: -7px;
    width: 15px;
    height: 15px;
    -webkit-appearance: none;
    -webkit-app-region: no-drag;
  }
}

.Slider--blue {
  &::-webkit-slider-thumb {
    background-color: $colorBlue;
    border: 2px solid $colorBlue;
    -webkit-app-region: no-drag;
  }
}

.Slider--green {
  &::-webkit-slider-thumb {
    background-color: $colorGreen;
    border: 2px solid $colorGreen;
    -webkit-app-region: no-drag;
  }
}

.Slider--red {
  &::-webkit-slider-thumb {
    background-color: $colorRed;
    border: 2px solid $colorRed;
    -webkit-app-region: no-drag;
  }
}
</style>
