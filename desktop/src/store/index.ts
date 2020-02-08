/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
import { StorePendingAccount } from '@/bloom/auth/models';
import * as model from '@/api/model';

Vue.use(Vuex);

interface AppState {
  is_authenticated: boolean,
  darkMode: boolean,
  pending_account?: StorePendingAccount,
  username?: string,
  displayName?: string,
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
    is_authenticated: false,
    darkMode: false,
  },
  mutations: {
    [Mutations.SIGN_IN](state: AppState, params: model.SignedIn) {
      state.is_authenticated = true;
      state.username = params.me.username;
      state.displayName = params.me.displayName;
    },
    [Mutations.SIGN_OUT](state) {
      state.is_authenticated = false;
      state.username = undefined;
      state.displayName = undefined;
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
