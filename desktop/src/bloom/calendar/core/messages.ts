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
  start_at: Date | null,
  end_at: Date | null,
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
