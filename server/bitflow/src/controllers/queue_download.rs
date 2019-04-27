use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    Download,
    download,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueDownload {
    pub url: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for QueueDownload {
    type Result = Result<Download, KernelError>;
}

impl Handler<QueueDownload> for DbActor {
    type Result = Result<Download, KernelError>;

    fn handle(&mut self, msg: QueueDownload, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            bitflow_downloads,
            bitflow_downloads_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Download
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let queue_cmd = download::Queue{
                url: msg.url,
                owner_id: msg.account_id,
                metadata,
            };
            let (download, event, _) = eventsourcing::execute(&conn, Download::new(), &queue_cmd)?;

            diesel::insert_into(bitflow_downloads::dsl::bitflow_downloads)
                .values(&download)
                .execute(&conn)?;
            diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(download);
        })?);
    }
}
