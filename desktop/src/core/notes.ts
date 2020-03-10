export enum Method {
  ListNotes = 'notes.listNotes',
  ListArchived = 'notes.listArchived',
  CreateNote = 'notes.createNote',
  UpdateNote = 'notes.updateNote',
  DeleteNote = 'notes.deleteNote',
}

export type Note = {
  id: string,
  title: string,
  body: string,
  createdAt: Date,
  updatedAt: Date,
  color: string,
  archivedAt: Date | null,
  isPinned: boolean,
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
