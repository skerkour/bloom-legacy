use crate::domain::scan;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

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
        use diesel::prelude::*;
        use kernel::db::schema::phaser_scans;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // retrieve Scan
            let scan_to_delete: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .for_update()
                .first(&conn)?;

            // delete Scan
            let delete_cmd = scan::Delete {};
            let (deleted_scan, _) = eventsourcing::execute(&conn, scan_to_delete, &delete_cmd)?;

            diesel::delete(&deleted_scan).execute(&conn)?;

            return Ok(());
        })?);
    }
}
