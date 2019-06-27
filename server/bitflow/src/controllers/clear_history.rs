use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
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
        use kernel::db::schema::{bitflow_downloads, bitflow_downloads_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let history: Vec<domain::Download> = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::owner_id.eq(msg.actor_id))
                .filter(bitflow_downloads::dsl::deleted_at.is_null())
                .filter(bitflow_downloads::dsl::removed_at.is_not_null())
                .load(&conn)?;

            let metadata = EventMetadata {
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let delete_cmd = domain::download::Delete { metadata };

            for download in history {
                let (download, event, _) = eventsourcing::execute(&conn, download, &delete_cmd)?;
                diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                    .values(&event)
                    .execute(&conn)?;
                diesel::update(&download).set(&download).execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
