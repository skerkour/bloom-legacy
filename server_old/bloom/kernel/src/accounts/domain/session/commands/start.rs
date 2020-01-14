use crate::{accounts::domain::session, utils};
use bloom_const::accounts;
use bloom_error::BloomError;
use crypto42::kdf::argon2id;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
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
    type Error = BloomError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let token = utils::random_base64_string(accounts::SESSION_TOKEN_BYTES as usize);
        let hashed_token = argon2id::hash_password(
            token.as_bytes(),
            accounts::SESSION_TOKEN_ARGON2_OPSLIMIT,
            accounts::SESSION_TOKEN_ARGON2_MEMLIMIT,
        )?
        .to_string();

        return Ok(Started {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            account_id: self.account_id,
            token_hash: hashed_token,
            token_plaintext: token,
            ip: self.ip.clone(),
            user_agent: self.user_agent.clone(),
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
    pub user_agent: String,
}

impl Event for Started {
    type Aggregate = session::Session;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            user_agent: self.user_agent.clone(),
            ip: self.ip.clone(),
            token_hash: self.token_hash.clone(),
            account_id: self.account_id,
        };
    }
}
