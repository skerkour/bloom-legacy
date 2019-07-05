use crate::domain::event;
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEvent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,

    pub event_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateEvent {
    type Result = Result<event::CalendarEvent, KernelError>;
}

impl Handler<UpdateEvent> for DbActor {
    type Result = Result<event::CalendarEvent, KernelError>;

    fn handle(&mut self, msg: UpdateEvent, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{calendar_events, calendar_events_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let event_to_update: event::CalendarEvent = calendar_events::dsl::calendar_events
                .filter(calendar_events::dsl::id.eq(msg.event_id))
                .filter(calendar_events::dsl::owner_id.eq(msg.actor_id))
                .filter(calendar_events::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // title
            let event_to_update = match &msg.title {
                Some(title) if title != &event_to_update.title => {
                    let update_title_cmd = event::UpdateTitle {
                        title: title.clone(),
                        metadata: metadata.clone(),
                    };

                    let (event_to_update, event, _) =
                        eventsourcing::execute(&conn, event_to_update, &update_title_cmd)?;

                    // update event
                    diesel::update(&event_to_update)
                        .set(&event_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(calendar_events_events::dsl::calendar_events_events)
                        .values(&event)
                        .execute(&conn)?;
                    event_to_update
                }
                _ => event_to_update,
            };

            // description
            let event_to_update = match &msg.description {
                Some(description) if description != &event_to_update.description => {
                    let update_description_cmd = event::UpdateDescription {
                        description: description.clone(),
                        metadata: metadata.clone(),
                    };

                    let (event_to_update, event, _) =
                        eventsourcing::execute(&conn, event_to_update, &update_description_cmd)?;

                    // update event
                    diesel::update(&event_to_update)
                        .set(&event_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(calendar_events_events::dsl::calendar_events_events)
                        .values(&event)
                        .execute(&conn)?;
                    event_to_update
                }
                _ => event_to_update,
            };

            // end_at
            let event_to_update = match &msg.end_at {
                Some(end_at) if end_at != &event_to_update.end_at => {
                    let update_end_at_cmd = event::UpdateEndAt {
                        end_at: *end_at,
                        metadata: metadata.clone(),
                    };

                    let (event_to_update, event, _) =
                        eventsourcing::execute(&conn, event_to_update, &update_end_at_cmd)?;

                    // update event
                    diesel::update(&event_to_update)
                        .set(&event_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(calendar_events_events::dsl::calendar_events_events)
                        .values(&event)
                        .execute(&conn)?;
                    event_to_update
                }
                _ => event_to_update,
            };

            // start_at
            let event_to_update = match &msg.start_at {
                Some(start_at) if start_at != &event_to_update.start_at => {
                    let update_start_at_cmd = event::UpdateStartAt {
                        start_at: *start_at,
                        metadata: metadata.clone(),
                    };

                    let (event_to_update, event, _) =
                        eventsourcing::execute(&conn, event_to_update, &update_start_at_cmd)?;

                    // update event
                    diesel::update(&event_to_update)
                        .set(&event_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(calendar_events_events::dsl::calendar_events_events)
                        .values(&event)
                        .execute(&conn)?;
                    event_to_update
                }
                _ => event_to_update,
            };

            Ok(event_to_update)
        })?)
    }
}
