pub trait Aggregate {
    fn increment_version(&mut self);
    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>);
}

pub trait Event {
    type Aggregate: Aggregate;

    fn apply(&self, agrgegate: Self::Aggregate) -> Self::Aggregate;
    #[inline]
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait Command<'a> {
    type Aggregate: Aggregate;
    type Event: Event;
    type Context;
    type Error;
    type NonStoredData;

    fn validate(&self, conn: &'a Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error>;
    fn build_event(&self, conn: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error>;
}


pub fn execute<'a, A, CON, CMD, Ev, Err, D>(ctx: &'a CON, aggregate: A, cmd: &CMD)
    -> Result<(A, Ev, D), Err>
    where A: Aggregate,
    CMD: Command<'a, Aggregate = A, Event = Ev, Context = CON, Error = Err, NonStoredData = D>,
    Ev: Event<Aggregate = A> {

    cmd.validate(ctx, &aggregate)?;
    let (event, non_stored_data) = cmd.build_event(ctx, &aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    aggregate.update_updated_at(event.timestamp());
    return Ok((aggregate, event, non_stored_data));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
