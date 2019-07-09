use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClearHistory {
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for ClearHistory {
    type Result = Result<(), KernelError>;
}

impl Handler<ClearHistory> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: ClearHistory, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::bitflow_downloads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let history: Vec<domain::Download> = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::owner_id.eq(msg.actor_id))
                .filter(bitflow_downloads::dsl::removed_at.is_not_null())
                .load(&conn)?;

            let delete_cmd = domain::download::Delete {};

            for download in history {
                let (download, _) = eventsourcing::execute(&conn, download, &delete_cmd)?;
                diesel::update(&download).set(&download).execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
