<template>
  <div class="Container">
    <p class="Drawer-heading">Themes</p>
    <div
      v-for="(theme, i) in themer.themes"
      :key="i"
      class="Setting-wrapper"
      @click="selectTheme(getThemeName(theme))"
    >
      <p class="Setting-title">
        {{ titleCase(getThemeName(theme)) }}
      </p>
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
    getThemeName(theme) {
      return Object.keys(theme)[0]
    },
    selectTheme(themeName) {
      const payload = { key: 'theme', val: themeName }
      this.$store.dispatch('setSetting', payload)
      this.themer.apply(themeName)
    },
    titleCase(string) {
      return string.charAt(0).toUpperCase() + string.slice(1)
    }
  }
}
</script>

<style lang="scss" scoped>
.Setting-wrapper {
  background-color: var(--color-background);
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  margin: 12px 0;
  padding: 12px;
}

.Setting-title {
  color: var(--color-foreground-darker);
  font-size: 14px;
  letter-spacing: 0.05em;
}
</style>
