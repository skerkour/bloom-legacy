pub trait Aggregate {
    fn increment_version(&mut self);
    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>);
}

pub trait Event {
    type Aggregate: Aggregate;

    fn apply(&self, agrgegate: &Self::Aggregate) -> Self::Aggregate;
    #[inline]
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait Command {
    type Aggregate: Aggregate;
    type Event: Event;
    type DbConn;
    type Error;

    fn build_event(&self, conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<Self::Event, Self::Error>;
    fn validate(&self, conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<(), Self::Error>;
}


pub fn execute<A, CON, CMD, Ev, Err>(conn: &CON, aggregate: &A, cmd: &CMD)
    -> Result<(A, Ev), Err>
    where A: Aggregate,
    CMD: Command<Aggregate = A, Event = Ev, DbConn = CON, Error = Err>,
    Ev: Event<Aggregate = A> {

    cmd.validate(conn, aggregate)?;
    let event = cmd.build_event(conn, aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    aggregate.update_updated_at(event.timestamp());
    return Ok((aggregate, event));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
