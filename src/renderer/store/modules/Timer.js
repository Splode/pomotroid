const state = {
  round: 1,
  workRounds: 4,
  timeLongBreak: 15,
  timeShortBreak: 1,
  timeWork: 2,
  currentRound: 'work' // work, short-break, long-break
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
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
