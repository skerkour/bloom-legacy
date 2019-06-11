use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use crate::{
    db::DbActor,
    myaccount::domain,
    error::KernelError,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccountAdmin {
    pub actor: domain::Account,
    pub account_id: uuid::Uuid,
}

impl Message for FindAccountAdmin {
    type Result = Result<domain::Account, KernelError>;
}

impl Handler<FindAccountAdmin> for DbActor {
    type Result = Result<domain::Account, KernelError>;

    fn handle(&mut self, msg: FindAccountAdmin, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts,
        };
        use diesel::prelude::*;

        if !msg.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let account: domain::Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .first(&conn)?;

        return Ok(account);
    }
}
