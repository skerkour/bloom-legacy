import * as models from '@/api/models';
import { log } from '@/libs/rz';
import { Contact } from './contacts';
import { Note } from './notes';
import { Event } from './calendar';

const { ipcRenderer } = window as any;

const Empty = {};

export type InitRes = {
  preferences: any,
  groups: models.Group[],
}

export type BlmObject<T extends Note | Contact | Event> = {
  id: string,
  createdAt: Date,
  updatedAt: Date,
  type: string,
  data: T,
  groupID: string | null,
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

export type Groups = {
  groups: models.Group[],
}


export type InitParams = {
  env: string,
  preferences: string[],
  backgroundSync: boolean,
}


async function call(method: string, params: any): Promise<any> {
  const message = {
    method,
    params,
  };
  log.with({ data: message }).debug('core.call.req');

  const data: any = await ipcRenderer.invoke('core:call', message);
  log.with({ data }).debug('core.call.res');

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

async function init(params: InitParams): Promise<InitRes> {
  return call('core.init', params);
}

export default {
  call,
  toDateIsoString,
  Empty,
  init,
};
