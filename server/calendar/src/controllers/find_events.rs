use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::{
    domain,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindEvents {
    pub owner_id: uuid::Uuid,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl Message for FindEvents {
    type Result = Result<Vec<domain::CalendarEvent>, KernelError>;
}

impl Handler<FindEvents> for DbActor {
    type Result = Result<Vec<domain::CalendarEvent>, KernelError>;

    fn handle(&mut self, msg: FindEvents, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            calendar_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let cal_events: Vec<domain::CalendarEvent> = calendar_events::dsl::calendar_events
            .filter(calendar_events::dsl::owner_id.eq(msg.owner_id))
            .filter(calendar_events::dsl::deleted_at.is_null())
            .filter(
                calendar_events::dsl::start_at.between(msg.start_at, msg.end_at)
                .or(calendar_events::dsl::end_at.between(msg.start_at, msg.end_at))
            )
            .load(&conn)?;

        return Ok(cal_events);
    }
}
