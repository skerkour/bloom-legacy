/* eslint-disable camelcase */

export interface GuiRegistrationStart {
  email: string,
  display_name: string,
  password: string,
}

export interface RegistrationStarted {
  id: string,
}

export interface RegistrationVerify {
  id: string,
  code: string,
}

export interface RegistrationComplete{
  id: string,
  username: string,
}
