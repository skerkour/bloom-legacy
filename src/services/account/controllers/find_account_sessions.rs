use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    error::KernelError,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccountSessions {
    pub account_id: uuid::Uuid,
}

impl Message for FindAccountSessions {
    type Result = Result<Vec<domain::Session>, KernelError>;
}

impl Handler<FindAccountSessions> for DbActor {
    type Result = Result<Vec<domain::Session>, KernelError>;

    fn handle(&mut self, msg: FindAccountSessions, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_sessions,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let sessions: Vec<domain::Session> = account_sessions::dsl::account_sessions
                .filter(account_sessions::dsl::account_id.eq(msg.account_id))
                .filter(account_sessions::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(sessions);
    }
}
