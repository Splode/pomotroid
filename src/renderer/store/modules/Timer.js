import { localStore } from './index'
import { defaults } from './../../utils/LocalStore'

const state = {
  round: 1,
  workRounds: parseInt(localStore.get('workRounds')),
  totalWorkRounds: 0,
  tickSounds: localStore.get('tickSounds'),
  tickSoundsDuringBreak: localStore.get('tickSoundsDuringBreak'),
  timeLongBreak: parseInt(localStore.get('timeLongBreak')),
  timeShortBreak: parseInt(localStore.get('timeShortBreak')),
  timeWork: parseInt(localStore.get('timeWork')),
  currentRound: 'work', // work, short-break, long-break
  volume: localStore.get('volume') || 100,
  globalShortcuts: localStore.get('globalShortcuts') || {}
}

const getters = {
  round() {
    return state.round
  },
  workRounds() {
    return state.workRounds
  },
  tickSounds() {
    return state.tickSounds
  },
  tickSoundsDuringBreak() {
    return state.tickSoundsDuringBreak
  },
  totalWorkRounds() {
    return state.totalWorkRounds
  },
  timeLongBreak() {
    return state.timeLongBreak
  },
  timeShortBreak() {
    return state.timeShortBreak
  },
  timeWork() {
    return state.timeWork
  },
  currentRound() {
    return state.currentRound
  },
  volume() {
    return state.volume
  },
  globalShortcuts() {
    return state.globalShortcuts
  }
}

const mutations = {
  INCREMENT_ROUND(state) {
    state.round += 1
  },

  RESET_ROUND(state) {
    state.round = 1
  },

  INCREMENT_TOTAL_WORK_ROUNDS(state) {
    state.totalWorkRounds += 1
  },

  RESET_DEFAULTS(state) {
    state.workRounds = defaults.workRounds
    state.timeLongBreak = defaults.timeLongBreak
    state.timeShortBreak = defaults.timeShortBreak
    state.timeWork = defaults.timeWork
  },

  SET_CURRENT_ROUND(state, payload) {
    state.currentRound = payload
  },

  SET_TICK_SOUNDS(state, payload) {
    state.tickSounds = payload
  },

  SET_TICK_SOUNDS_DURING_BREAK(state, payload) {
    state.tickSoundsDuringBreak = payload
  },

  SET_TIME_LONG_BREAK(state, payload) {
    state.timeLongBreak = payload
  },

  SET_TIME_SHORT_BREAK(state, payload) {
    state.timeShortBreak = payload
  },

  SET_TIME_WORK(state, payload) {
    state.timeWork = payload
  },

  SET_WORK_ROUNDS(state, payload) {
    state.workRounds = payload
  },

  SET_VOLUME(state, payload) {
    state.volume = payload
  },

  SET_GLOBAL_SHORTCUTS(state, shortcuts) {
    state.globalShortcuts = shortcuts
  }
}

const actions = {
  incrementRound({ commit }) {
    commit('INCREMENT_ROUND')
  },

  resetRound({ commit }) {
    commit('RESET_ROUND')
  },

  incrementTotalWorkRounds({ commit }) {
    commit('INCREMENT_TOTAL_WORK_ROUNDS')
  },

  resetDefaults({ commit }) {
    commit('RESET_DEFAULTS')
    localStore.set('workRounds', defaults.workRounds)
    localStore.set('timeLongBreak', defaults.timeLongBreak)
    localStore.set('timeShortBreak', defaults.timeShortBreak)
    localStore.set('timeWork', defaults.timeWork)
  },

  setCurrentRound({ commit }, payload) {
    commit('SET_CURRENT_ROUND', payload)
  },

  setTickSounds({ commit }, payload) {
    commit('SET_TICK_SOUNDS', payload)
    localStore.set('tickSounds', payload)
  },

  setTickSoundsDuringBreak({ commit }, payload) {
    commit('SET_TICK_SOUNDS_DURING_BREAK', payload)
    localStore.set('tickSoundsDuringBreak', payload)
  },

  setTimeLongBreak({ commit }, payload) {
    commit('SET_TIME_LONG_BREAK', payload)
    localStore.set('timeLongBreak', payload)
  },

  setTimeShortBreak({ commit }, payload) {
    commit('SET_TIME_SHORT_BREAK', payload)
    localStore.set('timeShortBreak', payload)
  },

  setTimeWork({ commit }, payload) {
    commit('SET_TIME_WORK', payload)
    localStore.set('timeWork', payload)
  },

  setWorkRounds({ commit }, payload) {
    commit('SET_WORK_ROUNDS', payload)
    localStore.set('workRounds', payload)
  },

  setVolume({ commit }, payload) {
    commit('SET_VOLUME', payload)
    localStore.set('volume', payload)
  },

  setGlobalShortcuts({ commit }, globalShortcuts) {
    commit('SET_GLOBAL_SHORTCUTS', globalShortcuts)
    localStore.set('globalShortcuts', globalShortcuts)
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
