use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use crate::{
    db::DbActor,
    accounts::domain,
    error::KernelError,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindSessionsForAccount {
    pub account_id: uuid::Uuid,
}

impl Message for FindSessionsForAccount {
    type Result = Result<Vec<domain::Session>, KernelError>;
}

impl Handler<FindSessionsForAccount> for DbActor {
    type Result = Result<Vec<domain::Session>, KernelError>;

    fn handle(&mut self, msg: FindSessionsForAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_sessions,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let sessions: Vec<domain::Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(msg.account_id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(sessions);
    }
}
