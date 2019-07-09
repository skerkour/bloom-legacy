use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveDownloads {
    pub downloads: Vec<uuid::Uuid>,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RemoveDownloads {
    type Result = Result<(), KernelError>;
}

impl Handler<RemoveDownloads> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RemoveDownloads, _: &mut Self::Context) -> Self::Result {
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;
        use kernel::db::schema::bitflow_downloads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let downloads: Vec<domain::Download> = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(any(&msg.downloads)))
                .filter(bitflow_downloads::dsl::owner_id.eq(msg.actor_id))
                .filter(bitflow_downloads::dsl::removed_at.is_null())
                .load(&conn)?;

            if downloads.len() != msg.downloads.len() {
                return Err(KernelError::NotFound("downloads not found".to_string()));
            }

            let remove_cmd = domain::download::Remove {};

            for download in downloads {
                let (download, _) = eventsourcing::execute(&conn, download, &remove_cmd)?;
                diesel::update(&download).set(&download).execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
