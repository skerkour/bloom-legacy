use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use crate::{
    db::DbActor,
    users::domain,
    error::KernelError,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindUser {
    pub user_id: uuid::Uuid,
}

impl Message for FindUser {
    type Result = Result<domain::User, KernelError>;
}

impl Handler<FindUser> for DbActor {
    type Result = Result<domain::User, KernelError>;

    fn handle(&mut self, msg: FindUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_users,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let account: domain::User = kernel_users::dsl::kernel_users
                .filter(kernel_users::dsl::id.eq(msg.user_id))
                .filter(kernel_users::dsl::deleted_at.is_null())
                .first(&conn)?;

        return Ok(account);
    }
}
