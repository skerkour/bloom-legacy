use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindScans {
    pub account_id: uuid::Uuid,
}

impl Message for FindScans {
    type Result = Result<Vec<domain::Scan>, KernelError>;
}

impl Handler<FindScans> for DbActor {
    type Result = Result<Vec<domain::Scan>, KernelError>;

    fn handle(&mut self, msg: FindScans, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::phaser_scans;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let scans: Vec<domain::Scan> = phaser_scans::dsl::phaser_scans
            .filter(phaser_scans::dsl::owner_id.eq(msg.account_id))
            .load(&conn)?;

        return Ok(scans);
    }
}
