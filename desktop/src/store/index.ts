/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
import { StorePendingAccount } from '@/bloom/auth/models';
import * as model from '@/api/model';

Vue.use(Vuex);

interface AppState {
  isAuthenticated: boolean,
  darkMode: boolean,
  pending_account?: StorePendingAccount,
  username?: string,
  displayName?: string,
  isAdmin: boolean,
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
    isAdmin: false,
  },
  mutations: {
    [Mutations.SIGN_IN](state: AppState, params: model.SignedIn) {
      state.isAuthenticated = true;
      state.username = params.me.username;
      state.displayName = params.me.displayName;
      state.isAdmin = params.me.isAdmin;
    },
    [Mutations.SIGN_OUT](state) {
      state.isAuthenticated = false;
      state.username = undefined;
      state.displayName = undefined;
      state.isAdmin = false;
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
