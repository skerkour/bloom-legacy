use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartDownload {
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for StartDownload {
    type Result = Result<domain::Download, KernelError>;
}

impl Handler<StartDownload> for DbActor {
    type Result = Result<domain::Download, KernelError>;

    fn handle(&mut self, msg: StartDownload, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            bitflow_downloads,
            bitflow_downloads_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::deleted_at.is_null())
                .filter(bitflow_downloads::dsl::removed_at.is_null())
                .filter(bitflow_downloads::dsl::status.eq(domain::download::DownloadStatus::Queued))
                .for_update()
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: None, // Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: None, // Some(msg.session_id),
            };
            let start_cmd = domain::download::Start{
                metadata,
            };

            let (download, event, _) = eventsourcing::execute(&conn, download, &start_cmd)?;
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
