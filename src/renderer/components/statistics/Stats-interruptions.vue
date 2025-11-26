<template>
  <div class="StatsInterruptions">
    <div class="StatsInterruptions-header">
      <div class="StatsInterruptions-title">
        <span class="StatsInterruptions-icon">⚠️</span>
        中断原因分析
      </div>
      <div class="StatsInterruptions-subtitle">识别主要干扰因素</div>
    </div>

    <div v-if="stats && stats.length > 0" class="InterruptionChart">
      <div
        v-for="(item, index) in stats"
        :key="index"
        class="InterruptionChart-item"
      >
        <div class="InterruptionChart-rank">{{ index + 1 }}</div>
        <div class="InterruptionChart-content">
          <div class="InterruptionChart-reason">{{ item.reason }}</div>
          <div class="InterruptionChart-bar-wrapper">
            <div
              class="InterruptionChart-bar"
              :style="{ width: getBarWidth(item.count) + '%' }"
            >
              <span class="InterruptionChart-count">{{ item.count }} 次</span>
            </div>
          </div>
        </div>
      </div>

      <div class="InterruptionChart-tip">
        <span class="InterruptionChart-tip-icon">💡</span>
        <span>针对高频干扰因素制定应对策略，提升专注效率</span>
      </div>
    </div>

    <div v-else-if="stats && stats.length === 0" class="StatsInterruptions-empty">
      <div class="StatsInterruptions-empty-icon">✅</div>
      <p>太棒了！最近没有中断记录</p>
      <p class="StatsInterruptions-empty-hint">保持专注状态</p>
    </div>

    <div v-else class="StatsInterruptions-loading">
      加载中...
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatsInterruptions',

  computed: {
    stats() {
      return this.$store.getters.interruptionStats
    },

    maxCount() {
      if (!this.stats || this.stats.length === 0) return 0
      return Math.max(...this.stats.map(s => s.count))
    }
  },

  mounted() {
    this.$store.dispatch('loadInterruptionStats', 30)
  },

  methods: {
    getBarWidth(count) {
      if (this.maxCount === 0) return 0
      return Math.max((count / this.maxCount) * 100, 10)
    }
  }
}
</script>

<style lang="scss" scoped>
.StatsInterruptions {
  background-color: var(--color-background);
  border-radius: 16px;
  padding: 24px;
}

.StatsInterruptions-header {
  margin-bottom: 20px;
  text-align: center;
}

.StatsInterruptions-title {
  align-items: center;
  color: var(--color-foreground);
  display: flex;
  font-size: 18px;
  font-weight: 600;
  justify-content: center;
  margin-bottom: 4px;
}

.StatsInterruptions-icon {
  font-size: 24px;
  margin-right: 8px;
}

.StatsInterruptions-subtitle {
  color: var(--color-foreground-darker);
  font-size: 13px;
}

.InterruptionChart {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.InterruptionChart-item {
  align-items: center;
  display: flex;
  gap: 12px;
}

.InterruptionChart-rank {
  align-items: center;
  background: linear-gradient(135deg, var(--color-accent) 0%, var(--color-accent-dark) 100%);
  border-radius: 50%;
  color: white;
  display: flex;
  flex-shrink: 0;
  font-size: 14px;
  font-weight: 700;
  height: 32px;
  justify-content: center;
  width: 32px;
}

.InterruptionChart-content {
  flex: 1;
}

.InterruptionChart-reason {
  color: var(--color-foreground);
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.InterruptionChart-bar-wrapper {
  background-color: var(--color-background-light);
  border-radius: 8px;
  height: 28px;
  overflow: hidden;
  position: relative;
}

.InterruptionChart-bar {
  align-items: center;
  background: linear-gradient(90deg, #ffc107 0%, #ff9800 100%);
  border-radius: 8px;
  display: flex;
  height: 100%;
  justify-content: flex-end;
  padding-right: 12px;
  transition: width 0.5s ease;
}

.InterruptionChart-count {
  color: white;
  font-size: 12px;
  font-weight: 700;
}

.InterruptionChart-tip {
  align-items: center;
  background-color: rgba(33, 150, 243, 0.1);
  border-left: 3px solid #2196f3;
  border-radius: 8px;
  color: var(--color-foreground-darker);
  display: flex;
  font-size: 13px;
  gap: 8px;
  margin-top: 8px;
  padding: 12px 16px;
}

.InterruptionChart-tip-icon {
  font-size: 18px;
}

.StatsInterruptions-empty {
  padding: 40px 20px;
  text-align: center;
}

.StatsInterruptions-empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.StatsInterruptions-empty p {
  color: var(--color-foreground-darker);
  font-size: 14px;
  margin: 4px 0;
}

.StatsInterruptions-empty-hint {
  color: var(--color-foreground-darkest);
  font-size: 12px;
}

.StatsInterruptions-loading {
  color: var(--color-foreground-darker);
  font-size: 14px;
  padding: 40px;
  text-align: center;
}
</style>
