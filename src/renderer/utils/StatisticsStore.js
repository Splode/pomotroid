import { logger } from './logger'
const electron = require('electron')
const fs = require('fs')
const path = require('path')

// UUID 生成函数（简单版本，避免依赖外部库）
function generateUUID() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0
    const v = c === 'x' ? r : (r & 0x3 | 0x8)
    return v.toString(16)
  })
}

/**
 * 统计数据存储类
 * 负责管理番茄钟会话记录的持久化存储
 */
export default class StatisticsStore {
  /**
   * 创建统计数据存储实例
   * @param {string} filename - 存储文件名
   */
  constructor(filename = 'pomodoro-sessions') {
    const userDataPath = this.getUserDir()
    this.path = path.join(userDataPath, filename + '.json')
    this.data = this.loadData()
    logger.info(`Statistics data loaded from ${this.path}`)
  }

  /**
   * 加载数据文件
   * @returns {Object} 包含 sessions 数组的数据对象
   */
  loadData() {
    if (!fs.existsSync(this.path)) {
      const initialData = { sessions: [], version: '1.0' }
      fs.writeFileSync(this.path, JSON.stringify(initialData, null, 2))
      return initialData
    }

    try {
      const content = fs.readFileSync(this.path, 'utf8')
      return JSON.parse(content)
    } catch (error) {
      logger.error('Failed to load statistics data:', error)
      return { sessions: [], version: '1.0' }
    }
  }

  /**
   * 保存数据到文件
   */
  saveData() {
    try {
      fs.writeFileSync(this.path, JSON.stringify(this.data, null, 2))
    } catch (error) {
      logger.error('Failed to save statistics data:', error)
    }
  }

  /**
   * 创建新的番茄钟会话记录
   * @param {Object} sessionData - 会话数据
   * @param {string} sessionData.type - 会话类型 ('work', 'short-break', 'long-break')
   * @param {number} sessionData.duration - 会话时长（分钟）
   * @param {string} sessionData.taskName - 关联任务名称（可选）
   * @returns {Object} 创建的会话对象
   */
  createSession(sessionData) {
    const session = {
      id: generateUUID(),
      type: sessionData.type,
      duration: sessionData.duration,
      startTime: new Date().toISOString(),
      endTime: null,
      completed: false,
      interrupted: false,
      interruptReason: null,
      taskName: sessionData.taskName || null,
      taskId: sessionData.taskId || null
    }

    this.data.sessions.push(session)
    this.saveData()
    logger.info(`Session created: ${session.id}`)
    return session
  }

  /**
   * 完成番茄钟会话
   * @param {string} sessionId - 会话ID
   * @param {boolean} completed - 是否正常完成
   * @param {string} interruptReason - 中断原因（如果未完成）
   */
  completeSession(sessionId, completed = true, interruptReason = null) {
    const session = this.data.sessions.find(s => s.id === sessionId)
    if (!session) {
      logger.warn(`Session not found: ${sessionId}`)
      return
    }

    session.endTime = new Date().toISOString()
    session.completed = completed
    session.interrupted = !completed
    session.interruptReason = interruptReason

    this.saveData()
    logger.info(`Session ${completed ? 'completed' : 'interrupted'}: ${sessionId}`)
  }

  /**
   * 获取所有会话记录
   * @returns {Array} 会话数组
   */
  getAllSessions() {
    return this.data.sessions
  }

  /**
   * 获取指定日期范围的会话
   * @param {Date} startDate - 开始日期
   * @param {Date} endDate - 结束日期
   * @returns {Array} 过滤后的会话数组
   */
  getSessionsByDateRange(startDate, endDate) {
    return this.data.sessions.filter(session => {
      const sessionDate = new Date(session.startTime)
      return sessionDate >= startDate && sessionDate <= endDate
    })
  }

  /**
   * 获取指定日期的会话
   * @param {Date} date - 日期
   * @returns {Array} 当天的会话数组
   */
  getSessionsByDate(date) {
    const startOfDay = new Date(date)
    startOfDay.setHours(0, 0, 0, 0)
    const endOfDay = new Date(date)
    endOfDay.setHours(23, 59, 59, 999)

    return this.getSessionsByDateRange(startOfDay, endOfDay)
  }

  /**
   * 删除指定会话
   * @param {string} sessionId - 会话ID
   */
  deleteSession(sessionId) {
    const index = this.data.sessions.findIndex(s => s.id === sessionId)
    if (index !== -1) {
      this.data.sessions.splice(index, 1)
      this.saveData()
      logger.info(`Session deleted: ${sessionId}`)
    }
  }

  /**
   * 清空所有会话数据
   */
  clearAllSessions() {
    this.data.sessions = []
    this.saveData()
    logger.info('All sessions cleared')
  }

  /**
   * 获取用户数据目录
   * @returns {string} 用户数据目录路径
   */
  getUserDir() {
    try {
      return (electron.app || electron.remote.app).getPath('userData')
    } catch (error) {
      logger.error('Failed to get user directory', error)
      return '.'
    }
  }

  /**
   * 导出数据为 JSON
   * @returns {string} JSON 字符串
   */
  exportToJSON() {
    return JSON.stringify(this.data, null, 2)
  }

  /**
   * 从 JSON 导入数据
   * @param {string} jsonString - JSON 字符串
   */
  importFromJSON(jsonString) {
    try {
      const importedData = JSON.parse(jsonString)
      if (importedData.sessions && Array.isArray(importedData.sessions)) {
        this.data = importedData
        this.saveData()
        logger.info('Data imported successfully')
      }
    } catch (error) {
      logger.error('Failed to import data:', error)
      throw new Error('Invalid JSON format')
    }
  }
}

/**
 * 创建统计数据存储单例
 */
let statisticsStoreInstance = null

export function getStatisticsStore() {
  if (!statisticsStoreInstance) {
    statisticsStoreInstance = new StatisticsStore()
  }
  return statisticsStoreInstance
}
