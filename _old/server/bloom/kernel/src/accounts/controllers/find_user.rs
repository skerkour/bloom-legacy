use crate::{db::DbActor, error::KernelError, accounts::domain};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindUser {
    pub username: String,
}

impl Message for FindUser {
    type Result = Result<domain::Account, KernelError>;
}

impl Handler<FindUser> for DbActor {
    type Result = Result<domain::Account, KernelError>;

    fn handle(&mut self, msg: FindUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let account: domain::Account = kernel_accounts::dsl::kernel_accounts
            .filter(kernel_accounts::dsl::username.eq(msg.username))
            .filter(kernel_accounts::dsl::disabled_at.is_null())
            .first(&conn)?;

        return Ok(account);
    }
}
