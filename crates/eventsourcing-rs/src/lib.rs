use std::any::{Any, TypeId};
use std::collections::HashMap;


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

pub trait Command {
    type Aggregate: Aggregate;
    type Event: Event;
    type Context;
    type Error;
    type NonStoredData;

    fn validate(&self, conn: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error>;
    fn build_event(&self, conn: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error>;
}


pub trait Subscription {
    type Error;
    type Message;
    type Context;

    fn handle(&self, ctx: &Self::Context, msg: &Self::Message) -> Result<(), Self::Error>;
}


impl EventBus {
    fn new() -> Self {
        return EventBus{
            subscriptions: HashMap::new(),
        };
    }

    fn subscribe<C: Any, M: Any, E: Any>(&mut self, subscription: Box<(dyn Subscription<Context = C, Message = M, Error = E> + 'static)>)
     {
        let msg_id = TypeId::of::<M>();
        let boxed = Box::new(subscription);
        // let boxed = Box::new(subscription) as Box<dyn Subscription<Context = C, Message = M, Error = E>>;
        // print_typeid(&boxed);
        self.subscriptions.insert(msg_id, vec![boxed]);
    }

    fn publish<C: Any, M: Any, E: Any>(&mut self, ctx: &C, message: &M) -> Result<(), E> {
        let msg_id = TypeId::of::<M>();
        if let Some(subscriptions) = self.subscriptions.get_mut(&msg_id) {
            for subscription in subscriptions {
                // println!("{:?}", subscription.get_type_id());
                let subscription: &Box<Subscription<Context = C, Message = M, Error = E>> = subscription.downcast_ref().expect("error downcasting");
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


pub fn subscribe<C: Any, M: Any, E: Any>(subscription: Box<(dyn Subscription<Context = C, Message = M, Error = E> + 'static)>)
{
    event_bus().subscribe(subscription);
}

pub fn publish<C: Any, M: Any, E: Any>(ctx: &C, message: &M) -> Result<(), E> {
    return event_bus().publish(ctx, message);
}




pub fn execute<A, CTX, CMD, Ev, Err, D>(ctx: &CTX, aggregate: A, cmd: &CMD)
    -> Result<(A, Ev, D), Err>
    where A: Aggregate,
    CMD: Command<Aggregate = A, Event = Ev, Context = CTX, Error = Err, NonStoredData = D>,
    Ev: Event<Aggregate = A> + Any,
    Err: Any,
    CTX: Any {

    cmd.validate(ctx, &aggregate)?;
    let (event, non_stored_data) = cmd.build_event(ctx, &aggregate)?;
    let mut aggregate = event.apply(aggregate);
    aggregate.increment_version();
    aggregate.update_updated_at(event.timestamp());
    // publish::<_, _, Err>(ctx, &event)?;
    return Ok((aggregate, event, non_stored_data));
}
// pub fn execute execute<A, C, E, ED>(conn: &PgConnection, aggregate: &mut A, cmd: C, event: &mut E)() -> Result<(Aggregate)
