use crate::domain::{upload, Upload};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUpload {
    pub file_name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for StartUpload {
    type Result = Result<Upload, KernelError>;
}

impl Handler<StartUpload> for DbActor {
    type Result = Result<Upload, KernelError>;

    fn handle(&mut self, msg: StartUpload, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{drive_uploads, drive_uploads_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // start Upload
            let metadata = EventMetadata {
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let start_cmd = upload::Start {
                file_name: msg.file_name.clone(),
                parent_id: msg.parent_id,
                owner_id: msg.account_id,
                metadata,
            };
            let (upload, event, _) = eventsourcing::execute(&conn, Upload::new(), &start_cmd)?;

            diesel::insert_into(drive_uploads::dsl::drive_uploads)
                .values(&upload)
                .execute(&conn)?;
            diesel::insert_into(drive_uploads_events::dsl::drive_uploads_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(upload);
        })?);
    }
}
