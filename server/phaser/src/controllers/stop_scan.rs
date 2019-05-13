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
pub struct StopScan {
    pub scan_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for StopScan {
    type Result = Result<(), KernelError>;
}

impl Handler<StopScan> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: StopScan, _: &mut Self::Context) -> Self::Result {
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
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            // retrieve Scan
            let scan_to_stop: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .filter(phaser_scans::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // stop Scan
            let stop_cmd = scan::Stop{
                metadata: metadata.clone(),
            };
            let (stopped_scan, event, _) = eventsourcing::execute(&conn, scan_to_stop, &stop_cmd)?;

            diesel::update(&stopped_scan)
                .set(&stopped_scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;

            // TODO: stop report
            let report_to_stop: report::Report = phaser_reports::dsl::phaser_reports
                .filter(phaser_reports::dsl::scan_id.eq(msg.scan_id))
                .filter(phaser_reports::dsl::deleted_at.is_null())
                .order(phaser_reports::dsl::created_at.desc())
                .for_update()
                .first(&conn)?;

            let stop_cmd = report::Stop{
                metadata,
            };
            let (stopped_report, event, _) = eventsourcing::execute(&conn, report_to_stop, &stop_cmd)?;

            diesel::update(&stopped_report)
                .set(&stopped_report)
                .execute(&conn)?;
            diesel::insert_into(phaser_reports_events::dsl::phaser_reports_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
