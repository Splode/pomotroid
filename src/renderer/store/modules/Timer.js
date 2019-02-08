import { localStore } from './index'
import { defaults } from './../../utils/local-store'

const state = {
  round: 1,
  workRounds: parseInt(localStore.get('workRounds')),
  timeLongBreak: parseInt(localStore.get('timeLongBreak')),
  timeShortBreak: parseInt(localStore.get('timeShortBreak')),
  timeWork: parseInt(localStore.get('timeWork')),
  currentRound: 'work', // work, short-break, long-break
  volume: localStore.get('volume') || 100
}

const getters = {
  round() {
    return state.round
  },
  workRounds() {
    return state.workRounds
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
  }
}

const mutations = {
  INCREMENT_ROUND(state) {
    state.round += 1
  },

  RESET_ROUND(state) {
    state.round = 1
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
  }
}

const actions = {
  incrementRound({ commit }) {
    commit('INCREMENT_ROUND')
  },

  resetRound({ commit }) {
    commit('RESET_ROUND')
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
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
