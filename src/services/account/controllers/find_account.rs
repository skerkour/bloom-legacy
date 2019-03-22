use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    error::KernelError,
};
use serde::{Serialize, Deserialize};


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
        use crate::db::schema::{
            account_accounts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let account: domain::Account = account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::id.eq(msg.account_id))
                .filter(account_accounts::dsl::deleted_at.is_null())
                .first(&conn)?;

        return Ok(account);
    }
}
