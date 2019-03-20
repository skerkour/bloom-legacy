#[derive(Debug, Clone)]
pub struct Account {
    id: u64,
    balance: i64,
    version: i64,
}

impl eventsourcing::Aggregate for Account {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, _timestamp: chrono::DateTime<chrono::Utc>) {
    }
}



#[derive(Clone, Debug)]
struct WithdrawFunds {
    account: String,
    amount: i64,
}

impl eventsourcing::Command for WithdrawFunds {
    type Aggregate = Account;
    type Event = AccountEvent;
    type DbConn = ();
    type Error = String;

    fn build_event(&self, _conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<Self::Event, Self::Error> {
        let data = AccountEventData::FundsWithdrawn(FundsWithdrawn{
            account: self.account.clone(),
            amount: self.amount,
        });
        return  Ok(AccountEvent{
            id: 1, // random
            timestamp: 123,
            data,
            aggregate_id: aggregate.id,
        });
    }

    fn validate(&self, _conn: &Self::DbConn, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }
}


#[derive(Clone, Debug)]
pub struct AccountEvent {
    pub id: u64,
    pub timestamp: u64,
    pub data: AccountEventData,
    pub aggregate_id: u64,
}

#[derive(Debug, Clone)]
pub enum AccountEventData {
    FundsWithdrawn(FundsWithdrawn),
    FundsDeposited(FundsDeposited),
}

#[derive(Debug, Clone)]
pub struct FundsWithdrawn {
    account: String,
    amount: i64,
}

#[derive(Debug, Clone)]
pub struct FundsDeposited {
    account: String,
    amount: i64,
}

impl eventsourcing::Event for AccountEvent {
    type Aggregate = Account;

    fn apply(&self, aggregate: &Self::Aggregate) -> Self::Aggregate {
        match self.data {
            AccountEventData::FundsWithdrawn(ref data) => Account {
                balance: aggregate.balance - data.amount,
                ..aggregate.clone()
            },
            AccountEventData::FundsDeposited(ref data) => Account {
                balance: aggregate.balance + data.amount,
                ..aggregate.clone()
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return chrono::Utc::now();
    }
}


fn main() {
    let withdraw_cmd = WithdrawFunds{
        account: "SAVINGS100".to_string(),
        amount: 500,
    };

    let initial_state = Account {
        id: 42,
        balance: 800,
        version: 1,
    };

    let (current_state, event) = eventsourcing::execute(&(), &initial_state, &withdraw_cmd)
        .expect("error execurting");

    println!("initial state: {:#?}", &initial_state);
    println!("current state: {:#?}", &current_state);
    println!("event: {:#?}", &event);
    assert_eq!(current_state.balance, 300);
    assert_eq!(current_state.version, 2);
}
