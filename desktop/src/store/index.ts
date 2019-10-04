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

export enum Mutations {
  SIGN_IN,
  SIGN_OUT,
  SET_PENDING_ACCOUNT,
  CLEAR_PENDING_ACCOUNT,
}

export default new Vuex.Store<AppState>({
  state: {
    is_authenticated: false,
    dark_mode: false,
    pending_account: null,
  },
  mutations: {
    [Mutations.SIGN_IN](state) {
      state.is_authenticated = true;
    },
    [Mutations.SIGN_OUT](state) {
      state.is_authenticated = false;
    },
    [Mutations.SET_PENDING_ACCOUNT](state: AppState, pendginAccount: StorePendingAccount) {
      state.pending_account = pendginAccount;
    },
    [Mutations.CLEAR_PENDING_ACCOUNT](state: AppState) {
      state.pending_account = null;
    },
  },
  actions: {

  },
});
