/* eslint-disable camelcase */

import {
  GuiRegistrationStart,
  RegistrationStarted,
  RegistrationVerify,
  RegistrationComplete,
} from './auth';

export type Message =
  | { 'type': 'auth.gui.registration_start', 'data': GuiRegistrationStart }
  | { 'type': 'auth.registration_started', 'data': RegistrationStarted }
  | { 'type': 'auth.registration_verify', 'data': RegistrationVerify }
  | { 'type': 'auth.registration_complete', 'data': RegistrationComplete }
  | { 'type': 'gui.to_remove.tick', 'data': { 'count': number } }
  | { 'type': 'error', 'data': { 'code': string, 'message': string, } }
