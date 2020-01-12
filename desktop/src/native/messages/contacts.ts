/* eslint-disable camelcase */

export interface Contact {
  id: string,
  created_at: Date,
  updated_at: Date,
  birthday: Date | null,
  first_name: string,
  last_name: string,
  notes: string,
  emails: Email[],
  phones: Phone[],
  websites: Website[],
  organizations: Organization[],
  addresses: Address[],
  device_id: string,
}

export interface Email {
  email: string,
  label: string,
}

export interface Phone {
  phone: string,
  label: string,
}

export interface Website {
  website: string,
  label: string,
}

export interface Organization {
  name: string,
  title: string,
}

export interface Address {
  city: string,
  country: string,
  label: string,
  postal_code: string,
  street_address: string,
  street_address2: string,
}

export interface GuiContact {
  contact: Contact,
}

export interface GuiContacts {
  contacts: Contact[],
}

export interface GuiCreateContact {
  birthday: Date | null,
  first_name: string,
  last_name: string,
  notes: string,
  emails: Email[],
  phones: Phone[],
  websites: Website[],
  organizations: Organization[],
  addresses: Address[],
  device_id: string,
}

export interface GuiDeleteContact {
  id: string,
}
