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
export default {
  computed: {
    // store getters
    currentRound () {
      return this.$store.getters.currentRound
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
  }
}
</script>

<style lang="scss" scoped>
.Dial-wrapper {
  display: flex;
  justify-content: center;
  position: relative;
}

.Dial-label {
  letter-spacing: .1em;
  position: absolute;
  top: 56%;
  text-transform: uppercase;
}

.Dial-bg {
  stroke: $colorBlueGrey;
}

.Dial-fill {
  position: absolute;
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
