use serde::{Serialize, Deserialize};
use crate::{
    users::domain::session,
    error::KernelError,
    events::EventMetadata,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Revoke {
    pub current_session_id: Option<uuid::Uuid>,
    pub reason: session::RevokedReason,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for Revoke {
    type Aggregate = session::Session;
    type Event = session::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation("Session is currently not active.".to_string()));
        }

        if let Some(current_session_id) = self.current_session_id {
            if current_session_id == aggregate.id {
                return Err(KernelError::Validation("Revoking current session is not permitted".to_string()));
            }
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = session::EventData::RevokedV1(self.reason);
        let timestamp = chrono::Utc::now();

        return  Ok((session::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
