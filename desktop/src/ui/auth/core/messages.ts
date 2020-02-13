/* eslint-disable camelcase, import/prefer-default-export */

import * as models from '@/api/models';

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
 | models.RegistrationStarted
 | boolean
 | models.SignedIn
 | StartRegistration;
