<template>
  <div class="StatsMonthView">
    <div v-if="stats" class="StatsMonthView-content">
      <div class="StatsMonthView-header">
        <h2 class="StatsMonthView-title">本月统计</h2>
        <div class="StatsMonthView-date">{{ formattedMonth }}</div>
      </div>

      <div class="StatsMonthView-summary">
        <div class="SummaryCard">
          <div class="SummaryCard-icon">🍅</div>
          <div class="SummaryCard-value">{{ stats.totalCompleted }}</div>
          <div class="SummaryCard-label">总完成数</div>
        </div>
        <div class="SummaryCard">
          <div class="SummaryCard-icon">⏱️</div>
          <div class="SummaryCard-value">{{ stats.totalHours }}</div>
          <div class="SummaryCard-label">总时长(小时)</div>
        </div>
        <div class="SummaryCard">
          <div class="SummaryCard-icon">📅</div>
          <div class="SummaryCard-value">{{ stats.activeDays }}</div>
          <div class="SummaryCard-label">活跃天数</div>
        </div>
        <div class="SummaryCard SummaryCard--streak">
          <div class="SummaryCard-icon">🔥</div>
          <div class="SummaryCard-value">{{ stats.streak }}</div>
          <div class="SummaryCard-label">连续打卡</div>
        </div>
      </div>

      <div class="StatsMonthView-calendar">
        <div class="Calendar-title">月历视图</div>
        <div class="Calendar">
          <div class="Calendar-weekdays">
            <div
              v-for="day in weekdays"
              :key="day"
              class="Calendar-weekday"
            >
              {{ day }}
            </div>
          </div>
          <div class="Calendar-days">
            <div
              v-for="(day, index) in calendarDays"
              :key="index"
              class="Calendar-day"
              :class="{
                'Calendar-day--empty': !day.date,
                'Calendar-day--today': day.isToday,
                'Calendar-day--has-data': day.count > 0
              }"
              :style="{ opacity: day.count > 0 ? 0.3 + (day.count / maxDayCount) * 0.7 : 1 }"
            >
              <div class="Calendar-day-num">{{ day.dayNum }}</div>
              <div v-if="day.count > 0" class="Calendar-day-count">{{ day.count }}</div>
            </div>
          </div>
        </div>
        <div class="Calendar-legend">
          <span>少</span>
          <div class="Calendar-legend-gradient"></div>
          <span>多</span>
        </div>
      </div>
    </div>

    <div v-else class="StatsMonthView-loading">
      加载中...
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatsMonthView',

  data() {
    return {
      weekdays: ['一', '二', '三', '四', '五', '六', '日']
    }
  },

  computed: {
    stats() {
      return this.$store.getters.monthStats
    },

    formattedMonth() {
      if (!this.stats) return ''
      // 解析YYYY-MM格式
      const [year, month] = this.stats.month.split('-').map(Number)
      return `${year}年${month}月`
    },

    calendarDays() {
      if (!this.stats || !this.stats.dailyStats) return []

      const firstDay = this.parseLocalDate(this.stats.dailyStats[0].date)
      const dayOfWeek = firstDay.getDay()
      const offset = dayOfWeek === 0 ? 6 : dayOfWeek - 1

      const days = []

      // 添加空白天
      for (let i = 0; i < offset; i++) {
        days.push({ date: null, dayNum: '', count: 0, isToday: false })
      }

      // 添加实际天数
      const today = new Date()
      today.setHours(0, 0, 0, 0)
      this.stats.dailyStats.forEach(day => {
        const date = this.parseLocalDate(day.date)
        const isToday = date.toDateString() === today.toDateString()
        days.push({
          date: day.date,
          dayNum: date.getDate(),
          count: day.completedCount,
          isToday
        })
      })

      return days
    },

    maxDayCount() {
      if (!this.stats || !this.stats.dailyStats) return 0
      return Math.max(...this.stats.dailyStats.map(d => d.completedCount), 1)
    }
  },

  mounted() {
    this.$store.dispatch('loadMonthStats')
  },

  methods: {
    parseLocalDate(dateStr) {
      // 解析YYYY-MM-DD格式的日期字符串为本地时区日期
      const [year, month, day] = dateStr.split('-').map(Number)
      return new Date(year, month - 1, day)
    }
  }
}
</script>

<style lang="scss" scoped>
.StatsMonthView {
  height: 100%;
  overflow-y: auto;
  padding: 20px;
  padding-bottom: 40px;
}

.StatsMonthView-content {
  max-width: 900px;
  margin: 0 auto;
}

.StatsMonthView-header {
  margin-bottom: 24px;
  text-align: center;
}

.StatsMonthView-title {
  color: var(--color-foreground);
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 8px 0;
}

.StatsMonthView-date {
  color: var(--color-foreground-darker);
  font-size: 14px;
}

.StatsMonthView-summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
  margin-bottom: 32px;
}

.SummaryCard {
  background-color: var(--color-background);
  border-radius: 12px;
  padding: 20px;
  text-align: center;

  &--streak {
    background: linear-gradient(135deg, #ff6b6b 0%, #ee5a6f 100%);

    .SummaryCard-value,
    .SummaryCard-label {
      color: white;
    }
  }
}

.SummaryCard-icon {
  font-size: 28px;
  margin-bottom: 8px;
}

.SummaryCard-value {
  color: var(--color-foreground);
  font-size: 28px;
  font-weight: 700;
  margin-bottom: 4px;
}

.SummaryCard-label {
  color: var(--color-foreground-darker);
  font-size: 12px;
  text-transform: uppercase;
}

.StatsMonthView-calendar {
  background-color: var(--color-background);
  border-radius: 12px;
  padding: 24px;
}

.Calendar-title {
  color: var(--color-foreground);
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
}

.Calendar-weekdays {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
  margin-bottom: 8px;
}

.Calendar-weekday {
  color: var(--color-foreground-darker);
  font-size: 12px;
  padding: 8px;
  text-align: center;
}

.Calendar-days {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.Calendar-day {
  aspect-ratio: 1;
  background-color: var(--color-background-light);
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  transition: all 0.2s ease;

  &:hover:not(&--empty) {
    transform: scale(1.05);
  }

  &--empty {
    background-color: transparent;
    cursor: default;
  }

  &--today {
    border: 2px solid var(--color-accent);
  }

  &--has-data {
    background-color: var(--color-accent);

    .Calendar-day-num {
      color: white;
    }
  }
}

.Calendar-day-num {
  color: var(--color-foreground);
  font-size: 13px;
  font-weight: 600;
}

.Calendar-day-count {
  color: white;
  font-size: 10px;
  font-weight: 700;
  margin-top: 2px;
}

.Calendar-legend {
  align-items: center;
  color: var(--color-foreground-darker);
  display: flex;
  font-size: 12px;
  gap: 8px;
  justify-content: center;
  margin-top: 16px;
}

.Calendar-legend-gradient {
  background: linear-gradient(90deg,
    var(--color-background-light) 0%,
    var(--color-accent) 100%
  );
  border-radius: 3px;
  height: 12px;
  width: 120px;
}

.StatsMonthView-loading {
  align-items: center;
  color: var(--color-foreground-darker);
  display: flex;
  font-size: 16px;
  height: 100%;
  justify-content: center;
}
</style>
