enum Method {
  ListEvents = 'calendar.list_events',
  CreateEvent = 'calendar.create_event',
  UpdateEvent = 'calendar.update_event',
  DeleteEvent = 'calendar.delete_event',
}

export { Method };

/* eslint-disable camelcase */

export type Event = {
  id: string,
  title: string,
  description: string,
  created_at: Date,
  updated_at: Date,
  start_at: Date,
  end_at: Date,
}

export type Events = {
  events: Event[],
}

export type ListEvents = {
  start_at: Date | undefined,
  end_at: Date | undefined,
}

export type CreateEvent = {
  title: string,
  description: string,
  start_at: Date,
  end_at: Date,
}

export type DeleteEvent = {
  id: string,
}
