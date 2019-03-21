use serde::{Serialize, Deserialize};
use crate::{
    services::account::domain::session,
    services::account,
    error::KernelError,
    services::common::utils,
    services::common::events::EventMetadata,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use rand::Rng;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Start {
    pub account_id: uuid::Uuid,
    pub ip: String,
    pub user_agent: String,
    pub metadata: EventMetadata,
}


#[derive(Clone, Debug)]
pub struct StartNonStored {
    pub token_cleartext: String,
}


impl<'a> eventsourcing::Command<'a> for Start {
    type Aggregate = session::Session;
    type Event = session::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = StartNonStored;

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.version != 0 {
            return Err(KernelError::Validation("Session is already started.".to_string()));
        }
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(account::SESSION_TOKEN_MIN_LENGTH, account::SESSION_TOKEN_MAX_LENGTH);
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, account::SESSION_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let timestamp = chrono::Utc::now();

        let new_session_id = uuid::Uuid::new_v4();
        let data = session::EventData::StartedV1(session::StartedV1{
            id: new_session_id,
            account_id: self.account_id,
            token: hashed_token,
            ip: self.ip.clone(),
            device: session::Device{}, // TODO
            location: session::Location{},
        });

        let non_stored = StartNonStored{
            token_cleartext: token,
        };

        return  Ok((session::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: new_session_id,
            metadata: self.metadata.clone(),
        }, non_stored));
    }
}
