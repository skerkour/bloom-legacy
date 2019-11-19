use crate::{accounts::domain::pending_account, utils};
use bloom_const::accounts;
use bloom_error::BloomError;
use chrono::Duration;
use crypto42::kdf::argon2id;
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
    type Error = BloomError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        let now = chrono::Utc::now();
        if now.signed_duration_since(aggregate.updated_at) < Duration::seconds(20) {
            return Err(BloomError::Validation(
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
        let verification_code_hash = argon2id::hash_password(
            code.as_bytes(),
            accounts::PENDING_USER_CODE_ARGON2_OPSLIMIT,
            accounts::PENDING_USER_CODE_ARGON2_MEMLIMIT,
        )?
        .to_string();

        return Ok(NewCodeSent {
            timestamp: chrono::Utc::now(),
            verification_code_hash,
            code,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct NewCodeSent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub verification_code_hash: String,
    pub code: String,
}

impl Event for NewCodeSent {
    type Aggregate = pending_account::PendingAccount;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            verification_code_hash: self.verification_code_hash.clone(),
            ..aggregate
        };
    }
}
