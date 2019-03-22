use crate::{
    error::KernelError,
    services::account::domain::pending_account,
    services::common::events::EventMetadata,
};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: String,
    pub code: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for CompleteRegistration {
    type Aggregate = pending_account::PendingAccount;
    type Event = pending_account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        let now = Utc::now();

        if aggregate.trials + 1 >= 10 {
            return Err(KernelError::Validation("Maximum number of trials reached. Please create another account.".to_string()));
        }

        // verify given code
        let is_valid = bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)?;
        if !is_valid {
            return Err(KernelError::Validation("Code is not valid.".to_string()));
        }

        // verify code expiration
        let duration = aggregate.created_at.signed_duration_since(now);
        if duration.num_minutes() > 30 {
            return Err(KernelError::Validation("Code has expired. Please create another account.".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return  Ok((pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: pending_account::EventData::RegistrationCompletedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
