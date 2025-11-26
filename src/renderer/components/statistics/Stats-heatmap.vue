<template>
  <div class="StatsHeatmap">
    <div class="StatsHeatmap-header">
      <div class="StatsHeatmap-title">
        <span class="StatsHeatmap-icon">🔥</span>
        时间分布热力图
      </div>
      <div class="StatsHeatmap-subtitle">发现你的高效时段</div>
    </div>

    <div v-if="heatmapData" class="Heatmap">
      <div class="Heatmap-container">
        <!-- 时间轴标签 -->
        <div class="Heatmap-hours">
          <div class="Heatmap-corner"></div>
          <div
            v-for="hour in displayHours"
            :key="`hour-${hour}`"
            class="Heatmap-hour-label"
          >
            {{ hour }}
          </div>
        </div>

        <!-- 热力图主体 -->
        <div class="Heatmap-body">
          <div
            v-for="(dayData, dayIndex) in heatmapData"
            :key="`day-${dayIndex}`"
            class="Heatmap-row"
          >
            <div class="Heatmap-day-label">{{ getDayLabel(dayIndex) }}</div>
            <div class="Heatmap-cells">
              <div
                v-for="(count, hourIndex) in dayData"
                :key="`cell-${dayIndex}-${hourIndex}`"
                class="Heatmap-cell"
                :class="getCellClass(count)"
                :title="`${getDayLabel(dayIndex)} ${hourIndex}:00 - ${count} 个番茄钟`"
              >
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 图例 -->
      <div class="Heatmap-legend">
        <span class="Heatmap-legend-label">强度:</span>
        <div class="Heatmap-legend-item">
          <div class="Heatmap-cell Heatmap-cell--level-0"></div>
          <span>无</span>
        </div>
        <div class="Heatmap-legend-item">
          <div class="Heatmap-cell Heatmap-cell--level-1"></div>
          <span>低</span>
        </div>
        <div class="Heatmap-legend-item">
          <div class="Heatmap-cell Heatmap-cell--level-2"></div>
          <span>中</span>
        </div>
        <div class="Heatmap-legend-item">
          <div class="Heatmap-cell Heatmap-cell--level-3"></div>
          <span>高</span>
        </div>
        <div class="Heatmap-legend-item">
          <div class="Heatmap-cell Heatmap-cell--level-4"></div>
          <span>很高</span>
        </div>
      </div>

      <!-- 洞察 -->
      <div v-if="insights" class="Heatmap-insights">
        <div class="Insight">
          <span class="Insight-icon">⭐</span>
          <span class="Insight-text">最高效时段: {{ insights.peakHour }}</span>
        </div>
        <div class="Insight">
          <span class="Insight-icon">📅</span>
          <span class="Insight-text">最高效日: {{ insights.peakDay }}</span>
        </div>
      </div>
    </div>

    <div v-else class="StatsHeatmap-loading">
      加载热力图数据...
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatsHeatmap',

  data() {
    return {
      displayHours: [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22]
    }
  },

  computed: {
    heatmapData() {
      return this.$store.getters.heatmapData
    },

    maxCount() {
      if (!this.heatmapData) return 0
      return Math.max(...this.heatmapData.flat())
    },

    insights() {
      if (!this.heatmapData) return null

      // 找到最高效的小时
      let maxHourCount = 0
      let peakHour = 0
      for (let hour = 0; hour < 24; hour++) {
        const hourTotal = this.heatmapData.reduce((sum, day) => sum + day[hour], 0)
        if (hourTotal > maxHourCount) {
          maxHourCount = hourTotal
          peakHour = hour
        }
      }

      // 找到最高效的日
      const dayTotals = this.heatmapData.map(day => day.reduce((sum, count) => sum + count, 0))
      const peakDayIndex = dayTotals.indexOf(Math.max(...dayTotals))

      return {
        peakHour: `${peakHour}:00 - ${peakHour + 1}:00`,
        peakDay: this.getDayLabel(peakDayIndex)
      }
    }
  },

  mounted() {
    this.$store.dispatch('loadHeatmapData', 4)
  },

  methods: {
    getDayLabel(index) {
      const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六']
      return days[index]
    },

    getCellClass(count) {
      if (count === 0) return 'Heatmap-cell--level-0'
      if (this.maxCount === 0) return 'Heatmap-cell--level-0'

      const ratio = count / this.maxCount
      if (ratio <= 0.25) return 'Heatmap-cell--level-1'
      if (ratio <= 0.5) return 'Heatmap-cell--level-2'
      if (ratio <= 0.75) return 'Heatmap-cell--level-3'
      return 'Heatmap-cell--level-4'
    }
  }
}
</script>

