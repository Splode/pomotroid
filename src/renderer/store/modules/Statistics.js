import { getStatisticsStore } from '@/utils/StatisticsStore'
import { getStatisticsAnalyzer } from '@/utils/StatisticsAnalyzer'

const statisticsStore = getStatisticsStore()
const analyzer = getStatisticsAnalyzer()

const state = {
  currentSession: null, // 当前正在进行的会话
  currentView: 'day', // 当前统计视图: 'day', 'week', 'month', 'history'
  dayStats: null,
  weekStats: null,
  monthStats: null,
  historyStats: null,
  heatmapData: null,
  interruptionStats: null,
  completionTrend: null,
  taskStats: null,
  showInterruptDialog: false
}

const getters = {
  currentSession: state => state.currentSession,
  currentView: state => state.currentView,
  dayStats: state => state.dayStats,
  weekStats: state => state.weekStats,
  monthStats: state => state.monthStats,
  historyStats: state => state.historyStats,
  heatmapData: state => state.heatmapData,
  interruptionStats: state => state.interruptionStats,
  completionTrend: state => state.completionTrend,
  taskStats: state => state.taskStats,
  showInterruptDialog: state => state.showInterruptDialog
}

const mutations = {
  SET_CURRENT_SESSION(state, session) {
    state.currentSession = session
  },

  SET_CURRENT_VIEW(state, view) {
    state.currentView = view
  },

  SET_DAY_STATS(state, stats) {
    state.dayStats = stats
  },

  SET_WEEK_STATS(state, stats) {
    state.weekStats = stats
  },

  SET_MONTH_STATS(state, stats) {
    state.monthStats = stats
  },

  SET_HISTORY_STATS(state, stats) {
    state.historyStats = stats
  },

  SET_HEATMAP_DATA(state, data) {
    state.heatmapData = data
  },

  SET_INTERRUPTION_STATS(state, stats) {
    state.interruptionStats = stats
  },

  SET_COMPLETION_TREND(state, trend) {
    state.completionTrend = trend
  },

  SET_TASK_STATS(state, stats) {
    state.taskStats = stats
  },

  SET_SHOW_INTERRUPT_DIALOG(state, show) {
    state.showInterruptDialog = show
  },

  CLEAR_CURRENT_SESSION(state) {
    state.currentSession = null
  }
}

const actions = {
  /**
   * 开始新的番茄钟会话
   */
  startSession({ commit }, { type, duration, taskName, taskId }) {
    const session = statisticsStore.createSession({
      type,
      duration,
      taskName,
      taskId
    })
    // 深拷贝 session，避免 Vuex state 和 statisticsStore 共享同一对象引用
    commit('SET_CURRENT_SESSION', JSON.parse(JSON.stringify(session)))
    return session
  },

  /**
   * 完成当前会话
   */
  completeSession({ state, commit, dispatch }, { completed = true, interruptReason = null }) {
    if (!state.currentSession) {
      return
    }

    statisticsStore.completeSession(
      state.currentSession.id,
      completed,
      interruptReason
    )

    commit('CLEAR_CURRENT_SESSION')

    // 刷新统计数据
    dispatch('refreshStats')
  },

  /**
   * 中断当前会话（显示对话框）
   */
  interruptSession({ commit }) {
    commit('SET_SHOW_INTERRUPT_DIALOG', true)
  },

  /**
   * 提交中断原因
   */
  submitInterruptReason({ dispatch }, reason) {
    dispatch('completeSession', {
      completed: false,
      interruptReason: reason
    })
  },

  /**
   * 切换统计视图
   */
  setCurrentView({ commit }, view) {
    commit('SET_CURRENT_VIEW', view)
  },

  /**
   * 刷新所有统计数据
   */
  refreshStats({ commit }) {
    commit('SET_DAY_STATS', analyzer.getTodayStats())
    commit('SET_WEEK_STATS', analyzer.getWeekStats())
    commit('SET_MONTH_STATS', analyzer.getMonthStats())
    commit('SET_HISTORY_STATS', analyzer.getHistoryStats())
  },

  /**
   * 加载日统计
   */
  loadDayStats({ commit }) {
    commit('SET_DAY_STATS', analyzer.getTodayStats())
  },

  /**
   * 加载周统计
   */
  loadWeekStats({ commit }) {
    commit('SET_WEEK_STATS', analyzer.getWeekStats())
  },

  /**
   * 加载月统计
   */
  loadMonthStats({ commit }) {
    commit('SET_MONTH_STATS', analyzer.getMonthStats())
  },

  /**
   * 加载历史统计
   */
  loadHistoryStats({ commit }) {
    commit('SET_HISTORY_STATS', analyzer.getHistoryStats())
  },

  /**
   * 加载热力图数据
   */
  loadHeatmapData({ commit }, weeks = 4) {
    commit('SET_HEATMAP_DATA', analyzer.getHeatmapData(weeks))
  },

  /**
   * 加载中断统计
   */
  loadInterruptionStats({ commit }, days = 30) {
    commit('SET_INTERRUPTION_STATS', analyzer.getInterruptionStats(days))
  },

  /**
   * 加载完成率趋势
   */
  loadCompletionTrend({ commit }, days = 30) {
    commit('SET_COMPLETION_TREND', analyzer.getCompletionTrend(days))
  },

  /**
   * 加载任务统计
   */
  loadTaskStats({ commit }, days = 30) {
    commit('SET_TASK_STATS', analyzer.getTaskStats(days))
  },

  /**
   * 关闭中断对话框
   */
  closeInterruptDialog({ commit }) {
    commit('SET_SHOW_INTERRUPT_DIALOG', false)
  },

  /**
   * 删除会话
   */
  deleteSession({ dispatch }, sessionId) {
    statisticsStore.deleteSession(sessionId)
    dispatch('refreshStats')
  },

  /**
   * 清空所有数据
   */
  clearAllData({ commit, dispatch }) {
    statisticsStore.clearAllSessions()
    commit('CLEAR_CURRENT_SESSION')
    dispatch('refreshStats')
  },

  /**
   * 导出数据
   */
  exportData() {
    return statisticsStore.exportToJSON()
  },

  /**
   * 导入数据
   */
  importData({ dispatch }, jsonString) {
    statisticsStore.importFromJSON(jsonString)
    dispatch('refreshStats')
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
