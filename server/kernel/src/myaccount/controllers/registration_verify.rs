use crate::error::KernelError;
use crate::{
    db::DbActor,
    myaccount::domain::{pending_account, PendingAccount},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyPendingAccount {
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: uuid::Uuid,
}

impl Message for VerifyPendingAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<VerifyPendingAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: VerifyPendingAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let verify_cmd = pending_account::Verify {
                id: msg.id,
                code: msg.code.clone(),
            };

            let pending_account_to_verify: PendingAccount =
                kernel_pending_accounts::dsl::kernel_pending_accounts
                    .filter(kernel_pending_accounts::dsl::id.eq(msg.id))
                    .for_update()
                    .first(&conn)?;

            let pending_account_to_verify =
                match eventsourcing::execute(&conn, pending_account_to_verify.clone(), &verify_cmd)
                {
                    Ok((pending_account_to_verify, _)) => pending_account_to_verify,
                    Err(err) => match err {
                        KernelError::Validation(_) => {
                            let fail_cmd = pending_account::FailVerification {};
                            let (pending_account_to_verify, _) = eventsourcing::execute(
                                &conn,
                                pending_account_to_verify,
                                &fail_cmd,
                            )?;
                            pending_account_to_verify
                        }
                        _ => return Err(err),
                    },
                };

            // update pending_account
            diesel::update(&pending_account_to_verify)
                .set(&pending_account_to_verify)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
