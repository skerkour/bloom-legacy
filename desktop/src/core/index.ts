import * as models from '@/api/models';
import { log } from '@/libs/rz';
import { Contact } from './contacts';
import { Note } from './notes';
import { Event } from './calendar';

const { ipcRenderer } = window as any;

const Empty = {};

export type InitRes = {
  signedIn: models.SignedIn | null,
  preferences: any,
}

export type BlmObject<T extends Note | Contact | Event> = {
  id: string,
  createdAt: Date,
  updatedAt: Date,
  type: string,
  data: T,
  groupId: string | null,
}

export type Notes = {
  notes: BlmObject<Note>[],
}

export type Contacts = {
  contacts: BlmObject<Contact>[],
}

export type Events = {
  events: BlmObject<Event>[],
}


async function call(method: string, params: any): Promise<any> {
  const message = {
    method,
    params,
  };
  log.with({ data: message }).debug('code.call.req');

  const data: any = await ipcRenderer.invoke('core:call', message);
  log.with({ data }).debug('core.call.res');

  // const { data } = res;
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
