import * as models from '@/api/models';

export enum Method {
  StartRegistration = 'users.startRegistration',
  VerifyRegistration = 'users.verifyRegistration',
  CompleteRegistration = 'users.completeRegistration',
  SignIn = 'users.signIn',
  SignOut = 'users.signOut',
  FetchMyProfile = 'users.fetchMyProfile',
  FetchMySessions = 'users.fetchMySessions',
  RevokeSession = 'users.revokeSession',
  UpdateProfile = 'users.updateProfile',
}

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

export type RevokeSessionParams = {
  id: string,
};

export type Message = SignIn
 | CompleteRegistration
 | VerifyRegistration
 | models.RegistrationStarted
 | boolean
 | models.SignedIn
 | StartRegistration
 | RevokeSessionParams;
