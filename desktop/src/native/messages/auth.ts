/* eslint-disable camelcase */

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
