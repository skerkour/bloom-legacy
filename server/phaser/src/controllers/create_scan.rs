use crate::domain::{scan, Scan};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateScan {
    pub name: String,
    pub description: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub target: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateScan {
    type Result = Result<Scan, KernelError>;
}

impl Handler<CreateScan> for DbActor {
    type Result = Result<Scan, KernelError>;

    fn handle(&mut self, msg: CreateScan, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::phaser_scans;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // create Scan
            // check the number of scans
            let number_of_scan: i64 = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::owner_id.eq(msg.account_id))
                .count()
                .get_result(&conn)?;

            if number_of_scan > 0 {
                return Err(KernelError::Validation(
                    "Please enable billing to create more scan".to_string(),
                ));
            }

            let create_cmd = scan::Create {
                name: msg.name.clone(),
                description: msg.description.clone(),
                profile: msg.profile.clone(),
                schedule: msg.schedule.clone(),
                target: msg.target.clone(),
                owner_id: msg.account_id,
            };
            let (scan, _) = eventsourcing::execute(&conn, Scan::new(), &create_cmd)?;

            diesel::insert_into(phaser_scans::dsl::phaser_scans)
                .values(&scan)
                .execute(&conn)?;

            return Ok(scan);
        })?);
    }
}
