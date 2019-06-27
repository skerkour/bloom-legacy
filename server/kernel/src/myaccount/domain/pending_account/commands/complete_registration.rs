use crate::{error::KernelError, events::EventMetadata, myaccount::domain::pending_account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for CompleteRegistration {
    type Aggregate = pending_account::PendingAccount;
    type Event = pending_account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.trials + 1 >= 10 {
            return Err(KernelError::Validation(
                "Maximum number of trials reached. Please create another account.".to_string(),
            ));
        }

        // verify given code
        if !aggregate.verified {
            return Err(KernelError::Validation(
                "Please verify your email before.".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return Ok((
            pending_account::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: pending_account::EventData::RegistrationCompletedV1,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
