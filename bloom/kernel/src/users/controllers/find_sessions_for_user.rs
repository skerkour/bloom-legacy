use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use crate::{
    db::DbActor,
    users::domain,
    error::KernelError,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindSessionsForUser {
    pub user_id: uuid::Uuid,
}

impl Message for FindSessionsForUser {
    type Result = Result<Vec<domain::Session>, KernelError>;
}

impl Handler<FindSessionsForUser> for DbActor {
    type Result = Result<Vec<domain::Session>, KernelError>;

    fn handle(&mut self, msg: FindSessionsForUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_sessions,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let sessions: Vec<domain::Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::user_id.eq(msg.user_id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(sessions);
    }
}
