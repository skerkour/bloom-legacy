use crate::domain::{upload, Upload};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
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
        use kernel::db::schema::drive_uploads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if msg.file_name == crate::BLOOM_ROOT_NAME {
                return Err(KernelError::Validation(
                    "file name is not valid".to_string(),
                ));
            }
            // start Upload
            let start_cmd = upload::Start {
                file_name: msg.file_name.clone(),
                parent_id: msg.parent_id,
                owner_id: msg.account_id,
            };
            let (upload, _) = eventsourcing::execute(&conn, Upload::new(), &start_cmd)?;

            diesel::insert_into(drive_uploads::dsl::drive_uploads)
                .values(&upload)
                .execute(&conn)?;

            return Ok(upload);
        })?);
    }
}
