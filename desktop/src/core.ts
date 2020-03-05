// import axios from 'axios';
import { ipcRenderer } from 'electron';
import * as models from '@/api/models';

// const { log } = require('@bloom42/astro');

// const CALL_URL = 'unix:/tmp/com.bloom42.bloom.sock:/electronCall';
// const UNIX_SOCKET_PATH = '/tmp/com.bloom42.bloom.sock';

const Empty = {};

async function call(method: string, params: any): Promise<any> {
  return ipcRenderer.send('core:call', method, params);
  // const message = JSON.stringify({
  //   method,
  //   params,
  // });
  // log.with({ msg: message }).debug('jsonMessage');

  // // const coreClient = axios.create({
  // //   url: CALL_URL,
  // //   socketPath: UNIX_SOCKET_PATH,
  // // });

  // const res = await axios({
  //   url: CALL_URL,
  //   method: 'post',
  //   data: message,
  //   socketPath: UNIX_SOCKET_PATH,
  // });
  // log.with({ res: res.data }).debug('resMessage');

  // const { data } = res;
  // if (data.error !== null) {
  //   throw data.error;
  // }

  // return data.data;
}

function toIsoDate(date: string | null): Date | null {
  if (date === null) {
    return null;
  }
  return new Date(date).toISOString() as unknown as Date;
}

async function init(): Promise<models.SignedIn | null> {
  return call('core.init', Empty);
}

export default {
  call,
  toIsoDate,
  Empty,
  init,
};
