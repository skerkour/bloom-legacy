use crate::domain::event;
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteEvent {
    pub event_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteEvent {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteEvent> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteEvent, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{calendar_events, calendar_events_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let delete_cmd = event::Delete { metadata };

            let event_to_delete: event::CalendarEvent = calendar_events::dsl::calendar_events
                .filter(calendar_events::dsl::id.eq(msg.event_id))
                .filter(calendar_events::dsl::owner_id.eq(msg.actor_id))
                .filter(calendar_events::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (event_to_delete, event, _) =
                eventsourcing::execute(&conn, event_to_delete, &delete_cmd)?;

            // update event
            diesel::update(&event_to_delete)
                .set(&event_to_delete)
                .execute(&conn)?;
            diesel::insert_into(calendar_events_events::dsl::calendar_events_events)
                .values(&event)
                .execute(&conn)?;

            Ok(())
        })?);
    }
}
