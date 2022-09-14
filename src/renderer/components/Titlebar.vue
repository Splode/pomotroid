<template>
  <nav class="Titlebar">
    <!-- menu -->
    <div
      class="Icon-wrapper Icon-wrapper--titlebar Icon-wrapper--single"
      style="position: absolute;"
      title="Settings"
      @click="toggleDrawer"
    >
      <div class="Menu-wrapper" :class="drawerOpen ? 'is-collapsed' : ''">
        <div class="Menu-line"></div>
        <div class="Menu-line"></div>
      </div>
    </div>

    <h1 class="Title">Pomotroid</h1>

    <div class="Icon-group" style="position: absolute; top: 0; right: 0;">
      <div
        class="Icon-wrapper Icon-wrapper--titlebar Icon-wrapper--double--left"
        style="padding-left: 5vw"
        @click="winMinimize"
      >
        <!-- minimize -->
        <svg
          version="1.2"
          baseProfile="tiny"
          id="Layer_1"
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          x="0px"
          y="0px"
          viewBox="0 0 14 2"
          xml:space="preserve"
          width="4.2vw"
          height="5.5vw"
          class="Icon Icon--minimize"
        >
          <line
            fill="none"
            stroke="#F6F2EB"
            stroke-width="2"
            stroke-linecap="round"
            stroke-miterlimit="10"
            x1="1"
            y1="1"
            x2="13"
            y2="1"
          />
        </svg>
      </div>
      <div
        class="Icon-wrapper Icon-wrapper--titlebar Icon-wrapper--double--right"
        style="padding-right: 5vw"
        @click="winClose"
      >
        <!-- close -->
        <svg
          version="1.2"
          baseProfile="tiny"
          id="Layer_1"
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          x="0px"
          y="0px"
          viewBox="0 0 12.6 12.6"
          xml:space="preserve"
          height="4.2vw"
          class="Icon Icon--close"
        >
          <line
            fill="none"
            stroke="#F6F2EB"
            stroke-width="2"
            stroke-linecap="round"
            stroke-miterlimit="10"
            x1="1"
            y1="1"
            x2="11.6"
            y2="11.6"
          />
          <line
            fill="none"
            stroke="#F6F2EB"
            stroke-width="2"
            stroke-linecap="round"
            stroke-miterlimit="10"
            x1="11.6"
            y1="1"
            x2="1"
            y2="11.6"
          />
        </svg>
      </div>
    </div>
  </nav>
</template>

<script>
import { ipcRenderer } from 'electron'

export default {
  computed: {
    drawerOpen() {
      return this.$store.getters.drawerOpen
    },

    minToTray() {
      return this.$store.getters.minToTray
    },

    minToTrayOnClose() {
      return this.$store.getters.minToTrayOnClose
    }
  },

  methods: {
    toggleDrawer() {
      this.$store.dispatch('toggleDrawer')
    },

    winClose() {
      this.minToTrayOnClose
        ? this.winMinimize()
        : ipcRenderer.send('window-close')
    },

    winMinimize() {
      ipcRenderer.send('window-minimize', this.minToTray)
    }
  }
}
</script>

<style lang="scss" scoped>
.Icon--close,
.Icon--minimize {
  & line {
    stroke: var(--color-background-lightest);
    transition: $transitionDefault;
  }
}

.Menu-line {
  background-color: var(--color-background-lightest);
  display: inline-block;
  transition: $transitionDefault;
  width: 5.5vw;
  height: 0.5vw;
  &:last-child {
    width: 2.8vw;
  }
}

.Menu-wrapper {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  height: 80%;
  &.is-collapsed {
    & .Menu-line:first-child {
      transform: rotate(-45deg);
      width: 3.3vw;
    }
    & .Menu-line:last-child {
      transform: rotate(45deg);
      width: 3.3vw;
    }
  }
}

.Title {
  color: var(--color-short-round);
  font-size: 1rem;
  font-weight: 200;
  padding-top: 5vw;
}

.Titlebar {
  letter-spacing: 0.05em;
  margin-bottom: 5vw;
  position: relative;
  text-align: center;
  height: 13.9vw;
  -webkit-app-region: drag;
}

.Icon-wrapper--titlebar {
  -webkit-app-region: no-drag;
  &:hover .Menu-line {
    background-color: var(--color-accent);
  }
  &:hover .Icon--close line {
    stroke: var(--color-focus-round);
  }
  &:hover .Icon--minimize line {
    stroke: var(--color-accent);
  }
}
</style>
