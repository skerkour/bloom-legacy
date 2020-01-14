use crate::error::KernelError;
use crate::{
    db::DbActor,
    accounts::domain::{account, pending_email, Account, PendingEmail},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyEmail {
    pub account: Account,
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for VerifyEmail {
    type Result = Result<Account, KernelError>;
}

impl Handler<VerifyEmail> for DbActor {
    type Result = Result<Account, KernelError>;

    fn handle(&mut self, msg: VerifyEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_emails;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        match conn.transaction::<_, KernelError, _>(|| {
            let account_to_update = msg.account.clone();

            let pending_email_to_verify: PendingEmail =
                kernel_pending_emails::dsl::kernel_pending_emails
                    .filter(kernel_pending_emails::dsl::id.eq(msg.id))
                    .filter(kernel_pending_emails::dsl::account_id.eq(account_to_update.id))
                    .for_update()
                    .first(&conn)?;

            let verify_cmd = pending_email::Verify {
                id: msg.id,
                code: msg.code.clone(),
                email: pending_email_to_verify.email.clone(),
            };

            let (pending_email_to_verify, _) =
                eventsourcing::execute(&conn, pending_email_to_verify.clone(), &verify_cmd)?;
            // delete pending_email
            diesel::delete(&pending_email_to_verify).execute(&conn)?;

            let update_email_cmd = account::UpdateEmail {
                email: pending_email_to_verify.email,
            };

            let (account_to_update, _) =
                eventsourcing::execute(&conn, account_to_update, &update_email_cmd)?;

            // update account
            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;

            // delete all other pending emails for account
            let pending_emails_to_delete: Vec<PendingEmail> =
                kernel_pending_emails::dsl::kernel_pending_emails
                    .filter(kernel_pending_emails::dsl::account_id.eq(account_to_update.id))
                    .filter(kernel_pending_emails::dsl::id.ne(msg.id))
                    .for_update()
                    .load(&conn)?;

            let delete_cmd = pending_email::Delete {};

            for pending_email_to_delete in pending_emails_to_delete {
                let (pending_email_to_delete, _) =
                    eventsourcing::execute(&conn, pending_email_to_delete, &delete_cmd)?;
                diesel::delete(&pending_email_to_delete).execute(&conn)?;
            }

            return Ok(account_to_update);
        }) {
            Ok(res) => return Ok(res),
            Err(err) => match err {
                KernelError::Validation(_) => {
                    let pending_email_to_verify: PendingEmail =
                        kernel_pending_emails::dsl::kernel_pending_emails
                            .filter(kernel_pending_emails::dsl::id.eq(msg.id))
                            .filter(kernel_pending_emails::dsl::account_id.eq(msg.account.id))
                            .for_update()
                            .first(&conn)?;
                    let fail_cmd = pending_email::FailVerification {};
                    let (pending_email_to_verify, _) =
                        eventsourcing::execute(&conn, pending_email_to_verify, &fail_cmd)?;
                    // update pending_email trials
                    diesel::update(&pending_email_to_verify)
                        .set(&pending_email_to_verify)
                        .execute(&conn)?;
                    return Err(err);
                }
                _ => return Err(err),
            },
        }
    }
}
