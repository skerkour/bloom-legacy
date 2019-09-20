use actix::{Handler, Message};
use kernel::{
    db::DbActor,
    error::KernelError,
    myaccount::domain::{account, Account},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnableAccount {
    pub actor: Account,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for EnableAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<EnableAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: EnableAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::kernel_accounts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if !msg.actor.is_admin {
                return Err(KernelError::Forbidden("Admin role is required".to_string()));
            }

            let account_to_disable: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let disable_cmd = account::Enable { actor: msg.actor };

            let (account_to_disable, _) =
                eventsourcing::execute(&conn, account_to_disable, &disable_cmd)?;
            diesel::update(&account_to_disable)
                .set(&account_to_disable)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
