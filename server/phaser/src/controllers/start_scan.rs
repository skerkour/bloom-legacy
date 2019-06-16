use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::{
    domain,
};


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

    fn handle(&mut self, msg: StartScan, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            phaser_scans,
            phaser_scans_events,
            phaser_reports,
            phaser_reports_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: None, // Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: None, // Some(msg.session_id),
            };

            // start report
            let report: domain::Report = phaser_reports::dsl::phaser_reports
                .filter(phaser_reports::dsl::deleted_at.is_null())
                .filter(phaser_reports::dsl::status.eq(domain::report::ReportStatus::Queued))
                .for_update()
                .first(&conn)?;

            let start_cmd = domain::report::Start{
                metadata: metadata.clone(),
            };

            let (report, event, _) = eventsourcing::execute(&conn, report, &start_cmd)?;
            diesel::update(&report)
                .set(&report)
                .execute(&conn)?;
            diesel::insert_into(phaser_reports_events::dsl::phaser_reports_events)
                .values(&event)
                .execute(&conn)?;

            // start scan
            let scan: domain::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::deleted_at.is_null())
                .filter(phaser_scans::dsl::id.eq(report.scan_id))
                .filter(phaser_scans::dsl::state.eq(domain::scan::ScanState::Queued))
                .for_update()
                .first(&conn)?;

            let start_cmd = domain::scan::Start{
                metadata: metadata.clone(),
            };

            let (scan, event, _) = eventsourcing::execute(&conn, scan, &start_cmd)?;
            diesel::update(&scan)
                .set(&scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;



            return Ok(report);
        })?);
    }
}
