use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindHistory {
    pub account_id: uuid::Uuid,
}

impl Message for FindHistory {
    type Result = Result<Vec<domain::Download>, KernelError>;
}

impl Handler<FindHistory> for DbActor {
    type Result = Result<Vec<domain::Download>, KernelError>;

    fn handle(&mut self, msg: FindHistory, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::bitflow_downloads;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let downloads: Vec<domain::Download> = bitflow_downloads::dsl::bitflow_downloads
            .filter(bitflow_downloads::dsl::owner_id.eq(msg.account_id))
            .filter(bitflow_downloads::dsl::removed_at.is_not_null())
            .load(&conn)?;

        return Ok(downloads);
    }
}
