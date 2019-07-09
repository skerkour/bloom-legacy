use crate::domain::event;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
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
        use kernel::db::schema::calendar_events;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let delete_cmd = event::Delete {};

            let event_to_delete: event::CalendarEvent = calendar_events::dsl::calendar_events
                .filter(calendar_events::dsl::id.eq(msg.event_id))
                .filter(calendar_events::dsl::owner_id.eq(msg.actor_id))
                .for_update()
                .first(&conn)?;

            let (event_to_delete, _) = eventsourcing::execute(&conn, event_to_delete, &delete_cmd)?;

            // update event
            diesel::delete(&event_to_delete).execute(&conn)?;

            Ok(())
        })?);
    }
}
