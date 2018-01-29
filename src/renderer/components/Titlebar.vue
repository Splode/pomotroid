<template>
  <nav class="Container Titlebar">
    <div class="Titlebar-icon-wrapper">
      <!-- menu -->
      <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
    x="0px" y="0px" viewBox="0 0 22 9.7" xml:space="preserve" width="20px" class="Icon Icon--menu" @click="toggleDrawer">
        <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="21" y2="1"/>
        <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="8.7" x2="13" y2="8.7" class="Icon--menu-secondLine"/>
      </svg>
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

.Icon--menu {
  & .Icon--menu-secondLine {
    transition: $transitionDefault;
  }
  &:hover .Icon--menu-secondLine {
    width: 100%;
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
}
</style>
