export const CONTACT_TYPE = 'com.bloom42.bloom.contact';

export enum Method {
  ListContacts = 'contacts.listContacts',
  CreateContact = 'contacts.createContact',
  UpdateContact = 'contacts.updateContact',
  DeleteContact = 'contacts.deleteContact',
}

export type Contact = {
  prefix: string,
  suffix: string,
  birthday: Date | null,
  firstName: string,
  lastName: string,
  notes: string,
  emails: ContactInformation[],
  phones: ContactInformation[],
  online: ContactInformation[],
  organizations: Organization[],
  addresses: Address[],
  deviceId: string,
  bloomUsername: string,
}

export type ContactInformation = {
  value: string,
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
  prefix: string,
  suffix: string,
  birthday: Date | null,
  firstName: string,
  lastName: string,
  notes: string,
  emails: ContactInformation[],
  phones: ContactInformation[],
  online: ContactInformation[],
  organizations: Organization[],
  addresses: Address[],
  deviceId: string,
  bloomUsername: string,
}

export type DeleteContact = {
  id: string,
}
