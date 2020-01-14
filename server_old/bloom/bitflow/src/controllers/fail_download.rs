use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};

#[derive(Clone)]
pub struct FailDownload {
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub error: String,
    pub download_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for FailDownload {
    type Result = Result<domain::Download, KernelError>;
}

impl Handler<FailDownload> for DbActor {
    type Result = Result<domain::Download, KernelError>;

    fn handle(&mut self, msg: FailDownload, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::bitflow_downloads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(msg.download_id))
                .first(&conn)?;

            let fail_cmd = domain::download::Fail { error: msg.error };

            let (download, _) = eventsourcing::execute(&conn, download, &fail_cmd)?;
            diesel::update(&download).set(&download).execute(&conn)?;

            return Ok(download);
        })?);
    }
}
