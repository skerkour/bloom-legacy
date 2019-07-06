use crate::{
    error::KernelError, events::EventMetadata, myaccount, myaccount::domain::session, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Start {
    pub account_id: uuid::Uuid,
    pub ip: String,
    pub user_agent: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Start {
    type Aggregate = session::Session;
    type Event = session::Event;
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
        let timestamp = chrono::Utc::now();

        let new_session_id = uuid::Uuid::new_v4();
        let data = session::EventData::StartedV1(session::StartedV1 {
            id: new_session_id,
            account_id: self.account_id,
            token_hash: hashed_token,
            token_plaintext: token,
            ip: self.ip.clone(),
            device: session::Device {},
            location: session::Location {},
        });

        return Ok(session::Event {
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: new_session_id,
            metadata: self.metadata.clone(),
        });
    }
}
