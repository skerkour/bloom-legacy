use crate::{
    config::Config, db::DbActor, error::KernelError, accounts::domain,
    accounts::domain::pending_email,
    accounts::notifications::emails::send_email_verification_code,
};
use actix::{Handler, Message};

#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub account: domain::Account,
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
        use crate::db::schema::kernel_pending_emails;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();
            let create_cmd = pending_email::Create {
                email: msg.email.clone(),
                account_id: msg.account.id,
                config: msg.config.clone(),
            };
            let (new_pending_email, event) =
                eventsourcing::execute(&conn, pending_email::PendingEmail::new(), &create_cmd)?;

            diesel::insert_into(kernel_pending_emails::dsl::kernel_pending_emails)
                .values(&new_pending_email)
                .execute(&conn)?;

            send_email_verification_code(
                &config,
                new_pending_email.email.as_str(),
                format!("{} {}", &msg.account.first_name, &msg.account.last_name).as_str(),
                &new_pending_email.email,
                &event.code,
            )?;

            return Ok(new_pending_email);
        })?);
    }
}
