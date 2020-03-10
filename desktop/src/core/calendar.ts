export enum Method {
  ListEvents = 'calendar.listEvents',
  CreateEvent = 'calendar.createEvent',
  UpdateEvent = 'calendar.updateEvent',
  DeleteEvent = 'calendar.deleteEvent',
}


export type Event = {
  id: string,
  title: string,
  description: string,
  createdAt: Date,
  updatedAt: Date,
  startAt: Date,
  endAt: Date,
}

export type Events = {
  events: Event[],
}

export type ListEvents = {
  startAt: Date | undefined,
  endAt: Date | undefined,
}

export type CreateEvent = {
  title: string,
  description: string,
  startAt: Date,
  endAt: Date,
}

export type DeleteEvent = {
  id: string,
}
