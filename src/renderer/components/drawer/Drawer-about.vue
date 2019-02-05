<template>
  <div class="Container">
    <p class="Drawer-heading">About</p>
    <section>
      <svg
        version="1.2"
        baseProfile="tiny"
        id="pomotroid_icon"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        x="0px"
        y="0px"
        width="72px"
        height="72px"
        viewBox="0 0 256 256"
        xml:space="preserve"
      >
        <g>
          <circle
            fill="#2F384B"
            cx="128"
            cy="128"
            r="126.81"
          />

          <circle
            fill="none"
            stroke="#FF4E4D"
            stroke-width="40"
            stroke-linecap="round"
            stroke-miterlimit="10"
            cx="128"
            cy="128"
            r="73.31"
          />
        </g>
      </svg>
      <h2>Pomotroid</h2>
      <p class="label">Version: {{ version }} <span
          class="link"
          @click="openUrl(releaseUrl)"
        >(release notes)</span></p>
      <p
        class="label link"
        @click="openUrl('https://github.com/splode/pomotroid')"
      >License and Documentation</p>
    </section>
  </div>
</template>

<script>
import * as pckg from './../../../../package.json'
import { shell } from 'electron'

export default {
  name: 'DrawerAbout',
  data() {
    return {
      license: pckg.license,
      version: pckg.version
    }
  },
  computed: {
    releaseUrl() {
      return `https://github.com/Splode/pomotroid/releases/tag/v${this.version}`
    }
  },
  methods: {
    openUrl(url) {
      shell.openExternal(url)
    }
  }
}
</script>

<style lang="scss" scoped>
h2 {
  color: $colorGreen;
  font-weight: 400;
  letter-spacing: 0.05em;
  margin: 0.5em 0;
}

section {
  align-items: center;
  display: flex;
  flex-direction: column;
  padding-top: 2em;
  height: 100%;
}

.label {
  font-size: 14px;
  letter-spacing: 0.05em;
  line-height: 2;
  & .link,
  &.link {
    cursor: pointer;
    transition: $transitionDefault;
    &:hover {
      color: $colorGreen;
    }
  }
}
</style>
