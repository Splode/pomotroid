<template>
  <nav class="Titlebar">
    <!-- menu -->
    <div class="Titlebar-icon-wrapper Titlebar-icon-wrapper--lg" style="position: absolute;" @click="toggleDrawer">
      <div class="Icon Menu-wrapper" :class="drawerOpen ? 'is-collapsed' : ''">
        <div class="Menu-line"></div>
        <div class="Menu-line"></div>
      </div>
    </div>

    <h1 class="Title">Pomotroid</h1>

    <div style="display: flex; position: absolute; top: 0; right: 0;">
      <div class="Titlebar-icon-wrapper Titlebar-icon-wrapper--sm" @click="winMinimize">
        <!-- minimize -->
        <div class="Icon-wrapper">
          <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
        x="0px" y="0px" viewBox="0 0 14 2" xml:space="preserve" width="15px" height="20px" class="Icon Icon--minimize">
            <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="13" y2="1"/>
          </svg>
        </div>
      </div>
      <div class="Titlebar-icon-wrapper Titlebar-icon-wrapper--sm" @click="winClose">
        <!-- close -->
        <div class="Icon-wrapper">
          <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
        x="0px" y="0px" viewBox="0 0 12.6 12.6" xml:space="preserve" height="15px" class="Icon Icon--close">
            <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="11.6" y2="11.6"/>
            <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="11.6" y1="1" x2="1" y2="11.6"/>
          </svg>
        </div>
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
  padding-top: 18px;
}

.Titlebar {
  letter-spacing: .05em;
  margin-bottom: 18px;
  position: relative;
  text-align: center;
  height: 50px;
  -webkit-app-region: drag;
}

.Titlebar-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 20px;
  height: 20px;
  -webkit-app-region: no-drag;
  &:hover .Menu-line {
    background-color: $colorRed;
  }
  &:hover .Icon--close line, &:hover .Icon--minimize line {
    stroke: $colorRed;
  }
}

.Titlebar-icon-wrapper--sm {
  padding: 18px 9px;
}

.Titlebar-icon-wrapper--lg {
  padding: 18px;
}
</style>
