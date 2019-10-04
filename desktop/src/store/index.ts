/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
// import { StorePendingAccount } from "@/bloom/auth/StorePendingAccount";

Vue.use(Vuex);

interface AppState {
  is_authenticated: boolean,
  dark_mode: boolean,
}

export default new Vuex.Store<AppState>({
  state: {
    is_authenticated: false,
    dark_mode: false,
  },
  mutations: {
    sign_in(state) {
      state.is_authenticated = true;
    },
    sign_out(state) {
      state.is_authenticated = false;
    },
  },
  actions: {

  },
});
