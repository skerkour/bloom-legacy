use actix::{Handler, Message};
use kernel::{db::DbActor, error::KernelError, myaccount::domain};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccount {
    pub actor: domain::Account,
    pub account_id: uuid::Uuid,
}

impl Message for FindAccount {
    type Result = Result<domain::Account, KernelError>;
}

impl Handler<FindAccount> for DbActor {
    type Result = Result<domain::Account, KernelError>;

    fn handle(&mut self, msg: FindAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::kernel_accounts;

        if !msg.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let account: domain::Account = kernel_accounts::dsl::kernel_accounts
            .filter(kernel_accounts::dsl::id.eq(msg.account_id))
            .first(&conn)?;

        return Ok(account);
    }
}
