/* eslint-disable camelcase */

import {
  RegistrationStart,
  RegistrationStarted,
  RegistrationVerify,
  GuiRegistrationComplete,
  GuiSignIn,
} from './auth';

import {
  GuiNote,
  GuiNotes,
  GuiCreateNote,
  GuiDeleteNote,
} from './notes';

import {
  GuiExpression,
  GuiResult,
} from './calculator';

import {
  GuiEvents,
  GuiEvent,
  GuiDeleteEvent,
  GuiCreateEvent,
  ListEvents,
} from './calendar';

import { GuiContact, GuiContacts } from './contacts';

export type Message =
  // auth
  | { 'type': 'auth.registration_start', 'data': RegistrationStart }
  | { 'type': 'auth.registration_started', 'data': RegistrationStarted }
  | { 'type': 'auth.registration_verify', 'data': RegistrationVerify }
  | { 'type': 'auth.gui.registration_complete', 'data': GuiRegistrationComplete }
  | { 'type': 'auth.gui.sign_in', 'data': GuiSignIn }
  | { 'type': 'auth.sign_out', 'data': {} }

  // notes
  | { 'type': 'notes.gui.list_notes', 'data': {} }
  | { 'type': 'notes.gui.get_archive', 'data': {} }
  | { 'type': 'notes.gui.notes', 'data': GuiNotes }
  | { 'type': 'notes.gui.note', 'data': GuiNote }
  | { 'type': 'notes.gui.create_note', 'data': GuiCreateNote }
  | { 'type': 'notes.gui.update_note', 'data': GuiNote }
  | { 'type': 'notes.gui.delete_note', 'data': GuiDeleteNote }

  // calculator
  | { 'type': 'calculator.gui.expression', 'data': GuiExpression }
  | { 'type': 'calculator.gui.result', 'data': GuiResult }

  // calendar
  | { 'type': 'calendar.gui.list_events', 'data': ListEvents }
  | { 'type': 'calendar.gui.events', 'data': GuiEvents }
  | { 'type': 'calendar.gui.event', 'data': GuiEvent }
  | { 'type': 'calendar.gui.create_event', 'data': GuiCreateEvent }
  | { 'type': 'calendar.gui.delete_event', 'data': GuiDeleteEvent }
  | { 'type': 'calendar.gui.update_event', 'data': GuiEvent }

  // contacts
  | { 'type': 'contacts.gui.list_contacts', 'data': {} }
  | { 'type': 'contacts.gui.contact', 'data': GuiContact }
  | { 'type': 'contacts.gui.contacts', 'data': GuiContacts }

  | { 'type': 'gui.to_remove.tick', 'data': { 'count': number } }
  | { 'type': 'error', 'data': { 'code': string, 'message': string, } }
