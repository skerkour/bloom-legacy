use crate::domain::{report, scan};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

// TODO: delete all associated reports
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CancelScan {
    pub scan_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CancelScan {
    type Result = Result<(), KernelError>;
}

impl Handler<CancelScan> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: CancelScan, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{phaser_reports, phaser_scans};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // retrieve Scan
            let scan_to_cancel: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .for_update()
                .first(&conn)?;

            // cancel Scan
            let cancel_cmd = scan::Cancel {};
            let (canceled_scan, _) = eventsourcing::execute(&conn, scan_to_cancel, &cancel_cmd)?;

            diesel::update(&canceled_scan)
                .set(&canceled_scan)
                .execute(&conn)?;

            // TODO: cancel report
            let report_to_cancel: report::Report = phaser_reports::dsl::phaser_reports
                .filter(phaser_reports::dsl::scan_id.eq(msg.scan_id))
                .order(phaser_reports::dsl::created_at.desc())
                .for_update()
                .first(&conn)?;

            let cancel_cmd = report::Cancel {};
            let (cancelped_report, _) =
                eventsourcing::execute(&conn, report_to_cancel, &cancel_cmd)?;

            diesel::update(&cancelped_report)
                .set(&cancelped_report)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
