use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindProfile {
    pub account_id: uuid::Uuid,
}

impl Message for FindProfile {
    type Result = Result<domain::Profile, KernelError>;
}

impl Handler<FindProfile> for DbActor {
    type Result = Result<domain::Profile, KernelError>;

    fn handle(&mut self, msg: FindProfile, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_profiles;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let profile = drive_profiles::dsl::drive_profiles
            .filter(drive_profiles::dsl::account_id.eq(msg.account_id))
            .first(&conn)?;

        return Ok(profile);
    }
}
