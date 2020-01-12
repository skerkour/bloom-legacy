/* eslint-disable camelcase, import/prefer-default-export */

export interface RegistrationStart {
  email: string,
  display_name: string,
}

export interface RegistrationStarted {
  id: string,
}

export interface RegistrationVerify {
  id: string,
  code: string,
}

export interface GuiRegistrationComplete {
  id: string,
  username: string,
  password: string,
}

export interface GuiSignIn {
  username: string,
  password: string,
}

export type Message = GuiSignIn
 | GuiRegistrationComplete
 | RegistrationVerify
 | RegistrationStarted
 | RegistrationStart;


export enum Method {
  StartRegistration = 'auth.registration_start',
  // | 'auth.registration_started'
  VerifyRegistration = 'auth.registration_verify',
  CompleteRegistration = 'auth.gui.registration_complete',
  SignIn = 'auth.gui.sign_in',
  SignOut = 'auth.sign_out',
}
