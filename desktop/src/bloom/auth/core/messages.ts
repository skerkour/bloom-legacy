/* eslint-disable camelcase, import/prefer-default-export */

import * as model from '@/api/model';

export type StartRegistration = {
  email: string,
  displayName: string,
};

export type VerifyRegistration = {
  id: string,
  code: string,
};

export type CompleteRegistration = {
  id: string,
  username: string,
  password: string,
};

export type SignIn = {
  username: string,
  password: string,
};

export type Message = SignIn
 | CompleteRegistration
 | VerifyRegistration
 | model.RegistrationStarted
 | boolean
 | model.SignedIn
 | StartRegistration;
