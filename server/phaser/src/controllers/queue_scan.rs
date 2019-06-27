use crate::{
    domain::{report, scan, Scan},
    validators,
};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
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
        use kernel::db::schema::{
            phaser_reports, phaser_reports_events, phaser_scans, phaser_scans_events,
        };

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            // retrieve Scan
            let scan_to_queue: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(msg.scan_id))
                .filter(phaser_scans::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            validators::target(&scan_to_queue.targets[0].clone())?;

            // queue report
            let trigger = scan::ReportTrigger::Manual;

            let queue_cmd = report::Queue {
                scan_id: scan_to_queue.id,
                targets: scan_to_queue.targets.clone(),
                profile: scan_to_queue.profile.clone(),
                trigger: trigger.clone(),
                metadata: metadata.clone(),
            };
            let (queued_report, event, _) =
                eventsourcing::execute(&conn, report::Report::new(), &queue_cmd)?;

            diesel::insert_into(phaser_reports::dsl::phaser_reports)
                .values(&queued_report)
                .execute(&conn)?;
            diesel::insert_into(phaser_reports_events::dsl::phaser_reports_events)
                .values(&event)
                .execute(&conn)?;

            // queue Scan
            let queue_cmd = scan::Queue {
                report_id: queued_report.id,
                trigger: trigger.clone(),
                metadata,
            };
            let (queued_scan, event, _) = eventsourcing::execute(&conn, scan_to_queue, &queue_cmd)?;

            diesel::update(&queued_scan)
                .set(&queued_scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(queued_scan);
        })?);
    }
}
