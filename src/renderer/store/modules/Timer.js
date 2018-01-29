const state = {
  round: 1,
  workRounds: 4,
  timeLongBreak: 15,
  timeShortBreak: 1,
  timeWork: 1,
  currentRound: 'work', // work, short-break, long-break
  isMuted: false
}

const getters = {
  round () {
    return state.round
  },
  workRounds () {
    return state.workRounds
  },
  timeLongBreak () {
    return state.timeLongBreak
  },
  timeShortBreak () {
    return state.timeShortBreak
  },
  timeWork () {
    return state.timeWork
  },
  currentRound () {
    return state.currentRound
  },
  isMuted () {
    return state.isMuted
  }
}

const mutations = {
  INCREMENT_ROUND (state) {
    state.round += 1
  },

  RESET_ROUND (state) {
    state.round = 1
  },

  SET_CURRENT_ROUND (state, payload) {
    state.currentRound = payload
  },

  TOGGLE_MUTE (state) {
    state.isMuted = !state.isMuted
  }
}

const actions = {
  incrementRound ({ commit }) {
    commit('INCREMENT_ROUND')
  },

  resetRound ({ commit }) {
    commit('RESET_ROUND')
  },

  setCurrentRound ({ commit }, payload) {
    commit('SET_CURRENT_ROUND', payload)
  },

  toggleMute ({ commit }) {
    commit('TOGGLE_MUTE')
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
