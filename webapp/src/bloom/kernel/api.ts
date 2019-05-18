import axios from 'axios';
import { AxiosInstance, AxiosRequestConfig } from 'axios';
const { log } = require('@bloom42/astro');
const store = require('store');

import vueStore from '@/store';
import router from './router';

class API {
  // do not use axios' baseUrl because if we need in the future to move APIs to subdomains
  api_url = process.env.VUE_APP_API_BASE_URL ?
      process.env.VUE_APP_API_BASE_URL
      : `https://api.${process.env.VUE_APP_ROOT_DOMAIN}`;
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
  SESSION_KEY = '__bloom_session';
  private static get MESSAGE_SESSION_EXPIRED() { return 'session expired'; }

  private _client: AxiosInstance = axios;

  init() {
    this.set_empty_client();

    const session = store.get(this.SESSION_KEY);
    vueStore.commit('set_session', session);
    if (session) {
      this.set_auth_header(vueStore.state.session!.token);
      this.fetch_me();
    }
  }

  async request(config: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.request(config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      throw err;
    }
  }

  async get(url: string, config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.get(url, config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      throw err;
    }
  }

  async delete(url: string, config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.delete(url, config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      throw err;
    }
  }

  async put(url: string, data?: any, config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.put(url, data, config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
      throw err;
    }
  }

  async post(url: string, data?: any, config?: AxiosRequestConfig): Promise<any> {
    try {
      const res = await this._client.post(url, data, config);
      log.with({ data: res.data.data }).debug();
      return res.data.data;
    } catch (err) {
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
