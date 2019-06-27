use crate::error::KernelError;
use crate::{
    db::DbActor,
    events::EventMetadata,
    myaccount::domain::{account, pending_email, Account, PendingEmail},
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
        use crate::db::schema::{
            kernel_accounts_events, kernel_pending_emails, kernel_pending_emails_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let account_to_update = msg.account;

            let pending_email: PendingEmail = kernel_pending_emails::dsl::kernel_pending_emails
                .filter(kernel_pending_emails::dsl::id.eq(msg.id))
                .filter(kernel_pending_emails::dsl::account_id.eq(account_to_update.id))
                .filter(kernel_pending_emails::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let verify_cmd = pending_email::Verify {
                id: msg.id,
                code: msg.code.clone(),
                email: pending_email.email.clone(),
                metadata: metadata.clone(),
            };

            let (pending_email, event, _) =
                eventsourcing::execute(&conn, pending_email, &verify_cmd)?;

            // update pending_email
            diesel::update(&pending_email)
                .set(&pending_email)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_emails_events::dsl::kernel_pending_emails_events)
                .values(&event)
                .execute(&conn)?;

            let account_to_update = match event.data {
                pending_email::EventData::VerificationSucceededV1 => {
                    let update_email_cmd = account::UpdateEmail {
                        email: pending_email.email,
                        metadata: metadata.clone(),
                    };

                    let (account_to_update, event, _) =
                        eventsourcing::execute(&conn, account_to_update, &update_email_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                        .values(&event)
                        .execute(&conn)?;

                    // delete all other pending emails for account
                    let pending_emails_to_delete: Vec<PendingEmail> =
                        kernel_pending_emails::dsl::kernel_pending_emails
                            .filter(kernel_pending_emails::dsl::account_id.eq(account_to_update.id))
                            .filter(kernel_pending_emails::dsl::id.ne(msg.id))
                            .filter(kernel_pending_emails::dsl::deleted_at.is_null())
                            .for_update()
                            .load(&conn)?;

                    let delete_cmd = pending_email::Delete {
                        metadata: metadata.clone(),
                    };

                    for pending_email_to_delete in pending_emails_to_delete {
                        let (pending_email_to_delete, event, _) =
                            eventsourcing::execute(&conn, pending_email_to_delete, &delete_cmd)?;
                        diesel::update(&pending_email_to_delete)
                            .set(&pending_email_to_delete)
                            .execute(&conn)?;
                        diesel::insert_into(
                            kernel_pending_emails_events::dsl::kernel_pending_emails_events,
                        )
                        .values(&event)
                        .execute(&conn)?;
                    }
                    account_to_update
                }
                _ => account_to_update,
            };

            return match event.data {
                pending_email::EventData::VerificationFailedV1(err) => {
                    Err(KernelError::Validation(err.to_string()))
                }
                _ => Ok(account_to_update),
            };
        })?);
    }
}
