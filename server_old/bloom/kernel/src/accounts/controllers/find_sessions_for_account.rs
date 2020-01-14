use crate::{db::DbActor, error::KernelError, accounts::domain};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

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
        use crate::db::schema::kernel_sessions;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let sessions: Vec<domain::Session> = kernel_sessions::dsl::kernel_sessions
            .filter(kernel_sessions::dsl::account_id.eq(msg.account_id))
            .load(&conn)?;

        return Ok(sessions);
    }
}
