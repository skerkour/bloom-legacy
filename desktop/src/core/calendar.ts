export const CALENDAR_EVENT_TYPE = 'com.bloom42.bloom.calendar_event';

export enum Method {
  FindEvents = 'calendar.findEvents',
  CreateEvent = 'calendar.createEvent',
  UpdateEvent = 'calendar.updateEvent',
  DeleteEvent = 'calendar.deleteEvent',
}


export type Event = {
  title: string,
  location: string,
  description: string,
  status: string,
  startAt: Date,
  endAt: Date,
}
