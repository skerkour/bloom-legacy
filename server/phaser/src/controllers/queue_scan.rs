use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    Scan,
    scan,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueScan {
    pub scan_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for QueueScan {
    type Result = Result<Scan, KernelError>;
}

impl Handler<QueueScan> for DbActor {
    type Result = Result<Scan, KernelError>;

    fn handle(&mut self, msg: QueueScan, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            phaser_scans,
            phaser_scans_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // queue Scan
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let scan_to_queue: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .filter(phaser_scans::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let queue_cmd = scan::Queue{
                metadata,
            };
            let (scan, event, _) = eventsourcing::execute(&conn, scan_to_queue, &queue_cmd)?;

            diesel::insert_into(phaser_scans::dsl::phaser_scans)
                .values(&scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(scan);
        })?);
    }
}
