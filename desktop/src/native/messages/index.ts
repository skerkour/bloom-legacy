/* eslint-disable camelcase */

import {
  RegistrationStart,
  RegistrationStarted,
  RegistrationVerify,
  GuiRegistrationComplete,
  GuiSignIn,
} from './auth';

export type Message =
  | { 'type': 'auth.registration_start', 'data': RegistrationStart }
  | { 'type': 'auth.registration_started', 'data': RegistrationStarted }
  | { 'type': 'auth.registration_verify', 'data': RegistrationVerify }
  | { 'type': 'auth.gui.registration_complete', 'data': GuiRegistrationComplete }
  | { 'type': 'auth.gui.sign_in', 'data': GuiSignIn }
  | { 'type': 'auth.sign_out', 'data': {} }
  | { 'type': 'gui.to_remove.tick', 'data': { 'count': number } }
  | { 'type': 'error', 'data': { 'code': string, 'message': string, } }
