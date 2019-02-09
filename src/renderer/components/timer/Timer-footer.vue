<template>
  <section class="Container Footer">
    <div class="Round-wrapper">
      <p>{{ round + '/' + workRounds }}</p>
      <p
        class="TextButton"
        @click="callForReset"
      >Reset</p>
    </div>
    <div
      class="Icon-group"
      style="position: absolute; right: 0;"
    >
      <!-- skip -->
      <div
        class="Icon-wrapper Icon-wrapper--double--left"
        @click="skipRound"
      >
        <svg
          version="1.2"
          baseProfile="tiny"
          id="Layer_1"
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          x="0px"
          y="0px"
          viewBox="0 0 8 12"
          xml:space="preserve"
          height="15px"
          class="Icon--skip"
        >
          <polygon
            fill="#858C99"
            points="0,0 0,12 6.1,5.9"
          />
          <rect
            x="6.9"
            y="0"
            fill="#858C99"
            width="1.1"
            height="12"
          />
        </svg>
      </div>
      <!-- mute -->
      <div
        class="Icon-wrapper Icon-wrapper--double--right"
        @click="toggleMute"
        @mouseenter="volumeSliderHidden = false"
      >
        <transition
          name="fade"
          mode="out-in"
        >
          <svg
            version="1.2"
            baseProfile="tiny"
            id="Layer_1"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            x="0px"
            y="0px"
            viewBox="0 0 12.3 12"
            xml:space="preserve"
            height="15px"
            class="Icon--mute"
            v-if="localVolume > 0"
          >
            <path
              fill="#858C99"
              d="M0,3.9v4.1h2.7l3.4,3.4V0.5L2.7,3.9H0z M9.2,6c0-1.2-0.7-2.3-1.7-2.8v5.5C8.5,8.3,9.2,7.2,9.2,6z M7.5,0v1.4
      c2,0.6,3.4,2.4,3.4,4.6s-1.4,4-3.4,4.6V12c2.7-0.6,4.8-3.1,4.8-6S10.3,0.6,7.5,0z"
            />
          </svg>
          <svg
            version="1.1"
            id="Layer_1"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            x="0px"
            y="0px"
            viewBox="-467 269 24 24"
            style="enable-background:new -467 269 24 24;"
            xml:space="preserve"
            height="20px"
            class="Icon--muted"
            v-else
          >
            <path
              fill="#858C99"
              d="M-450.5,281c0-1.8-1-3.3-2.5-4v2.2l2.5,2.5C-450.5,281.4-450.5,281.2-450.5,281z M-448,281c0,0.9-0.2,1.8-0.5,2.6l1.5,1.5
              c0.7-1.2,1-2.6,1-4.1c0-4.3-3-7.9-7-8.8v2.1C-450.1,275.1-448,277.8-448,281z M-462.7,272l-1.3,1.3l4.7,4.7h-4.7v6h4l5,5v-6.7
              l4.3,4.3c-0.7,0.5-1.4,0.9-2.3,1.2v2.1c1.4-0.3,2.6-1,3.7-1.8l2,2l1.3-1.3l-9-9L-462.7,272z M-455,273l-2.1,2.1l2.1,2.1V273z"
            />
            <path
              fill="none"
              d="M-467,269h24v24h-24V269z"
            />
          </svg>
        </transition>
      </div>
      <!-- volume slider -->
      <transition name="fade">
        <div
          class="Slider-wrapper Slider-wrapper--vert"
          v-show="!volumeSliderHidden"
        >
          <input
            type="range"
            min="0"
            max="100"
            class="Slider"
            v-model="localVolume"
            @change="setVolume"
          >
          <div class="Slider-bar Slider-bar--blueGrey"></div>
        </div>
      </transition>

    </div>
  </section>
</template>

<script>
import { EventBus } from '@/utils/event-bus'

export default {
  name: 'TimerFooter',
  data() {
    return {
      currentMousePosition: {
        x: null,
        y: null
      },
      localVolume: 0,
      volumeSliderHidden: true
    }
  },
  computed: {
    // store getters
    currentRound() {
      return this.$store.getters.currentRound
    },

    round() {
      return this.$store.getters.round
    },

    workRounds() {
      return this.$store.getters.workRounds
    },

    volume() {
      return this.$store.getters.volume
    }
  },

  methods: {
    callForReset() {
      EventBus.$emit('call-timer-reset')
    },

    /**
     * Hides the volume slider unless the last recorded mouse position
     * falls within a range containing the volume slider.
     */
    volumeSliderTimeout() {
      setInterval(() => {
        if (
          this.currentMousePosition.x >= 305 &&
          this.currentMousePosition.x <= 355 &&
          (this.currentMousePosition.y >= 305 &&
            this.currentMousePosition.y <= 455)
        ) {
        } else {
          this.volumeSliderHidden = true
        }
      }, 6000)
    },

    skipRound() {
      EventBus.$emit('timer-completed')
    },

    toggleMute() {
      // set volume to zero if not muted
      // otherwise set to 100 (default)
      if (this.localVolume === '0') {
        this.localVolume = '100'
        this.$store.dispatch('setVolume', 100)
      } else {
        this.localVolume = '0'
        this.$store.dispatch('setVolume', 0)
      }
    },

    setVolume(e) {
      this.$store.dispatch('setVolume', parseInt(e.target.value))
    }
  },

  mounted() {
    this.localVolume = this.volume
    this.volumeSliderTimeout()

    // record last mouse position for volume slider timeout
    window.addEventListener('mousemove', e => {
      this.currentMousePosition.x = e.clientX
      this.currentMousePosition.y = e.clientY
    })
  }
}
</script>

<style lang="scss" scoped>
.Footer {
  align-items: center;
  display: flex;
  justify-content: space-between;
}

.Icon--mute {
  & path {
    transition: $transitionDefault;
  }
}

.Icon--muted {
  & path {
    transition: $transitionDefault;
  }
}

.Icon-wrapper {
  &:hover .Icon--muted path:first-child {
    fill: $colorRed;
  }
  &:hover .Icon--mute path {
    fill: $colorRed;
  }
  &:hover .Icon--skip polygon,
  &:hover .Icon--skip rect {
    fill: $colorRed;
  }
}

.Icon--skip {
  & polygon,
  & rect {
    transition: $transitionDefault;
  }
}

.Round-wrapper {
  text-align: center;
}

.Slider-wrapper {
  padding: 8px;
  position: absolute;
  top: -61px;
  right: -29px;
}

.Slider {
  &::-webkit-slider-runnable-track {
    background-color: $colorBlueGrey;
  }
  &::-webkit-slider-thumb {
    margin-top: -7px;
    transition: $transitionDefault;
    &:hover {
      background-color: $colorRed;
      border-color: $colorRed;
    }
  }
}
</style>
