/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

interface AppState {
  is_authenticated: boolean;
}

export default new Vuex.Store<AppState>({
  state: {
    is_authenticated: false,
  },
  mutations: {
    signIn(state) {
      state.is_authenticated = true;
    },
    signOut(state) {
      state.is_authenticated = false;
    },
  },
  actions: {

  },
});
