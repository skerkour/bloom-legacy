class Note {
  id: string;
  title: string;
  body: string;
  createdAt: Date;
  updatedAt: Date;
  color: number;
  archivedAt: Date | null;
  isPinned: boolean;

  constructor({
    id,
    body,
    title,
    createdAt,
    updatedAt,
    color,
    archivedAt,
    isPinned,
  }: {
    id: string,
    body: string,
    title: string,
    createdAt: Date,
    updatedAt: Date,
    color: number, archivedAt: Date | null, isPinned: boolean }) {
    this.id = id;
    this.body = body;
    this.title = title;
    this.createdAt = createdAt;
    this.updatedAt = updatedAt;
    this.color = color;
    this.archivedAt = archivedAt;
    this.isPinned = isPinned;
  }
}

export default Note;
