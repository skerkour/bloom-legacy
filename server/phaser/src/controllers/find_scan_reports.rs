use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindScanReports {
    pub scan_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
}

impl Message for FindScanReports {
    type Result = Result<Vec<domain::Report>, KernelError>;
}

impl Handler<FindScanReports> for DbActor {
    type Result = Result<Vec<domain::Report>, KernelError>;

    fn handle(&mut self, msg: FindScanReports, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            phaser_reports,
            phaser_scans,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let _: domain::Scan = phaser_scans::dsl::phaser_scans
            .filter(phaser_scans::dsl::id.eq(msg.scan_id))
            .filter(phaser_scans::dsl::deleted_at.is_null())
            .filter(phaser_scans::dsl::owner_id.eq(msg.account_id))
            .first(&conn)?;

        let reports: Vec<domain::Report> = phaser_reports::dsl::phaser_reports
            .filter(phaser_reports::dsl::scan_id.eq(msg.scan_id))
            .filter(phaser_reports::dsl::deleted_at.is_null())
            .filter(phaser_reports::dsl::status.ne(domain::report::ReportStatus::Canceled))
            .order_by(phaser_reports::dsl::created_at.desc())
            .load(&conn)?;

        return Ok(reports);
    }
}
