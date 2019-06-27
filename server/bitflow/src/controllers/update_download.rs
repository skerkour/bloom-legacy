use crate::domain::{download, Download};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDownload {
    pub name: Option<String>,
    pub progress: Option<u32>,
    pub download_id: uuid::Uuid,
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for UpdateDownload {
    type Result = Result<Download, KernelError>;
}

impl Handler<UpdateDownload> for DbActor {
    type Result = Result<Download, KernelError>;

    fn handle(&mut self, msg: UpdateDownload, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{bitflow_downloads, bitflow_downloads_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: None, // Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: None, //Some(msg.session_id),
            };

            let download_to_update: Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(msg.download_id))
                // .filter(bitflow_downloads::dsl::owner_id.eq(msg.actor_id))
                .filter(bitflow_downloads::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // name
            let download_to_update = match &msg.name {
                Some(name) if name != &download_to_update.name => {
                    let update_name_cmd = download::UpdateName {
                        name: name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let (download_to_update, event, _) =
                        eventsourcing::execute(&conn, download_to_update, &update_name_cmd)?;

                    // update download
                    diesel::update(&download_to_update)
                        .set(&download_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                        .values(&event)
                        .execute(&conn)?;
                    download_to_update
                }
                _ => download_to_update,
            };

            // progress
            let download_to_update = match msg.progress {
                Some(progress) if (progress as i32) != download_to_update.progress => {
                    let update_progress_cmd = download::UpdateProgress {
                        progress: progress,
                        metadata: metadata.clone(),
                    };

                    let (download_to_update, event, _) =
                        eventsourcing::execute(&conn, download_to_update, &update_progress_cmd)?;

                    // update download
                    diesel::update(&download_to_update)
                        .set(&download_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                        .values(&event)
                        .execute(&conn)?;
                    download_to_update
                }
                _ => download_to_update,
            };

            return Ok(download_to_update);
        })?);
    }
}
