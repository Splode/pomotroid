import { localStore } from './index'

const state = {
  currentDrawer: 'appDrawerTimer',
  drawerOpen: false,
  autoStartWorkTimer:
    localStore.get('autoStartWorkTimer') === undefined
      ? true
      : localStore.get('autoStartWorkTimer'),
  autoStartBreakTimer:
    localStore.get('autoStartBreakTimer') === undefined
      ? true
      : localStore.get('autoStartBreakTimer'),
  alwaysOnTop: localStore.get('alwaysOnTop'),
  breakAlwaysOnTop: localStore.get('breakAlwaysOnTop'),
  minToTray: localStore.get('minToTray'),
  minToTrayOnClose: localStore.get('minToTrayOnClose'),
  notifications: localStore.get('notifications'),
  os: process.platform,
  theme: localStore.get('theme') || 'Pomotroid'
}

const getters = {
  autoStartWorkTimer() {
    return state.autoStartWorkTimer
  },

  autoStartBreakTimer() {
    return state.autoStartBreakTimer
  },

  currentDrawer() {
    return state.currentDrawer
  },

  drawerOpen() {
    return state.drawerOpen
  },

  alwaysOnTop() {
    return state.alwaysOnTop
  },

  breakAlwaysOnTop() {
    return state.breakAlwaysOnTop
  },

  minToTray() {
    return state.minToTray
  },

  minToTrayOnClose() {
    return state.minToTrayOnClose
  },

  notifications() {
    return state.notifications
  },

  os() {
    return state.os
  },

  theme() {
    return state.theme
  }
}

const mutations = {
  SET_SETTING(state, payload) {
    localStore.set(payload.key, payload.val)
  },

  SET_VIEW_STATE(state, payload) {
    state[payload.key] = payload.val
  },

  TOGGLE_DRAWER(state) {
    state.drawerOpen = !state.drawerOpen
  }
}

const actions = {
  setSetting({ commit }, payload) {
    commit('SET_SETTING', payload)
  },

  setViewState({ commit }, payload) {
    commit('SET_VIEW_STATE', payload)
  },

  toggleDrawer({ commit }) {
    commit('TOGGLE_DRAWER')
  }
}

export default {
  state,
  getters,
  mutations,
  actions
}
