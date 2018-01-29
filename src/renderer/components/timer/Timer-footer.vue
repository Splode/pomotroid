<template>
  <section class="Container Footer">
    <div class="Round-wrapper">
      <p>{{ round + '/' + workRounds }}</p>
      <p class="TextButton">Reset</p>
    </div>
    <div class="Footer-icon-wrapper">
      <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
	 x="0px" y="0px" viewBox="0 0 8 12" xml:space="preserve" height="15px" class="Icon--skip" @click="skipRound">
        <polygon fill="#858C99" points="0,0 0,12 6.1,5.9"/>
        <rect x="6.9" y="0" fill="#858C99" width="1.1" height="12"/>
      </svg>
      <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
	 x="0px" y="0px" viewBox="0 0 12.3 12" xml:space="preserve" height="15px" class="Icon--mute">
        <path fill="#858C99" d="M0,3.9v4.1h2.7l3.4,3.4V0.5L2.7,3.9H0z M9.2,6c0-1.2-0.7-2.3-1.7-2.8v5.5C8.5,8.3,9.2,7.2,9.2,6z M7.5,0v1.4
	c2,0.6,3.4,2.4,3.4,4.6s-1.4,4-3.4,4.6V12c2.7-0.6,4.8-3.1,4.8-6S10.3,0.6,7.5,0z"/>
      </svg>
    </div>
  </section>
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
    skipRound () {
      EventBus.$emit('timer-completed')
    }
  }
}
</script>

<style lang="scss" scoped>
.Footer {
  align-items: center;
  display: flex;
  justify-content: space-between;
  -webkit-app-region: drag;
}

.Footer-icon-wrapper {
  display: flex;
  justify-content: space-between;
  width: 40px;
  -webkit-app-region: no-drag;
}

.Icon--mute {
  & path {
    transition: $transitionDefault;
  }
  &:hover path {
    fill: $colorRed;
  }
}

.Icon--skip {
  & polygon, & rect {
    transition: $transitionDefault;
  }
  &:hover polygon, &:hover rect {
    fill: $colorRed;
  }
}

.Round-wrapper {
  text-align: center;
  -webkit-app-region: no-drag;
}
</style>
