/* eslint-disable camelcase */

export enum Method {
  ListContacts = 'contacts.list_contacts',
  CreateContact = 'contacts.create_contact',
  UpdateContact = 'contacts.update_contact',
  DeleteContact = 'contacts.delete_contact',
}

export type Contact = {
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

export type Email = {
  email: string,
  label: string,
}

export type Phone = {
  phone: string,
  label: string,
}

export type Website = {
  website: string,
  label: string,
}

export type Organization = {
  name: string,
  title: string,
}

export type Address = {
  city: string,
  country: string,
  label: string,
  postal_code: string,
  street_address: string,
  street_address2: string,
}

export type Contacts = {
  contacts: Contact[],
}

export type CreateContactParams = {
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

export type DeleteContact = {
  id: string,
}
