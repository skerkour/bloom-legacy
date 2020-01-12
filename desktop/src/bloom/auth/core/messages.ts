/* eslint-disable camelcase, import/prefer-default-export */

export type StartRegistration = {
  email: string,
  display_name: string,
};

export type RegistrationStarted = {
  id: string,
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
 | RegistrationStarted
 | StartRegistration;
