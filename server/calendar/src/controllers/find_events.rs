use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use diesel::{
    sql_query,
    pg::types::sql_types,
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
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let cal_events: Vec<domain::CalendarEvent> = sql_query(include_str!("../../sql_requests/find_events.sql"))
            .bind::<sql_types::Timestamptz, _>(msg.start_at)
            .bind::<sql_types::Timestamptz, _>(msg.end_at)
            .load(&conn)?;

        return Ok(cal_events);
    }
}
