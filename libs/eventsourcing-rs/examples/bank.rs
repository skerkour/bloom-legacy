use eventsourcing::{Aggregate, Event, EventTs};
#[derive(Aggregate, Debug, Clone)]
pub struct Account {
    id: u64,
    balance: i64,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug)]
struct WithdrawFunds {
    account: String,
    amount: i64,
}

impl eventsourcing::Command for WithdrawFunds {
    type Aggregate = Account;
    type Event = FundsWithdrawn;
    type Context = Ctx;
    type Error = String;

    fn build_event(
        &self,
        _conn: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(FundsWithdrawn {
            timestamp: chrono::Utc::now(),
            account: self.account.clone(),
            amount: self.amount,
        });
    }

    fn validate(
        &self,
        _conn: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }
}

#[derive(Debug, Clone, EventTs)]
pub struct FundsWithdrawn {
    account: String,
    amount: i64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, EventTs)]
pub struct FundsDeposited {
    account: String,
    amount: i64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct Ctx {
    pub x: i32,
}

impl Event for FundsWithdrawn {
    type Aggregate = Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            balance: aggregate.balance - self.amount,
            ..aggregate
        };
    }
}

impl Event for FundsDeposited {
    type Aggregate = Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            balance: aggregate.balance + self.amount,
            ..aggregate
        };
    }
}

// fn create_profile(_ctx: &Ctx, event: &AccountEvent) -> Result<(), String> {
//     println!("event received: {:?}", event.data);
//     return Ok(());
// }

struct FundsWithdrawnReactor;
impl eventsourcing::Subscription for FundsWithdrawnReactor {
    type Error = String;
    type Event = FundsWithdrawn;
    type Context = Ctx;

    fn handle(&self, _ctx: &Self::Context, event: &Self::Event) -> Result<(), Self::Error> {
        println!("amount withdrawn: {}", event.amount);
        return Ok(());
    }
}

fn main() {
    eventsourcing::subscribe::<_, FundsWithdrawn, _>(Box::new(FundsWithdrawnReactor {}));

    let withdraw_cmd = WithdrawFunds {
        account: "SAVINGS100".to_string(),
        amount: 500,
    };

    let initial_state = Account {
        id: 42,
        balance: 800,
        version: 1,
        updated_at: chrono::Utc::now(),
    };
    let initial_state2 = initial_state.clone();

    let x = 42;
    let ctx = Ctx { x: x };

    let (current_state, event) =
        eventsourcing::execute(&ctx, initial_state, &withdraw_cmd).expect("error execurting");

    println!("initial state: {:#?}", &initial_state2);
    println!("current state: {:#?}", &current_state);
    println!("event: {:#?}", &event);
    assert_eq!(current_state.balance, 300);
    assert_eq!(current_state.version, 2);
}
