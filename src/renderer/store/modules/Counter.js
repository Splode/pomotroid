const state = {
  round: 1
}

const mutations = {
  INCREMENT_ROUND_COUNTER (state) {
    state.round += 1
  }
}

const actions = {
  someAsyncTask ({ commit }) {
    // do something async
    commit('INCREMENT_ROUND_COUNTER')
  }
}

export default {
  state,
  mutations,
  actions
}
