import { BlmObject } from './index';

export const NOTE_TYPE = 'com.bloom42.bloom.note';

export enum Method {
  ListNotes = 'notes.listNotes',
  ListArchived = 'notes.listArchived',
  CreateNote = 'notes.createNote',
  UpdateNote = 'notes.updateNote',
  DeleteNote = 'notes.deleteNote',
}

export type Note = {
  title: string,
  body: string,
  color: string,
  archivedAt: Date | null,
  isPinned: boolean,
}

export type Notes = {
  notes: BlmObject[],
}

export type CreateNote = {
  title: string,
  body: string,
  color: string,
}

export type DeleteNote = {
  id: string,
}
