/* eslint-disable camelcase */

export type Note = {
  id: string,
  title: string,
  body: string,
  created_at: Date,
  updated_at: Date,
  color: string,
  archived_at: Date | null,
  is_pinned: boolean,
}

export type Notes = {
  notes: Note[],
}

export type CreateNote = {
  title: string,
  body: string,
  color: string,
}

export type DeleteNote = {
  id: string,
}
