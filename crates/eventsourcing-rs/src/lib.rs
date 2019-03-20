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

    fn build_event(&self, aggregate: &Self::Aggregate) -> Result<Self::Event, String>;
    fn validate(&self, aggregate: &Self::Aggregate) -> Result<(), String>;
}


pub fn execute<A, C, E>(aggregate: &A, cmd: &C)
    -> Result<(A, E), String>
    where A: Aggregate,
    C: Command<Aggregate = A, Event = E>,
    E: Event<Aggregate = A> {

    let event = cmd.build_event(aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    return Ok((aggregate, event));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
