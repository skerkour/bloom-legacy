/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
import { StorePendingAccount } from '@/bloom/auth/models';
import * as models from '@/api/models';

Vue.use(Vuex);

interface AppState {
  isAuthenticated: boolean,
  darkMode: boolean,
  pending_account?: StorePendingAccount,
  me: models.User | null,
}

export enum Mutations {
  SIGN_IN,
  SIGN_OUT,
  SET_PENDING_ACCOUNT,
  CLEAR_PENDING_ACCOUNT,
  SWITCH_DARK_MODE,
}

export default new Vuex.Store<AppState>({
  state: {
    isAuthenticated: false,
    darkMode: false,
    me: null,
  },
  mutations: {
    [Mutations.SIGN_IN](state: AppState, params: models.SignedIn) {
      state.isAuthenticated = true;
      state.me = params.me;
    },
    [Mutations.SIGN_OUT](state) {
      state.isAuthenticated = false;
      state.me = null;
    },
    [Mutations.SET_PENDING_ACCOUNT](state: AppState, pendginAccount: StorePendingAccount) {
      state.pending_account = pendginAccount;
    },
    [Mutations.CLEAR_PENDING_ACCOUNT](state: AppState) {
      state.pending_account = undefined;
    },
    [Mutations.SWITCH_DARK_MODE](state: AppState) {
      state.darkMode = !state.darkMode;
    },
  },
  actions: {

  },
});