<style lang="scss" scoped>
.StatsHeatmap {
  background-color: var(--color-background);
  border-radius: 16px;
  padding: 24px;
}

.StatsHeatmap-header {
  margin-bottom: 20px;
  text-align: center;
}

.StatsHeatmap-title {
  align-items: center;
  color: var(--color-foreground);
  display: flex;
  font-size: 18px;
  font-weight: 600;
  justify-content: center;
  margin-bottom: 4px;
}

.StatsHeatmap-icon {
  font-size: 24px;
  margin-right: 8px;
}

.StatsHeatmap-subtitle {
  color: var(--color-foreground-darker);
  font-size: 13px;
}

.Heatmap {
  overflow-x: auto;
}

.Heatmap-container {
  max-width: 500px;
  margin: 0 auto;
}

.Heatmap-hours {
  display: grid;
  grid-template-columns: 40px repeat(12, 1fr);
  gap: 2px;
  margin-bottom: 2px;
}

.Heatmap-corner {
  width: 40px;
}

.Heatmap-hour-label {
  color: var(--color-foreground-darker);
  font-size: 9px;
  text-align: center;
}

.Heatmap-body {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.Heatmap-row {
  display: grid;
  grid-template-columns: 40px 1fr;
  gap: 2px;
}

.Heatmap-day-label {
  align-items: center;
  color: var(--color-foreground-darker);
  display: flex;
  font-size: 11px;
  font-weight: 600;
  justify-content: flex-end;
  padding-right: 6px;
}

.Heatmap-cells {
  display: grid;
  grid-template-columns: repeat(24, 1fr);
  gap: 1px;
}

.Heatmap-cell {
  aspect-ratio: 1;
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.2s ease;
  max-width: 16px;
  max-height: 16px;

  &:hover {
    transform: scale(1.2);
    z-index: 1;
  }

  &--level-0 {
    background-color: var(--color-background-light);
  }

  &--level-1 {
    background-color: rgba(var(--color-accent-rgb, 76, 175, 80), 0.3);
  }

  &--level-2 {
    background-color: rgba(var(--color-accent-rgb, 76, 175, 80), 0.5);
  }

  &--level-3 {
    background-color: rgba(var(--color-accent-rgb, 76, 175, 80), 0.75);
  }

  &--level-4 {
    background-color: var(--color-accent);
  }
}

.Heatmap-legend {
  align-items: center;
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-top: 20px;
}

.Heatmap-legend-label {
  color: var(--color-foreground-darker);
  font-size: 12px;
  margin-right: 4px;
}

.Heatmap-legend-item {
  align-items: center;
  display: flex;
  gap: 4px;

  span {
    color: var(--color-foreground-darker);
    font-size: 11px;
  }

  .Heatmap-cell {
    height: 12px;
    width: 12px;
  }
}

.Heatmap-insights {
  border-top: 1px solid var(--color-background-light);
  display: flex;
  gap: 24px;
  justify-content: center;
  margin-top: 20px;
  padding-top: 16px;
}

.Insight {
  align-items: center;
  display: flex;
  gap: 8px;
}

.Insight-icon {
  font-size: 18px;
}

.Insight-text {
  color: var(--color-foreground);
  font-size: 13px;
  font-weight: 600;
}

.StatsHeatmap-loading {
  color: var(--color-foreground-darker);
  font-size: 14px;
  padding: 40px;
  text-align: center;
}
</style>
