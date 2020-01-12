/* eslint-disable camelcase */

export type Note = {
  id: string,
  title: string,
  body: string,
  created_at: Date,
  updated_at: Date,
  color: number,
  archived_at: Date | null,
  is_pinned: boolean,
}

export type Notes = {
  notes: Note[],
}

export type CreateNote = {
  title: string,
  body: string,
  color: number,
}

export type DeleteNote = {
  id: string,
}
