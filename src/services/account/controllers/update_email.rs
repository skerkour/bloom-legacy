use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account::domain::pending_email,
    services::account::notifications::emails::send_email_verification_code,
    services::common::events::EventMetadata,
    config::Config,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub account: domain::Account,
    pub email: String,
    pub config: Config,
    pub request_id: String,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateEmail {
    type Result = Result<domain::PendingEmail, KernelError>;
}

impl Handler<UpdateEmail> for DbActor {
    type Result = Result<domain::PendingEmail, KernelError>;

    fn handle(&mut self, msg: UpdateEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_emails,
            account_pending_emails_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();

            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id.clone()),
                session_id: Some(msg.session_id),
            };
            let new_pending_email = pending_email::PendingEmail::new();
            let create_cmd = pending_email::Create{
                email: msg.email.clone(),
                account_id: msg.account.id,
                metadata,
            };
            let (new_pending_email, event, non_persisted) = eventsourcing::execute(&conn, new_pending_email, &create_cmd)?;

            diesel::insert_into(account_pending_emails::dsl::account_pending_emails)
                .values(&new_pending_email)
                .execute(&conn)?;
            diesel::insert_into(account_pending_emails_events::dsl::account_pending_emails_events)
                .values(&event)
                .execute(&conn)?;

            send_email_verification_code(
                &config,
                new_pending_email.email.as_str(),
                format!("{} {}", &msg.account.first_name, &msg.account.last_name).as_str(),
                &new_pending_email.email,
                &non_persisted.code,
            ).expect("error sending email");

            return Ok(new_pending_email);
        })?);
    }
}
