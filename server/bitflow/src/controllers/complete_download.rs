use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteDownload {
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub download_id: uuid::Uuid,
    pub complete_data: domain::download::CompleteData,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteDownload {
    type Result = Result<domain::Download, KernelError>;
}

impl Handler<CompleteDownload> for DbActor {
    type Result = Result<domain::Download, KernelError>;

    fn handle(&mut self, msg: CompleteDownload, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            bitflow_downloads,
            bitflow_downloads_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(msg.download_id))
                .filter(bitflow_downloads::dsl::deleted_at.is_null())
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: None, // Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: None, // Some(msg.session_id),
            };
            let complete_cmd = domain::download::Complete{
                data: msg.complete_data,
                metadata,
            };

            let (download, event, _) = eventsourcing::execute(&conn, download, &complete_cmd)?;
            diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                .values(&event)
                .execute(&conn)?;
            diesel::update(&download)
                .set(&download)
                .execute(&conn)?;

            return Ok(download);
        })?);
    }
}
