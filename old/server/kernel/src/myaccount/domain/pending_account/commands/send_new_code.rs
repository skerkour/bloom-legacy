use crate::{error::KernelError, myaccount, myaccount::domain::pending_account, utils};
use chrono::Duration;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone, Debug)]
pub struct SendNewCode {}

impl eventsourcing::Command for SendNewCode {
    type Aggregate = pending_account::PendingAccount;
    type Event = NewCodeSent;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        let now = chrono::Utc::now();
        if now.signed_duration_since(aggregate.updated_at) < Duration::seconds(20) {
            return Err(KernelError::Validation(
                "Please wait at least for 20 seconds beffore requesting a new code".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let code = utils::random_digit_string(8);
        let token_hash = bcrypt::hash(&code, myaccount::PENDING_USER_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(NewCodeSent {
            timestamp: chrono::Utc::now(),
            token_hash,
            code,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct NewCodeSent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub token_hash: String,
    pub code: String,
}

impl Event for NewCodeSent {
    type Aggregate = pending_account::PendingAccount;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            token: self.token_hash.clone(),
            ..aggregate
        };
    }
}
