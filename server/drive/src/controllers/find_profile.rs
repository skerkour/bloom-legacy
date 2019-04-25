use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain;


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
        use kernel::db::schema::{
            drive_profiles,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let profile = drive_profiles::dsl::drive_profiles
            .filter(drive_profiles::dsl::account_id.eq(msg.account_id))
            .filter(drive_profiles::dsl::deleted_at.is_null())
            .first(&conn)?;

        return Ok(profile);
    }
}
