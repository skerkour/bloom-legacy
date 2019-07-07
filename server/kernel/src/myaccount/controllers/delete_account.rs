use crate::error::KernelError;
use crate::{db::DbActor, myaccount::domain::account};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteAccount {
    pub account: account::Account,
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

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let delete_account_cmd = account::Delete {};

            // just pass uuid
            let _ = eventsourcing::execute(&conn, &mut msg.account, &delete_account_cmd)?;

            // update just deleted_at = chrono::Utc::now() or check that eventsourcing done that
            diesel::update(&msg.account)
                .set(&msg.account)
                .execute(&conn)?;

            Ok(())
        })?);
    }
}
