import * as models from '@/api/models';
import { log } from '@/libs/rz';

const { ipcRenderer } = window as any;

const Empty = {};

export type InitRes = {
  signedIn: models.SignedIn | null,
  preferences: any,
}

export type BlmObject = {
  id: string,
  createdAt: Date,
  updatedAt: Date,
  type: string,
  data: any,
  groupId: string | null,
}

async function call(method: string, params: any): Promise<any> {
  const message = JSON.stringify({
    method,
    params,
  });
  log.with({ msg: message }).debug('jsonMessage');

  const res: any = await ipcRenderer.invoke('core:call', message);
  log.with({ res: res.data }).debug('resMessage');

  const { data } = res;
  if (data.error !== null) {
    throw data.error;
  }

  return data.data;
}

function toDateIsoString(date: string | null): Date | null {
  if (date === null) {
    return null;
  }
  return new Date(date).toISOString() as unknown as Date;
}

async function init(preferences: string[]): Promise<InitRes> {
  return call('core.init', { preferences });
}

export default {
  call,
  toDateIsoString,
  Empty,
  init,
};
