import { getStatisticsStore } from './StatisticsStore'

/**
 * 统计分析服务类
 * 提供各种维度的数据分析功能
 */
export default class StatisticsAnalyzer {
  constructor() {
    this.store = getStatisticsStore()
  }

  /**
   * 将Date对象格式化为YYYY-MM-DD格式（本地时区）
   * @param {Date} date - 日期对象
   * @returns {string} YYYY-MM-DD格式的日期字符串
   */
  formatLocalDate(date) {
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    return `${year}-${month}-${day}`
  }

  /**
   * 获取今日统计数据
   * @returns {Object} 今日统计信息
   */
  getTodayStats() {
    const today = new Date()
    const sessions = this.store.getSessionsByDate(today)
    return this.calculateDayStats(sessions, today)
  }

  /**
   * 计算单日统计数据
   * @param {Array} sessions - 会话数组
   * @param {Date} date - 日期
   * @returns {Object} 统计数据
   */
  calculateDayStats(sessions, date) {
    const workSessions = sessions.filter(s => s.type === 'work')
    const completedWork = workSessions.filter(s => s.completed)
    const interruptedWork = workSessions.filter(s => s.interrupted)

    const totalMinutes = completedWork.reduce((sum, s) => sum + s.duration, 0)
    const avgFocusTime = completedWork.length > 0
      ? totalMinutes / completedWork.length
      : 0

    return {
      date: this.formatLocalDate(date),
      completedCount: completedWork.length,
      interruptedCount: interruptedWork.length,
      totalSessions: workSessions.length,
      totalMinutes: totalMinutes,
      totalHours: (totalMinutes / 60).toFixed(1),
      avgFocusTime: avgFocusTime.toFixed(1),
      completionRate: workSessions.length > 0
        ? ((completedWork.length / workSessions.length) * 100).toFixed(1)
        : 0,
      sessions: sessions
    }
  }

  /**
   * 获取本周统计数据
   * @returns {Object} 本周统计信息
   */
  getWeekStats() {
    const today = new Date()
    const dayOfWeek = today.getDay()
    const monday = new Date(today)
    monday.setDate(today.getDate() - (dayOfWeek === 0 ? 6 : dayOfWeek - 1))
    monday.setHours(0, 0, 0, 0)

    const sunday = new Date(monday)
    sunday.setDate(monday.getDate() + 6)
    sunday.setHours(23, 59, 59, 999)

    const sessions = this.store.getSessionsByDateRange(monday, sunday)

    // 按天分组
    const dailyStats = []
    for (let i = 0; i < 7; i++) {
      const day = new Date(monday)
      day.setDate(monday.getDate() + i)
      const daySessions = sessions.filter(s => {
        const sessionDate = new Date(s.startTime)
        return sessionDate.toDateString() === day.toDateString()
      })
      dailyStats.push(this.calculateDayStats(daySessions, day))
    }

    const totalCompleted = dailyStats.reduce((sum, day) => sum + day.completedCount, 0)
    const totalMinutes = dailyStats.reduce((sum, day) => sum + day.totalMinutes, 0)

    return {
      weekStart: this.formatLocalDate(monday),
      weekEnd: this.formatLocalDate(sunday),
      dailyStats: dailyStats,
      totalCompleted: totalCompleted,
      totalHours: (totalMinutes / 60).toFixed(1),
      avgPerDay: (totalCompleted / 7).toFixed(1),
      bestDay: this.findBestDay(dailyStats),
      worstDay: this.findWorstDay(dailyStats)
    }
  }

