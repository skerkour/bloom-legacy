import * as models from '@/api/models';

enum Method {
  StartRegistration = 'users.start_registration',
  VerifyRegistration = 'users.verify_registration',
  CompleteRegistration = 'users.complete_registration',
  SignIn = 'users.sign_in',
  SignOut = 'users.sign_out',
  FetchMyProfile = 'users.fetch_my_profile',
  FetchMySessions = 'users.fetch_my_sessions',
  RevokeSession = 'users.revoke_session',
  UpdateProfile = 'users.update_profile',
}

export { Method };


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
