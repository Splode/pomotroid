<template>
  <div class="DrawerStatistics">
    <div class="DrawerStatistics-nav">
      <button
        v-for="view in views"
        :key="view.id"
        class="DrawerStatistics-nav-btn"
        :class="{ 'is-active': currentView === view.id }"
        @click="setView(view.id)"
      >
        <span class="DrawerStatistics-nav-icon">{{ view.icon }}</span>
        <span class="DrawerStatistics-nav-label">{{ view.label }}</span>
      </button>
    </div>

    <div class="DrawerStatistics-content">
      <transition name="slide-fade" mode="out-in">
        <component :is="currentComponent" :key="currentView" />
      </transition>
    </div>
  </div>
</template>

<script>
import StatsDayView from '../statistics/Stats-day.vue'
import StatsWeekView from '../statistics/Stats-week.vue'
import StatsMonthView from '../statistics/Stats-month.vue'
import StatsHistoryView from '../statistics/Stats-history.vue'

export default {
  name: 'DrawerStatistics',

  components: {
    StatsDayView,
    StatsWeekView,
    StatsMonthView,
    StatsHistoryView
  },

  data() {
    return {
      views: [
        { id: 'day', label: '今日', icon: '📅' },
        { id: 'week', label: '本周', icon: '📊' },
        { id: 'month', label: '本月', icon: '📈' },
        { id: 'history', label: '历史', icon: '🏆' }
      ]
    }
  },

  computed: {
    currentView() {
      return this.$store.getters.currentView || 'day'
    },

    currentComponent() {
      const componentMap = {
        day: 'StatsDayView',
        week: 'StatsWeekView',
        month: 'StatsMonthView',
        history: 'StatsHistoryView'
      }
      return componentMap[this.currentView] || 'StatsDayView'
    }
  },

  methods: {
    setView(view) {
      this.$store.dispatch('setCurrentView', view)
    }
  }
}
</script>

<style lang="scss" scoped>
.DrawerStatistics {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}

.DrawerStatistics-nav {
  background-color: var(--color-background);
  border-bottom: 1px solid var(--color-background-light);
  display: flex;
  flex-shrink: 0;
  gap: 4px;
  padding: 12px;
}

.DrawerStatistics-nav-btn {
  align-items: center;
  background-color: transparent;
  border: none;
  border-radius: 8px;
  color: var(--color-foreground-darker);
  cursor: pointer;
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--color-background-light);
  }

  &.is-active {
    background-color: var(--color-accent);
    color: white;

    .DrawerStatistics-nav-icon {
      transform: scale(1.1);
    }
  }
}

.DrawerStatistics-nav-icon {
  font-size: 20px;
  transition: transform 0.2s ease;
}

.DrawerStatistics-nav-label {
  font-size: 12px;
  font-weight: 600;
}

.DrawerStatistics-content {
  background-color: var(--color-background-light);
  flex: 1;
  overflow: hidden;
  position: relative;
  padding-bottom: 36px;
}

.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease;
}

.slide-fade-enter {
  opacity: 0;
  transform: translateX(10px);
}

.slide-fade-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}
</style>
