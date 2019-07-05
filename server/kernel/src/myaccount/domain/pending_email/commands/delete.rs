use crate::{error::KernelError, events::EventMetadata, myaccount::domain::pending_email};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Delete {
    type Aggregate = pending_email::PendingEmail;
    type Event = pending_email::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation(
                "PDeleteing email has already been deleted.".to_string(),
            ));
        }
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(
            pending_email::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: pending_email::EventData::DeletedV1,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            });
    }
}
