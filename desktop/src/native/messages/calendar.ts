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
  notes: Event[],
}
