use crate::{
    services::common::events::EventMetadata,
    services::account::domain::pending_account,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: String,
    pub code: String,
    pub metadata: EventMetadata,
}


impl<'a> eventsourcing::Command<'a> for Verify {
    type Aggregate = pending_account::PendingAccount;
    type Event = pending_account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let metadata = self.metadata.clone();
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        let data = if aggregate.trials + 1 >= 10 {
            pending_account::EventData::VerificationFailedV1("Maximum number of trials reached. Please create another account.".to_string())
        } else if !bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)? {
            // verify given code
            pending_account::EventData::VerificationFailedV1("Code is not valid.".to_string())
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            pending_account::EventData::VerificationFailedV1("Code has expired. Please create another account.".to_string())
        } else {
            pending_account::EventData::VerificationSucceededV1
        };

        return  Ok((pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: aggregate.id,
            metadata,
        }, ()));
    }
}

