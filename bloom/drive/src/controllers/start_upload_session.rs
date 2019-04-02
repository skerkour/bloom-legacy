use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    UploadSession,
    upload_session,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUploadSession {
    pub file_name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub s3_bucket: String,
    pub s3_region: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for StartUploadSession {
    type Result = Result<UploadSession, KernelError>;
}

impl Handler<StartUploadSession> for DbActor {
    type Result = Result<UploadSession, KernelError>;

    fn handle(&mut self, msg: StartUploadSession, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_upload_sessions,
            drive_upload_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Note
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let start_cmd = upload_session::Start{
                file_name: msg.file_name.clone(),
                parent_id: msg.parent_id,
                owner_id: msg.account_id,
                s3_bucket: msg.s3_bucket.clone(),
                s3_region: msg.s3_region.clone(),
                metadata,
            };
            let (upload_session, event, _) = eventsourcing::execute(&conn, UploadSession::new(), &start_cmd)?;

            diesel::insert_into(drive_upload_sessions::dsl::drive_upload_sessions)
                .values(&upload_session)
                .execute(&conn)?;
            diesel::insert_into(drive_upload_sessions_events::dsl::drive_upload_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(upload_session);
        })?);
    }
}
