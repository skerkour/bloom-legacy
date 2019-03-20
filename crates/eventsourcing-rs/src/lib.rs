pub trait Aggregate {
    fn increment_version(&mut self);
}

pub trait Event {
    type Aggregate: Aggregate;

    fn apply(&self, agrgegate: &Self::Aggregate) -> Self::Aggregate;
}

pub trait Command {
    type Aggregate: Aggregate;
    type Event: Event;
    type DbConn;

    fn build_event(&self, conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<Self::Event, String>;
    fn validate(&self, conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<(), String>;
}


pub fn execute<A, CON, CMD, E>(conn: &CON, aggregate: &A, cmd: &CMD)
    -> Result<(A, E), String>
    where A: Aggregate,
    CMD: Command<Aggregate = A, Event = E, DbConn = CON>,
    E: Event<Aggregate = A> {

    cmd.validate(conn, aggregate)?;
    let event = cmd.build_event(conn, aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    return Ok((aggregate, event));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
