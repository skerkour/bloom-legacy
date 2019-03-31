use serde::{Serialize, Deserialize};
use crate::{
    accounts,
    accounts::domain::sessions,
    error::KernelError,
    utils,
    events::EventMetadata,
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
    pub account_agent: String,
    pub metadata: EventMetadata,
}


#[derive(Clone, Debug)]
pub struct StartNonStored {
    pub token_plaintext: String,
}


impl<'a> eventsourcing::Command<'a> for Start {
    type Aggregate = sessions::Session;
    type Event = sessions::Event;
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
        let token_length = rng.gen_range(accounts::SESSION_TOKEN_MIN_LENGTH, accounts::SESSION_TOKEN_MAX_LENGTH);
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, accounts::SESSION_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let timestamp = chrono::Utc::now();

        let new_session_id = uuid::Uuid::new_v4();
        let data = sessions::EventData::StartedV1(sessions::StartedV1{
            id: new_session_id,
            account_id: self.account_id,
            token: hashed_token,
            ip: self.ip.clone(),
            device: sessions::Device{},
            location: sessions::Location{},
        });

        let non_stored = StartNonStored{
            token_plaintext: token,
        };

        return  Ok((sessions::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: new_session_id,
            metadata: self.metadata.clone(),
        }, non_stored));
    }
}
