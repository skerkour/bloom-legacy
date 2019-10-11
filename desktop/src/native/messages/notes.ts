export interface Note {
  id: string,
  title: string,
  body: string,
  createdAt: Date,
  updatedAt: Date,
  color: number,
  archivedAt: Date | null,
  isPinned: boolean,
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
