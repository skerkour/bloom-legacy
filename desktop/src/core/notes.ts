export const NOTE_TYPE = 'com.bloom42.bloom.note';

export enum Method {
  FindNotes = 'notes.findNotes',
  FindArchived = 'notes.findArchived',
  CreateNote = 'notes.createNote',
  UpdateNote = 'notes.updateNote',
  DeleteNote = 'notes.deleteNote',
}

export type Note = {
  title: string,
  body: string,
  color: string,
  archivedAt: Date | null,
  isFavorite: boolean,
}
