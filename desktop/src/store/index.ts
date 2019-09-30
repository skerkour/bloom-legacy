/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
import { StorePendingAccount } from '@/bloom/auth/models';

Vue.use(Vuex);

interface AppState {
  is_authenticated: boolean,
  dark_mode: boolean,
  pending_account: StorePendingAccount | null,
}

export default new Vuex.Store<AppState>({
  state: {
    is_authenticated: false,
    dark_mode: false,
    pending_account: null,
  },
  mutations: {
    sign_in(state) {
      state.is_authenticated = true;
    },
    sign_out(state) {
      state.is_authenticated = false;
    },
    set_pending_account(state: AppState, pendginAccount: StorePendingAccount) {
      state.pending_account = pendginAccount;
    },
  },
  actions: {

  },
});
