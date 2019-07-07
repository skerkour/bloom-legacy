use crate::domain::event;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateEvent {
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,

    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateEvent {
    type Result = Result<event::CalendarEvent, KernelError>;
}

impl Handler<CreateEvent> for DbActor {
    type Result = Result<event::CalendarEvent, KernelError>;

    fn handle(&mut self, msg: CreateEvent, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::calendar_events;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        Ok(conn.transaction::<_, KernelError, _>(|| {
            let create_cmd = event::Create {
                title: msg.title,
                description: msg.description,
                start_at: msg.start_at,
                end_at: msg.end_at,
                owner_id: msg.owner_id,
            };
            let (new_calendar_event, _) =
                eventsourcing::execute(&conn, event::CalendarEvent::new(), &create_cmd)?;
            diesel::insert_into(calendar_events::dsl::calendar_events)
                .values(&new_calendar_event)
                .execute(&conn)?;

            Ok(new_calendar_event)
        })?)
    }
}
