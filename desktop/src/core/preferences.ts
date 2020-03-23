/* eslint-disable import/prefer-default-export */

export enum Method {
  Set = 'preferences.set',
  Get = 'preferences.get',
  Delete = 'preferences.delete',
}

export type SetParams = {
  key: string,
  value: string,
}

export type GetParams = {
  key: string,
}

export type DeleteParams = {
  key: string,
}
