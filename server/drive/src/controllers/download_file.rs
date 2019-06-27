use crate::{domain, domain::file};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadFile {
    pub file_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_region: String,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for DownloadFile {
    type Result = Result<String, KernelError>;
}

impl Handler<DownloadFile> for DbActor {
    type Result = Result<String, KernelError>;

    fn handle(&mut self, msg: DownloadFile, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, drive_files_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let file_to_download: domain::File = drive_files::dsl::drive_files
                .filter(drive_files::dsl::id.eq(msg.file_id))
                .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                .filter(drive_files::dsl::deleted_at.is_null())
                .filter(drive_files::dsl::trashed_at.is_null())
                .first(&conn)?;

            let metadata = EventMetadata {
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let download_cmd = file::Download {
                file_id: msg.file_id,
                owner_id: msg.owner_id,
                s3_bucket: msg.s3_bucket.clone(),
                s3_region: msg.s3_region.clone(),
                metadata: metadata.clone(),
            };
            let (file_to_download, event, presigned_url) =
                eventsourcing::execute(&conn, file_to_download, &download_cmd)?;
            // update file
            diesel::update(&file_to_download)
                .set(&file_to_download)
                .execute(&conn)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(presigned_url);
        })?);
    }
}
