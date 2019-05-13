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
    report,
};


// TODO: delete all associated reports
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteScan {
    pub scan_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for DeleteScan {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteScan> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteScan, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            phaser_scans,
            phaser_scans_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            // retrieve Scan
            let scan_to_delete: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .filter(phaser_scans::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // delete Scan
            let delete_cmd = scan::Delete{
                metadata,
            };
            let (deleted_scan, event, _) = eventsourcing::execute(&conn, scan_to_delete, &delete_cmd)?;

            diesel::update(&deleted_scan)
                .set(&deleted_scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
