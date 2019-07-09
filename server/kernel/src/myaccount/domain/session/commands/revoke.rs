use crate::{error::KernelError, myaccount::domain::session};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Revoke {
    pub current_session_id: Option<uuid::Uuid>,
    pub reason: session::RevokedReason,
}

impl eventsourcing::Command for Revoke {
    type Aggregate = session::Session;
    type Event = Revoked;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if let Some(current_session_id) = self.current_session_id {
            if current_session_id == aggregate.id {
                return Err(KernelError::Validation(
                    "Revoking current session is not permitted".to_string(),
                ));
            }
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Revoked {
            timestamp: chrono::Utc::now(),
            reason: self.reason,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Revoked {
    pub reason: session::RevokedReason,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Revoked {
    type Aggregate = session::Session;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
