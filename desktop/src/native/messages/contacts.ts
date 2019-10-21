/* eslint-disable camelcase */

export interface Contact {
  id: string,
  created_at: Date,
  updated_at: Date,
}

export interface GuiContact {
  contact: Contact,
}

export interface GuiContacts {
  contacts: Contact[],
}
