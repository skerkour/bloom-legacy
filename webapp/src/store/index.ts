import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);


interface AppState {
  session: any;
  pending_account: any;
  account: any;
  drive_profile: any;
  search: string;
  dark_mode: boolean;
}

/* tslint:disable:object-literal-sort-keys */
export default new Vuex.Store<AppState>({
  strict: process.env.NODE_ENV !== 'production',
  state: {
    account: null,
    drive_profile: null,
    pending_account: null,
    search: '',
    session: null,
    dark_mode: false,
  },
  mutations: {
    set_drive_profile(state, profile) {
      state.drive_profile = profile;
    },
    set_session(state, session) {
      state.session = session;
    },
    set_pending_account(state, pending_account) {
      state.pending_account = pending_account;
    },
    set_search(state, search) {
      if (!search) {
        search = '';
      }
      state.search = search;
    },
    set_account(state: any, account) {
      if (state.account === null) {
        state.account = {};
      }
      if (account.first_name) {
        state.account.first_name = account.first_name;
      }
      if (account.last_name) {
        state.account.last_name = account.last_name;
      }
      if (account.username) {
        state.account.username = account.username;
      }
      if (account.email) {
        state.account.email = account.email;
      }
      if (account.avatar_url) {
        state.account.avatar_url = account.avatar_url;
      }
      if (account.is_admin !== undefined && account.is_admin !== null) {
        state.account.is_admin = account.is_admin;
      }
    },
    set_dark_mode(state, dark_mode: boolean) {
      state.dark_mode = dark_mode;
    },
  },
  actions: {

  },
});
