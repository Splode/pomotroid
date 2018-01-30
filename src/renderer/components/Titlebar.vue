<template>
  <nav class="Container Titlebar">
    <div class="Titlebar-icon-wrapper">
      <!-- menu -->
      <div class="Icon Menu-wrapper" :class="drawerOpen ? 'is-collapsed' : ''" @click="toggleDrawer">
        <div class="Menu-line"></div>
        <div class="Menu-line"></div>
      </div>
    </div>
    <h1 class="Title">Pomotroid</h1>
    <div class="Titlebar-icon-wrapper">
      <!-- minimize -->
      <div class="Icon-wrapper" @click="winMinimize">
        <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
      x="0px" y="0px" viewBox="0 0 14 2" xml:space="preserve" width="15px" height="20px" class="Icon Icon--minimize">
          <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="13" y2="1"/>
        </svg>
      </div>
      <!-- close -->
      <div class="Icon-wrapper" @click="winClose">
        <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
      x="0px" y="0px" viewBox="0 0 12.6 12.6" xml:space="preserve" height="15px" class="Icon Icon--close">
          <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="11.6" y2="11.6"/>
          <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="11.6" y1="1" x2="1" y2="11.6"/>
        </svg>
      </div>
    </div>
  </nav>
</template>

<script>
import { ipcRenderer } from 'electron'

export default {
  computed: {
    // store getters
    drawerOpen () {
      return this.$store.getters.drawerOpen
    }
  },

  methods: {
    toggleDrawer () {
      this.$store.dispatch('toggleDrawer')
    },

    winClose () {
      ipcRenderer.send('window-close')
    },

    winMinimize () {
      ipcRenderer.send('window-minimize')
    }
  }
}
</script>

<style lang="scss" scoped>
.Icon-wrapper {
  &:hover .Icon--close line, &:hover .Icon--minimize line {
    stroke: $colorRed;
  }
}

.Icon--close, .Icon--minimize {
  & line {
    stroke: $colorBlueGrey;
    transition: $transitionDefault;
  }
}

.Menu-line {
  background-color: $colorBlueGrey;
  display: inline-block;
  transition: $transitionDefault;
  width: 20px;
  height: 2px;
  &:last-child {
    width: 10px;
  }
}

.Menu-wrapper {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  height: 80%;
  &:hover .Menu-line {
    background-color: $colorRed;
  }
  &.is-collapsed {
    & .Menu-line:first-child {
      transform: rotate(-45deg);
      width: 12px;
    }
    & .Menu-line:last-child {
      transform: rotate(45deg);
      width: 12px;
    }
  }
}

.Title {
  color: $colorGreen;
  font-size: 1rem;
  font-weight: 200;
}

.Titlebar {
  align-items: flex-start;
  display: flex;
  justify-content: space-between;
  letter-spacing: .05em;
  padding-top: 18px;
  height: 50px;
  -webkit-app-region: drag;
}

.Titlebar-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 45px;
  height: 20px;
}
</style>