  /**
   * 获取本月统计数据
   * @returns {Object} 本月统计信息
   */
  getMonthStats() {
    const today = new Date()
    const firstDay = new Date(today.getFullYear(), today.getMonth(), 1)
    const lastDay = new Date(today.getFullYear(), today.getMonth() + 1, 0, 23, 59, 59, 999)

    const sessions = this.store.getSessionsByDateRange(firstDay, lastDay)
    const daysInMonth = lastDay.getDate()

    // 按天分组
    const dailyStats = []
    for (let i = 1; i <= daysInMonth; i++) {
      const day = new Date(today.getFullYear(), today.getMonth(), i)
      const daySessions = sessions.filter(s => {
        const sessionDate = new Date(s.startTime)
        return sessionDate.toDateString() === day.toDateString()
      })
      dailyStats.push(this.calculateDayStats(daySessions, day))
    }

    const totalCompleted = dailyStats.reduce((sum, day) => sum + day.completedCount, 0)
    const totalMinutes = dailyStats.reduce((sum, day) => sum + day.totalMinutes, 0)
    const daysWithActivity = dailyStats.filter(day => day.totalSessions > 0).length

    return {
      month: `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}`,
      dailyStats: dailyStats,
      totalCompleted: totalCompleted,
      totalHours: (totalMinutes / 60).toFixed(1),
      avgPerDay: daysWithActivity > 0 ? (totalCompleted / daysWithActivity).toFixed(1) : 0,
      activeDays: daysWithActivity,
      streak: this.calculateStreak(dailyStats)
    }
  }

  /**
   * 获取历史总览统计
   * @returns {Object} 历史统计信息
   */
  getHistoryStats() {
    const allSessions = this.store.getAllSessions()
    const workSessions = allSessions.filter(s => s.type === 'work')
    const completedWork = workSessions.filter(s => s.completed)

    const totalMinutes = completedWork.reduce((sum, s) => sum + s.duration, 0)

    // 计算首次和最后一次记录
    const sortedSessions = [...allSessions].sort((a, b) =>
      new Date(a.startTime) - new Date(b.startTime)
    )

    const firstSession = sortedSessions[0]
    const lastSession = sortedSessions[sortedSessions.length - 1]

    // 计算连续打卡天数
    const currentStreak = this.calculateCurrentStreak()

    return {
      totalSessions: allSessions.length,
      totalCompleted: completedWork.length,
      totalInterrupted: workSessions.filter(s => s.interrupted).length,
      totalHours: (totalMinutes / 60).toFixed(1),
      totalDays: totalMinutes / 60 / 24,
      firstSessionDate: firstSession ? this.formatLocalDate(new Date(firstSession.startTime)) : null,
      lastSessionDate: lastSession ? this.formatLocalDate(new Date(lastSession.startTime)) : null,
      currentStreak: currentStreak,
      avgPerSession: completedWork.length > 0 ? (totalMinutes / completedWork.length).toFixed(1) : 0
    }
  }

  /**
   * 生成时间分布热力图数据
   * @param {number} weeks - 要显示的周数，默认4周
   * @returns {Array} 热力图数据
   */
  getHeatmapData(weeks = 4) {
    const today = new Date()
    const startDate = new Date(today)
    startDate.setDate(today.getDate() - (weeks * 7))
    startDate.setHours(0, 0, 0, 0)

    const sessions = this.store.getSessionsByDateRange(startDate, today)

    // 创建 24小时 x 7天 的矩阵
    const heatmap = Array(7).fill(null).map(() => Array(24).fill(0))

    sessions.forEach(session => {
      if (session.type === 'work' && session.completed) {
        const date = new Date(session.startTime)
        const dayOfWeek = date.getDay()
        const hour = date.getHours()
        heatmap[dayOfWeek][hour]++
      }
    })

    return heatmap
  }

  /**
   * 获取中断原因统计
   * @param {number} days - 统计天数，默认30天
   * @returns {Array} 中断原因统计数组
   */
  getInterruptionStats(days = 30) {
    const today = new Date()
    const startDate = new Date(today)
    startDate.setDate(today.getDate() - days)

    const sessions = this.store.getSessionsByDateRange(startDate, today)
    const interruptedSessions = sessions.filter(s => s.interrupted && s.interruptReason)

    const reasonCounts = {}
    interruptedSessions.forEach(session => {
      const reason = session.interruptReason
      reasonCounts[reason] = (reasonCounts[reason] || 0) + 1
    })

    return Object.entries(reasonCounts)
      .map(([reason, count]) => ({ reason, count }))
      .sort((a, b) => b.count - a.count)
  }

