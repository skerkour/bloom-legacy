export const CONTACT_TYPE = 'com.bloom42.bloom.contact';

export enum Method {
  ListContacts = 'contacts.listContacts',
  CreateContact = 'contacts.createContact',
  UpdateContact = 'contacts.updateContact',
  DeleteContact = 'contacts.deleteContact',
}

export type Contact = {
  birthday: Date | null,
  firstName: string,
  lastName: string,
  notes: string,
  emails: Email[],
  phones: Phone[],
  websites: Website[],
  organizations: Organization[],
  addresses: Address[],
  deviceId: string,
  bloomUsername: string,
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
  postalCode: string,
  street: string,
  state: string,
}

export type CreateContactParams = {
  birthday: Date | null,
  firstName: string,
  lastName: string,
  notes: string,
  emails: Email[],
  phones: Phone[],
  websites: Website[],
  organizations: Organization[],
  addresses: Address[],
  deviceId: string,
  bloomUsername: string,
}

export type DeleteContact = {
  id: string,
}
