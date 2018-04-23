import { createLocalStore } from '@/utils/local-store'

const localStore = createLocalStore()

const state = {
  currentDrawer: 'appDrawerTimer',
  drawerOpen: false,
  autoStartTimer: localStore.get('autoStartTimer') === undefined
    ? true
    : localStore.get('autoStartTimer'),
  alwaysOnTop: localStore.get('alwaysOnTop'),
  notifications: localStore.get('notifications')
}

const getters = {
  autoStartTimer () {
    return state.autoStartTimer
  },

  currentDrawer () {
    return state.currentDrawer
  },

  drawerOpen () {
    return state.drawerOpen
  },

  alwaysOnTop () {
    return state.alwaysOnTop
  },

  notifications () {
    return state.notifications
  }
}

const mutations = {
  SET_SETTING (state, payload) {
    localStore.set(payload.key, payload.val)
  },

  SET_VIEW_STATE (state, payload) {
    state[payload.key] = payload.val
  },

  TOGGLE_DRAWER (state) {
    state.drawerOpen = !state.drawerOpen
  }
}

const actions = {
  setSetting ({ commit }, payload) {
    commit('SET_SETTING', payload)
  },

  setViewState ({ commit }, payload) {
    commit('SET_VIEW_STATE', payload)
  },

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
