use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

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

    fn handle(&mut self, _msg: StartDownload, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::bitflow_downloads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::removed_at.is_null())
                .filter(bitflow_downloads::dsl::status.eq(domain::download::DownloadStatus::Queued))
                .for_update()
                .first(&conn)?;

            let start_cmd = domain::download::Start {};

            let (download, _) = eventsourcing::execute(&conn, download, &start_cmd)?;
            diesel::update(&download).set(&download).execute(&conn)?;

            return Ok(download);
        })?);
    }
}
