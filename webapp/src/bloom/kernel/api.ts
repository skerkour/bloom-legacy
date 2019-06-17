import axios from 'axios';
import { AxiosInstance, AxiosRequestConfig } from 'axios';
import config from '@/config';
const { log } = require('@bloom42/astro');
const store = require('store');

import vueStore from '@/store';
import router from './router';

class API {
  // do not use axios' baseUrl because if we need in the future to move APIs to subdomains
  api_url = config.API_BASE_URL ? config.API_BASE_URL : `${config.HOST}/api`;
  ACCOUNT_GRAPHQL = `${this.api_url}/account/graphql`;
  MYACCOUNT = `${this.api_url}/myaccount`;
  NOTES = `${this.api_url}/notes`;
  CONTACTS = `${this.api_url}/contacts`;
  GALLERY = `${this.api_url}/gallery`;
  DRIVE = `${this.api_url}/drive`;
  MUSIC = `${this.api_url}/music`;
  BITFLOW = `${this.api_url}/bitflow`;
  DRIVE_UPLOAD = `${this.api_url}/drive/v1/upload`;
  PHASER_GRAPHQL = `${this.api_url}/phaser/graphql`;
  PHASER = `${this.api_url}/phaser`;
  ADMIN = `${this.api_url}/admin`;
  SESSION_KEY = '__bloom_session';
  DARK_MODE_KEY = '__bloom_dark_mode';
  private static get MESSAGE_SESSION_EXPIRED() { return 'session expired'; }
  private MESSAGE_SESSION_NOT_VALID = 'ession is not valid';

  private _client: AxiosInstance = axios;

  init() {
    this.set_empty_client();

    const session = store.get(this.SESSION_KEY);
    vueStore.commit('set_session', session);

    const dark_mode = store.get(this.DARK_MODE_KEY);
    vueStore.commit('set_dark_mode', dark_mode);
    if (session) {
      this.set_auth_header(vueStore.state.session!.token);
      this.fetch_me();
    }
  }

  async request(req_config: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.request(req_config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      if (err.response && err.response.data && err.response.data.error) {
        const err_msg = err.response.data.error.message;
        if (err.response.status === 401
          && err.response.data.error.message.includes(this.MESSAGE_SESSION_NOT_VALID)) {
            this.sign_out();
        }
        throw new Error(err_msg);
      }
      throw err;
    }
  }

  async get(url: string, req_config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.get(url, req_config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      if (err.response && err.response.data && err.response.data.error) {
        const err_msg = err.response.data.error.message;
        if (err.response.status === 401
          && err.response.data.error.message.includes(this.MESSAGE_SESSION_NOT_VALID)) {
            this.sign_out();
        }
        throw new Error(err_msg);
      }
      throw err;
    }
  }

  async delete(url: string, req_config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.delete(url, req_config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      if (err.response && err.response.data && err.response.data.error) {
        const err_msg = err.response.data.error.message;
        if (err.response.status === 401
          && err.response.data.error.message.includes(this.MESSAGE_SESSION_NOT_VALID)) {
            this.sign_out();
        }
        throw new Error(err_msg);
      }
      throw err;
    }
  }

  async put(url: string, data?: any, req_config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.put(url, data, req_config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      if (err.response && err.response.data && err.response.data.error) {
        const err_msg = err.response.data.error.message;
        if (err.response.status === 401
          && err.response.data.error.message.includes(this.MESSAGE_SESSION_NOT_VALID)) {
            this.sign_out();
        }
        throw new Error(err_msg);
      }
      throw err;
    }
  }

  async post(url: string, data?: any, req_config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.post(url, data, req_config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      if (err.response && err.response.data && err.response.data.error) {
        const err_msg = err.response.data.error.message;
        if (err.response.status === 401
          && err.response.data.error.message.includes(this.MESSAGE_SESSION_NOT_VALID)) {
            this.sign_out();
        }
        throw new Error(err_msg);
      }
      throw err;
    }
  }

  async query(route: string, query: string, variables?: any) {
    let res = null;
    try {
      res = await this._client.post(route, {
        operationName: null,
        query,
        variables,
      });
    } catch (err) {
      if (err.response) {
        res = err.response;
      } else {
        throw err;
      }
    }
    if (res && res.data.errors && res.data.errors.length >= 1) {
      const err = res.data.errors[0].message;
      if (err.toString().indexOf(API.MESSAGE_SESSION_EXPIRED) !== -1) {
        // Notification.error({
        //   title: 'Error',
        //   message: SESSION_EXPIRED_ERROR_MESSAGE,
        //   duration: 0,
        // });
        this.end_session();
      }
      log.error(err);
      throw new Error(err);
    } else if (res && res.data.error) {
      const err = res.data.error;
      log.error(err);
      throw new Error(err);
    }
    log.with({ data: res.data.data }).debug();
    return res.data.data;
  }

  async mutate(route: string, mutation: string, variables?: any) {
    let res = null;
    try {
      res = await this._client.post(route, {
        operationName: null,
        query: mutation,
        variables,
      });
    } catch (err) {
      if (err.response) {
        res = err.response;
      } else {
        throw err;
      }
    }
    if (res && res.data.errors && res.data.errors.length >= 1) {
      const err = res.data.errors[0].message;
      if (err.toString().indexOf(API.MESSAGE_SESSION_EXPIRED) !== -1) {
        // Notification.error({
        //   title: 'Error',
        //   message: SESSION_EXPIRED_ERROR_MESSAGE,
        //   duration: 0,
        // });
        this.end_session();
      }
      log.error(err);
      throw new Error(err);
    } else if (res && res.data.error) {
      const err = res.data.error;
      log.error(err);
      throw new Error(err);
    }
    log.with({ data: res.data.data }).debug();
    return res.data.data;
  }

  sign_in(session: any) {
    store.set(this.SESSION_KEY, session);
    vueStore.commit('set_session', session);
    this.set_auth_header(session.token);

    this.fetch_me();
  }

  async sign_out() {
    try {
      await this.post(`${this.MYACCOUNT}/v1/sign-out`);
    } catch (err) {
      log.error(err);
    } finally {
      this.end_session();
    }
  }

  is_authenticated() {
    return vueStore.state.session !== null && vueStore.state.session !== undefined;
  }

  async fetch_me() {
    try {
      const res = await this.get(`${this.MYACCOUNT}/v1/me`);
      vueStore.commit('set_account', res);
    } catch (err) {
      log.error(err);
    }
  }

  store_dark_mode(dark_mode: boolean) {
    store.set(this.DARK_MODE_KEY, dark_mode);
    vueStore.commit('set_dark_mode', dark_mode);
  }

  client(): AxiosInstance {
    return this._client;
  }

  private set_empty_client() {
    this._client = axios.create();
    delete this._client.defaults.headers.common.Authorization;
  }

  private set_auth_header(token: string) {
    this._client.defaults.headers.common.Authorization = `Basic ${token}`;
  }

  private end_session() {
    store.remove(this.SESSION_KEY);
    vueStore.commit('set_session', null);
    this.set_empty_client();
    router.push({ path: '/' });
  }
}

export default new API();
