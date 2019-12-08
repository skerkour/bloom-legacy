class Event {
  id: string;
  title: string;
  description: string;
  createdAt: Date;
  updatedAt: Date;
  startAt: Date;
  endAt: Date;

  constructor({
    id,
    description,
    title,
    createdAt,
    updatedAt,
    startAt,
    endAt,
  }: {
    id: string,
    description: string,
    title: string,
    createdAt: Date,
    updatedAt: Date,
    startAt: Date, endAt: Date}) {
    this.id = id;
    this.description = description;
    this.title = title;
    this.createdAt = createdAt;
    this.updatedAt = updatedAt;
    this.startAt = startAt;
    this.endAt = endAt;
  }
}

export default Event;
