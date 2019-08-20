use crate::domain::{download, Download};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

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
        use diesel::prelude::*;
        use kernel::db::schema::{bitflow_downloads, drive_profiles};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // create Download
            let profile: drive::domain::Profile = drive_profiles::dsl::drive_profiles
                .filter(drive_profiles::dsl::account_id.eq(msg.account_id))
                .first(&conn)?;

            let active_downloads: i64 = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::owner_id.eq(msg.account_id))
                .filter(bitflow_downloads::dsl::status.ne(download::DownloadStatus::Stopped))
                .filter(bitflow_downloads::dsl::status.ne(download::DownloadStatus::Success))
                .filter(bitflow_downloads::dsl::status.ne(download::DownloadStatus::Failed))
                .count()
                .get_result(&conn)?;

            if active_downloads > 4 {
                return Err(KernelError::Validation(
                    "Please update your subscription to create more parallel downloads".to_string(),
                ));
            }

            if profile.used_space >= profile.total_space {
                return Err(KernelError::Validation(
                    "No space available in your drive".to_string(),
                ));
            }

            let queue_cmd = download::Queue {
                url: msg.url,
                owner_id: msg.account_id,
            };
            let (download, _) = eventsourcing::execute(&conn, Download::new(), &queue_cmd)?;

            diesel::insert_into(bitflow_downloads::dsl::bitflow_downloads)
                .values(&download)
                .execute(&conn)?;

            return Ok(download);
        })?);
    }
}
