use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain;


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
        use kernel::db::schema::{
            bitflow_downloads,
            bitflow_downloads_events,
        };
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let downloads: Vec<domain::Download> = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(any(&msg.downloads)))
                .filter(bitflow_downloads::dsl::owner_id.eq(msg.actor_id))
                .filter(bitflow_downloads::dsl::deleted_at.is_null())
                .filter(bitflow_downloads::dsl::removed_at.is_null())
                .load(&conn)?;

            if downloads.len() != msg.downloads.len() {
                return Err(KernelError::NotFound("downloads not found".to_string()));
            }

            let metadata = EventMetadata{
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let remove_cmd = domain::download::Remove{
                metadata,
            };

            for download in downloads {
                let (download, event, _) = eventsourcing::execute(&conn, download, &remove_cmd)?;
                diesel::insert_into(bitflow_downloads_events::dsl::bitflow_downloads_events)
                    .values(&event)
                    .execute(&conn)?;
                diesel::update(&download)
                    .set(&download)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
