use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        PendingEmail,
        pending_email,
        Account,
        account,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyEmail {
    pub account: Account,
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: String,
}

impl Message for VerifyEmail {
    type Result = Result<Account, KernelError>;
}

impl Handler<VerifyEmail> for DbActor {
    type Result = Result<Account, KernelError>;

    fn handle(&mut self, msg: VerifyEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_emails,
            account_pending_emails_events,
            account_accounts,
            account_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id.clone()),
            };

            let account_to_update = msg.account;

            let pending_email: PendingEmail = account_pending_emails::dsl::account_pending_emails
                .filter(account_pending_emails::dsl::id.eq(msg.id))
                .filter(account_pending_emails::dsl::deleted_at.is_null())
                .filter(account_pending_emails::dsl::account_id.eq(account_to_update.id))
                .first(&conn)?;

            let verify_cmd = pending_email::Verify{
                id: msg.id,
                code: msg.code.clone(),
                email: pending_email.email.clone(),
                metadata: metadata.clone(),
            };

            let (pending_email, event, _) = eventsourcing::execute(&conn, pending_email, &verify_cmd)?;

            // update pending_email
            diesel::update(account_pending_emails::dsl::account_pending_emails
                .filter(account_pending_emails::dsl::id.eq(pending_email.id)))
                .set((
                    account_pending_emails::dsl::trials.eq(pending_email.trials),
                    account_pending_emails::dsl::version.eq(pending_email.version),
                    account_pending_emails::dsl::updated_at.eq(pending_email.updated_at),
                    account_pending_emails::dsl::deleted_at.eq(pending_email.deleted_at),
                ))
                .execute(&conn)?;
            diesel::insert_into(account_pending_emails_events::dsl::account_pending_emails_events)
                .values(&event)
                .execute(&conn)?;

            let account_to_update = match event.data {
                pending_email::EventData::VerificationSucceededV1 => {
                    let update_email_cmd = account::UpdateEmail{
                        email: pending_email.email,
                        metadata: metadata.clone(),
                    };

                    let (account_to_update, event, _) = eventsourcing::execute(&conn, account_to_update, &update_email_cmd)?;

                    // update account
                    diesel::update(account_accounts::dsl::account_accounts
                        .filter(account_accounts::dsl::id.eq(account_to_update.id)))
                        .set((
                            account_accounts::dsl::version.eq(account_to_update.version),
                            account_accounts::dsl::updated_at.eq(account_to_update.updated_at),
                            account_accounts::dsl::email.eq(&account_to_update.email),
                        ))
                        .execute(&conn)?;
                    diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                        .values(&event)
                        .execute(&conn)?;

                    // delete all other pending emails for account
                    let pending_emails_to_delete: Vec<PendingEmail> = account_pending_emails::dsl::account_pending_emails
                        .filter(account_pending_emails::dsl::account_id.eq(account_to_update.id))
                        .filter(account_pending_emails::dsl::deleted_at.is_null())
                        .filter(account_pending_emails::dsl::id.ne(msg.id))
                        .load(&conn)?;

                    let delete_cmd = pending_email::Delete{
                        metadata: metadata.clone(),
                    };

                    for pending_email_to_delete in pending_emails_to_delete {
                        let (session, event, _) = eventsourcing::execute(&conn, pending_email_to_delete, &delete_cmd)?;
                        diesel::update(account_pending_emails::dsl::account_pending_emails
                            .filter(account_pending_emails::dsl::id.eq(session.id)))
                            .set((
                                account_pending_emails::dsl::version.eq(session.version),
                                account_pending_emails::dsl::updated_at.eq(session.updated_at),
                                account_pending_emails::dsl::deleted_at.eq(session.deleted_at),
                            ))
                            .execute(&conn)?;
                        diesel::insert_into(account_pending_emails_events::dsl::account_pending_emails_events)
                            .values(&event)
                            .execute(&conn)?;
                    }
                    account_to_update
                },
                _ => account_to_update,
            };

            return match event.data {
                pending_email::EventData::VerificationFailedV1(err) => Err(KernelError::Validation(err.reason)),
                _ => Ok(account_to_update),
            };
        })?);
    }
}