  /**
   * 获取完成率趋势数据
   * @param {number} days - 天数
   * @returns {Array} 趋势数据
   */
  getCompletionTrend(days = 30) {
    const today = new Date()
    const trend = []

    for (let i = days - 1; i >= 0; i--) {
      const date = new Date(today)
      date.setDate(today.getDate() - i)
      const sessions = this.store.getSessionsByDate(date)
      const stats = this.calculateDayStats(sessions, date)

      trend.push({
        date: stats.date,
        completionRate: parseFloat(stats.completionRate),
        completedCount: stats.completedCount
      })
    }

    return trend
  }

  /**
   * 查找最高效的一天
   */
  findBestDay(dailyStats) {
    const sorted = [...dailyStats].sort((a, b) => b.completedCount - a.completedCount)
    return sorted[0] || null
  }

  /**
   * 查找最低效的一天
   */
  findWorstDay(dailyStats) {
    const withActivity = dailyStats.filter(day => day.totalSessions > 0)
    const sorted = [...withActivity].sort((a, b) => a.completedCount - b.completedCount)
    return sorted[0] || null
  }

  /**
   * 计算连续打卡天数（从今天或数组中最近有数据的日期往前算）
   */
  calculateStreak(dailyStats) {
    let streak = 0
    const today = new Date()
    today.setHours(0, 0, 0, 0)

    // 过滤掉未来日期，只保留到今天为止的数据
    const validStats = dailyStats.filter(day => {
      const [year, month, dayNum] = day.date.split('-').map(Number)
      const dayDate = new Date(year, month - 1, dayNum)
      return dayDate <= today
    })

    const reversed = [...validStats].reverse()

    // 如果今天没有数据，从昨天开始算
    let startIndex = 0
    if (reversed.length > 0 && reversed[0].completedCount === 0) {
      startIndex = 1
    }

    for (let i = startIndex; i < reversed.length; i++) {
      if (reversed[i].completedCount > 0) {
        streak++
      } else {
        break
      }
    }

    return streak
  }

  /**
   * 计算当前连续打卡天数（从今天往前）
   */
  calculateCurrentStreak() {
    const allSessions = this.store.getAllSessions()
    if (allSessions.length === 0) return 0

    const today = new Date()
    today.setHours(0, 0, 0, 0)

    let streak = 0
    const checkDate = new Date(today)

    while (true) {
      const daySessions = this.store.getSessionsByDate(checkDate)
      const hasCompletedWork = daySessions.some(s => s.type === 'work' && s.completed)

      if (hasCompletedWork) {
        streak++
        checkDate.setDate(checkDate.getDate() - 1)
      } else {
        // 如果是今天没有记录，继续往前查
        if (checkDate.toDateString() === today.toDateString()) {
          checkDate.setDate(checkDate.getDate() - 1)
          continue
        }
        break
      }
    }

    return streak
  }

  /**
   * 按任务统计
   * @param {number} days - 统计天数
   * @returns {Array} 任务统计数组
   */
  getTaskStats(days = 30) {
    const today = new Date()
    const startDate = new Date(today)
    startDate.setDate(today.getDate() - days)

    const sessions = this.store.getSessionsByDateRange(startDate, today)
    const workSessions = sessions.filter(s => s.type === 'work' && s.completed && s.taskName)

    const taskCounts = {}
    const taskMinutes = {}

    workSessions.forEach(session => {
      const task = session.taskName
      taskCounts[task] = (taskCounts[task] || 0) + 1
      taskMinutes[task] = (taskMinutes[task] || 0) + session.duration
    })

    return Object.entries(taskCounts)
      .map(([task, count]) => ({
        task,
        count,
        totalMinutes: taskMinutes[task],
        totalHours: (taskMinutes[task] / 60).toFixed(1)
      }))
      .sort((a, b) => b.count - a.count)
  }
}

/**
 * 创建统计分析器单例
 */
let analyzerInstance = null

export function getStatisticsAnalyzer() {
  if (!analyzerInstance) {
    analyzerInstance = new StatisticsAnalyzer()
  }
  return analyzerInstance
}
