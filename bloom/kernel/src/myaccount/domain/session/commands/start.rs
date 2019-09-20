use crate::{error::KernelError, myaccount, myaccount::domain::session, utils};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Start {
    pub account_id: uuid::Uuid,
    pub ip: String,
    pub user_agent: String,
}

impl eventsourcing::Command for Start {
    type Aggregate = session::Session;
    type Event = Started;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.version != 0 {
            return Err(KernelError::Validation(
                "Session is already started.".to_string(),
            ));
        }
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(
            myaccount::SESSION_TOKEN_MIN_LENGTH,
            myaccount::SESSION_TOKEN_MAX_LENGTH,
        );
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, myaccount::SESSION_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(Started {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            account_id: self.account_id,
            token_hash: hashed_token,
            token_plaintext: token,
            ip: self.ip.clone(),
            device: session::Device {},
            location: session::Location {},
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Started {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub token_hash: String,
    pub token_plaintext: String,
    pub ip: String,
    pub location: session::Location,
    pub device: session::Device,
}

impl Event for Started {
    type Aggregate = session::Session;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            device: self.device.clone(),
            ip: self.ip.clone(),
            location: Some(self.location.clone()),
            token: self.token_hash.clone(),
            account_id: self.account_id,
        };
    }
}
