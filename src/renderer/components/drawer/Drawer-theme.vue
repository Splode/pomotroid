<template>
  <div class="Container">
    <p class="Drawer-heading">Themes</p>
    <div
      v-for="(t, i) in themer.themes"
      :key="i"
      class="Setting-wrapper"
      :style="
        `background-color: ${themer.getThemeValue(
          t,
          '--color-background'
        )}; border-color: ${themer.getThemeValue(
          t,
          '--color-accent'
        )}`
      "
      @click="selectTheme(themer.getThemeName(t))"
    >
      <p
        class="Setting-title"
        :style="
          `color: ${themer.getThemeValue(t, '--color-foreground')}`
        "
      >
        {{ themer.getThemeName(t) }}
      </p>
      <transition name="fade">
        <svg
          v-if="theme === themer.getThemeName(t)"
          xmlns="http://www.w3.org/2000/svg"
          height="24"
          viewBox="0 0 24 24"
          width="24"
        >
          <path d="M0 0h24v24H0z" fill="none" />
          <path
            d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"
            fill="var(--color-accent)"
          />
        </svg>
      </transition>
    </div>
  </div>
</template>

<script>
import themer from '@/utils/Themer'
export default {
  name: 'Drawer-theme',

  data() {
    return {
      themer
    }
  },

  computed: {
    theme() {
      return this.$store.getters.theme
    }
  },

  methods: {
    selectTheme(themeName) {
      const payload = { key: 'theme', val: themeName }
      this.$store.dispatch('setSetting', payload)
      this.$store.dispatch('setViewState', payload)
      this.themer.apply(themeName)
    }
  }
}
</script>

<style lang="scss" scoped>
.Container {
  max-height: calc(100% - 36px);
  overflow-y: auto;
}

.Setting-wrapper {
  align-items: center;
  border-left: 3px solid;
  border-radius: 0 4px 4px 0;
  display: flex;
  justify-content: space-between;
  margin: 12px 0;
  min-height: 48px;
  padding: 0 12px;
}

.Setting-title {
  font-size: 14px;
  letter-spacing: 0.05em;
}
</style>
