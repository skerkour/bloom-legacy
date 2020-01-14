use crate::{db::DbActor, error::KernelError, accounts::domain};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccount {
    pub account_id: uuid::Uuid,
}

impl Message for FindAccount {
    type Result = Result<domain::Account, KernelError>;
}

impl Handler<FindAccount> for DbActor {
    type Result = Result<domain::Account, KernelError>;

    fn handle(&mut self, msg: FindAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let account: domain::Account = kernel_accounts::dsl::kernel_accounts
            .filter(kernel_accounts::dsl::id.eq(msg.account_id))
            .first(&conn)?;

        return Ok(account);
    }
}
