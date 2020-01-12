/* eslint-disable camelcase */

export interface Note {
  id: string,
  title: string,
  body: string,
  created_at: Date,
  updated_at: Date,
  color: number,
  archived_at: Date | null,
  is_pinned: boolean,
}

export interface GuiNote {
  note: Note,
}

export interface GuiNotes {
  notes: Note[],
}

export interface GuiCreateNote {
  title: string,
  body: string,
  color: number,
}

export interface GuiDeleteNote {
  id: string,
}
