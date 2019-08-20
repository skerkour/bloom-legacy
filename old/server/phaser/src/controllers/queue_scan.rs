use crate::domain::{report, scan, Scan};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

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
        use diesel::prelude::*;
        use kernel::db::schema::{phaser_reports, phaser_scans};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // retrieve Scan
            let scan_to_queue: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .for_update()
                .first(&conn)?;

            scan::validators::target(&scan_to_queue.targets[0].clone())?;

            // queue report
            let trigger = scan::ReportTrigger::Manual;

            let queue_cmd = report::Queue {
                scan_id: scan_to_queue.id,
                targets: scan_to_queue.targets.clone(),
                profile: scan_to_queue.profile.clone(),
                trigger: trigger.clone(),
            };
            let (queued_report, _) =
                eventsourcing::execute(&conn, report::Report::new(), &queue_cmd)?;

            diesel::insert_into(phaser_reports::dsl::phaser_reports)
                .values(&queued_report)
                .execute(&conn)?;

            // queue Scan
            let queue_cmd = scan::Queue {
                report_id: queued_report.id,
                trigger: trigger.clone(),
            };
            let (queued_scan, _) = eventsourcing::execute(&conn, scan_to_queue, &queue_cmd)?;

            diesel::update(&queued_scan)
                .set(&queued_scan)
                .execute(&conn)?;

            return Ok(queued_scan);
        })?);
    }
}
