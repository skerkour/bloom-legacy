#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub use eventsourcing_derive::{Aggregate, EventTs};

pub trait AggregateData {
    fn increment_version(&mut self);
    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>);
}

pub trait EventData {
    #[inline]
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait Event {
    type Aggregate: AggregateData;

    fn apply(&self, agrgegate: Self::Aggregate) -> Self::Aggregate;
}

pub trait Command {
    type Aggregate: AggregateData;
    type Event: Event + EventData;
    type Context;
    type Error;

    fn validate(
        &self,
        conn: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error>;
    fn build_event(
        &self,
        conn: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error>;
}

pub trait Subscription {
    type Error;
    type Event;
    type Context;

    fn handle(&self, ctx: &Self::Context, event: &Self::Event) -> Result<(), Self::Error>;
}

impl EventBus {
    fn new() -> Self {
        return EventBus {
            subscriptions: HashMap::new(),
        };
    }

    fn subscribe<C: Any, Ev: Any, Err: Any>(
        &mut self,
        subscription: Box<(dyn Subscription<Context = C, Event = Ev, Error = Err> + 'static)>,
    ) {
        let msg_id = TypeId::of::<Ev>();
        let boxed = Box::new(subscription);
        // let boxed = Box::new(subscription) as Box<dyn Subscription<Context = C, Event = Ev, Error = Err>>;
        // print_typeid(&boxed);
        if let Some(subscriptions) = self.subscriptions.get_mut(&msg_id) {
            subscriptions.push(boxed);
        } else {
            self.subscriptions.insert(msg_id, vec![boxed]);
        }
    }

    fn publish<C: Any, Ev: Any, Err: Any>(&mut self, ctx: &C, message: &Ev) -> Result<(), Err> {
        let msg_id = TypeId::of::<Ev>();
        if let Some(subscriptions) = self.subscriptions.get_mut(&msg_id) {
            for subscription in subscriptions {
                // println!("{:?}", subscription.get_type_id());
                let subscription: &Box<Subscription<Context = C, Event = Ev, Error = Err>> =
                    subscription.downcast_ref().expect("error downcasting");
                subscription.handle(ctx, message)?;
            }
        }
        return Ok(());
    }
}
// pub type Subscription<C, M, E> = for<'r> fn(ctx: &'r C, message: &'r M) -> Result<(), E>;

pub type TypeMap<A> = HashMap<TypeId, A>;

struct EventBus {
    pub subscriptions: TypeMap<Vec<Box<Any>>>,
}

// impl EventBus {
//     fn new() -> Self {
//         return EventBus{
//             subscriptions: HashMap::new(),
//         };
//     }

//     fn subscribe<C: Any, M: Any, E: Any>(&mut self, subscription: Subscription<C, M, E>) {
//         let msg_id = TypeId::of::<M>();
//         let boxed = Box::new(subscription);

//         if let Some(subscriptions) = self.subscriptions.get_mut(&msg_id) {
//             subscriptions.push(boxed);
//             return;
//         }

//         self.subscriptions.insert(msg_id, vec![boxed]);
//     }

//     fn publish<C: Any, M: Any, E: Any>(&mut self, ctx: &C, message: &M) -> Result<(), E> {
//         let msg_id = TypeId::of::<M>();
//         if let Some(subscriptions) = self.subscriptions.get_mut(&msg_id) {
//             for subscription in subscriptions {
//                 let subscription = subscription.downcast_ref::<Subscription<C, M, E>>().unwrap();
//                 subscription(ctx, message)?;
//             }
//         }
//         return Ok(());
//     }
// }

static mut EVENT_BUS: Option<EventBus> = None;

fn event_bus() -> &'static mut EventBus {
    unsafe {
        if EVENT_BUS.is_none() {
            EVENT_BUS = Some(EventBus::new());
        }

        return EVENT_BUS.as_mut().unwrap();
    }
}

// pub fn subscribe<M: Any, C: Any, E: Any>(subscription: Subscription<C, M, E>) {
//     event_bus().subscribe(subscription);
// }

// fn publish<C: Any, M: Any, E: Any>(ctx: &C, message: &M) -> Result<(), E> {
//     return event_bus().publish(ctx, message);
// }

pub fn subscribe<C: Any, M: Any, E: Any>(
    subscription: Box<(dyn Subscription<Context = C, Event = M, Error = E> + 'static)>,
) {
    event_bus().subscribe(subscription);
}

pub fn publish<C: Any, M: Any, E: Any>(ctx: &C, message: &M) -> Result<(), E> {
    return event_bus().publish(ctx, message);
}

pub fn execute<A, CTX, CMD, Ev, Err>(ctx: &CTX, aggregate: A, cmd: &CMD) -> Result<(A, Ev), Err>
where
    A: AggregateData,
    CMD: Command<Aggregate = A, Event = Ev, Context = CTX, Error = Err>,
    Ev: Event<Aggregate = A> + Any + EventData,
    Err: Any,
    CTX: Any,
{
    cmd.validate(ctx, &aggregate)?;
    let event = cmd.build_event(ctx, &aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    aggregate.update_updated_at(event.timestamp());
    publish::<_, _, Err>(ctx, &event)?;
    return Ok((aggregate, event));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
