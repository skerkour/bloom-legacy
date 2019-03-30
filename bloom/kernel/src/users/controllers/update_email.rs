use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain,
    users::domain::pending_email,
    users::notifications::emails::send_email_verification_code,
    events::EventMetadata,
    config::Config,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub user: domain::User,
    pub email: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateEmail {
    type Result = Result<domain::PendingEmail, KernelError>;
}

impl Handler<UpdateEmail> for DbActor {
    type Result = Result<domain::PendingEmail, KernelError>;

    fn handle(&mut self, msg: UpdateEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_pending_emails,
            kernel_pending_emails_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();

            let metadata = EventMetadata{
                actor_id: Some(msg.user.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let new_pending_email = pending_email::PendingEmail::new();
            let create_cmd = pending_email::Create{
                email: msg.email.clone(),
                user_id: msg.user.id,
                metadata,
            };
            let (new_pending_email, event, non_persisted) = eventsourcing::execute(&conn, new_pending_email, &create_cmd)?;

            diesel::insert_into(kernel_pending_emails::dsl::kernel_pending_emails)
                .values(&new_pending_email)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_emails_events::dsl::kernel_pending_emails_events)
                .values(&event)
                .execute(&conn)?;

            send_email_verification_code(
                &config,
                new_pending_email.email.as_str(),
                format!("{} {}", &msg.user.first_name, &msg.user.last_name).as_str(),
                &new_pending_email.email,
                &non_persisted.code,
            ).expect("error sending email");

            return Ok(new_pending_email);
        })?);
    }
}
