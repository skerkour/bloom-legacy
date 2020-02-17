/* eslint-disable camelcase */

enum Method {
  ListNotes = 'notes.list_notes',
  ListArchived = 'notes.list_archived',
  CreateNote = 'notes.create_note',
  UpdateNote = 'notes.update_note',
  DeleteNote = 'notes.delete_note',
}

export { Method };

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
