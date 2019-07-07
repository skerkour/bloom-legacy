use actix::{Handler, Message};
use kernel::{
    db::DbActor,
    error::KernelError,
    myaccount::domain::{account, Account},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteAccount {
    pub actor: Account,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::kernel_accounts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if !msg.actor.is_admin {
                return Err(KernelError::Forbidden("Admin role is required".to_string()));
            }

            let account_to_delete: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let delete_cmd = account::Delete { actor: msg.actor };

            let (account_to_delete, _) =
                eventsourcing::execute(&conn, account_to_delete, &delete_cmd)?;
            diesel::update(&account_to_delete)
                .set(&account_to_delete)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
