/* eslint-disable camelcase */
import Vue from 'vue';
import Vuex from 'vuex';
import { StorePendingAccount } from '@/ui/auth/models';
import * as models from '@/api/models';

Vue.use(Vuex);

interface AppState {
  darkMode: boolean,
  pendingAccount?: StorePendingAccount,
  me: models.User | null,
  session: models.Session | null,
}

export enum Mutations {
  SIGN_IN,
  SIGN_OUT,
  SET_PENDING_ACCOUNT,
  CLEAR_PENDING_ACCOUNT,
  SWITCH_DARK_MODE,
  UPDATE_DISPLAY_NAME,
}

export default new Vuex.Store<AppState>({
  state: {
    darkMode: false,
    me: null,
    session: null,
  },
  mutations: {
    [Mutations.SIGN_IN](state: AppState, params: models.SignedIn) {
      state.session = params.session;
      state.me = params.me;
    },
    [Mutations.SIGN_OUT](state) {
      state.session = null;
      state.me = null;
    },
    [Mutations.SET_PENDING_ACCOUNT](state: AppState, pendginAccount: StorePendingAccount) {
      state.pendingAccount = pendginAccount;
    },
    [Mutations.CLEAR_PENDING_ACCOUNT](state: AppState) {
      state.pendingAccount = undefined;
    },
    [Mutations.SWITCH_DARK_MODE](state: AppState) {
      state.darkMode = !state.darkMode;
    },
    [Mutations.UPDATE_DISPLAY_NAME](state: AppState, displayName: string) {
      state.me!.displayName = displayName;
    },
  },
  actions: {

  },
});
