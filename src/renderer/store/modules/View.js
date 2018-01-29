const state = {
  drawerOpen: false
}

const getters = {
  drawerOpen () {
    return state.drawerOpen
  }
}

const mutations = {
  TOGGLE_DRAWER (state) {
    state.drawerOpen = !state.drawerOpen
  }
}

const actions = {
  toggleDrawer ({ commit }) {
    commit('TOGGLE_DRAWER')
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
