use crate::error::KernelError;
use crate::{
    db::DbActor,
    messages,
    messages::kernel::NoData,
    myaccount::domain::{pending_account, PendingAccount},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationVerify {
    pub message: messages::auth::RegistrationVerify,
}

impl Message for RegistrationVerify {
    type Result = Result<messages::Message, KernelError>;
}

impl Handler<RegistrationVerify> for DbActor {
    type Result = Result<messages::Message, KernelError>;

    fn handle(&mut self, msg: RegistrationVerify, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        match conn.transaction::<_, KernelError, _>(|| {
            let verify_cmd = pending_account::Verify {
                id: msg.message.id,
                code: msg.message.code.clone(),
            };

            let pending_account_to_verify: PendingAccount =
                kernel_pending_accounts::dsl::kernel_pending_accounts
                    .filter(kernel_pending_accounts::dsl::id.eq(msg.message.id))
                    .for_update()
                    .first(&conn)?;

            let (pending_account_to_verify, _) =
                eventsourcing::execute(&conn, pending_account_to_verify.clone(), &verify_cmd)?;

            // update pending_account
            diesel::update(&pending_account_to_verify)
                .set(&pending_account_to_verify)
                .execute(&conn)?;

            return Ok(messages::Message::from(NoData {}));
        }) {
            Ok(_) => return Ok(NoData {}.into()),
            Err(err) => match err {
                KernelError::Validation(_) => {
                    let pending_account_to_verify: PendingAccount =
                        kernel_pending_accounts::dsl::kernel_pending_accounts
                            .filter(kernel_pending_accounts::dsl::id.eq(msg.message.id))
                            .for_update()
                            .first(&conn)?;
                    let fail_cmd = pending_account::FailVerification {};
                    let (pending_account_to_verify, _) =
                        eventsourcing::execute(&conn, pending_account_to_verify, &fail_cmd)?;
                    // update pending_account_to_verify trials
                    diesel::update(&pending_account_to_verify)
                        .set(&pending_account_to_verify)
                        .execute(&conn)?;
                    return Err(err);
                }
                _ => return Err(err),
            },
        }
    }
}
