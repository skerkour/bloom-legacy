use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartScan {
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for StartScan {
    type Result = Result<domain::Report, KernelError>;
}

impl Handler<StartScan> for DbActor {
    type Result = Result<domain::Report, KernelError>;

    fn handle(&mut self, _msg: StartScan, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{phaser_reports, phaser_scans};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // start report
            let report: domain::Report = phaser_reports::dsl::phaser_reports
                .filter(phaser_reports::dsl::status.eq(domain::report::ReportStatus::Queued))
                .for_update()
                .first(&conn)?;

            let start_cmd = domain::report::Start {};

            let (report, _) = eventsourcing::execute(&conn, report, &start_cmd)?;
            diesel::update(&report).set(&report).execute(&conn)?;

            // start scan
            let scan: domain::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(report.scan_id))
                .filter(phaser_scans::dsl::state.eq(domain::scan::ScanState::Queued))
                .for_update()
                .first(&conn)?;

            let start_cmd = domain::scan::Start {};

            let (scan, _) = eventsourcing::execute(&conn, scan, &start_cmd)?;
            diesel::update(&scan).set(&scan).execute(&conn)?;

            return Ok(report);
        })?);
    }
}
