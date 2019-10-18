/* eslint-disable camelcase */

export interface Event {
  id: string,
  title: string,
  description: string,
  created_at: Date,
  updated_at: Date,
  start_at: Date,
  end_at: Date,
}

export interface GuiEvents {
  events: Event[],
}

export interface GuiEvent {
  event: Event,
}

export interface GuiCreateEvent {
  title: string,
  description: string,
  start_at: Date,
  end_at: Date,
}

export interface GuiDeleteEvent {
  id: string,
}
