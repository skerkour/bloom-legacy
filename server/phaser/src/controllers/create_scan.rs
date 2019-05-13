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
};


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
        use kernel::db::schema::{
            phaser_scans,
            phaser_scans_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Scan
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = scan::Create{
                name: msg.name.clone(),
                description: msg.description.clone(),
                profile: msg.profile.clone(),
                schedule: msg.schedule.clone(),
                target: msg.target.clone(),
                owner_id: msg.account_id,
                metadata,
            };
            let (scan, event, _) = eventsourcing::execute(&conn, Scan::new(), &create_cmd)?;

            diesel::insert_into(phaser_scans::dsl::phaser_scans)
                .values(&scan)
                .execute(&conn)?;
            diesel::insert_into(phaser_scans_events::dsl::phaser_scans_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(scan);
        })?);
    }
}
