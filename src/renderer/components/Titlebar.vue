<template>
  <nav class="Container Titlebar">
    <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
	 x="0px" y="0px" viewBox="0 0 22 9.7" xml:space="preserve" width="20px" class="Icon">
      <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="21" y2="1"/>
      <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="8.7" x2="13" y2="8.7"/>
    </svg>
    <h1 class="Title">Pomotroid</h1>
    <svg version="1.2" baseProfile="tiny" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
	 x="0px" y="0px" viewBox="0 0 12.6 12.6" xml:space="preserve" height="15px" class="Icon Icon--close" @click="winClose">
      <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="1" y1="1" x2="11.6" y2="11.6"/>
      <line fill="none" stroke="#F6F2EB" stroke-width="2" stroke-linecap="round" stroke-miterlimit="10" x1="11.6" y1="1" x2="1" y2="11.6"/>
    </svg>
  </nav>
</template>

<script>
import { ipcRenderer } from 'electron'

export default {
  methods: {
    winClose () {
      ipcRenderer.send('window-close')
    }
  }
}
</script>

<style lang="scss" scoped>
.Icon--close {
  & line {
    transition: $transitionDefault;
  }
  &:hover line {
    stroke: $colorRed;
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
</style>
