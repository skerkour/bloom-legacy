use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account::domain::account,
    services::account::notifications::emails::send_password_reset,
    services::common::events::EventMetadata,
    config::Config,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct RequestPasswordReset {
    pub email_or_username: String,
    pub config: Config,
    pub request_id: String,
}

impl Message for RequestPasswordReset {
    type Result = Result<(), KernelError>;
}

impl Handler<RequestPasswordReset> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RequestPasswordReset, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_accounts,
            account_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let user: domain::Account = account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::email.eq(&msg.email_or_username).or(account_accounts::dsl::username.eq(&msg.email_or_username)))
                .filter(account_accounts::dsl::deleted_at.is_null())
                .first(&conn)?;


            let metadata = EventMetadata{
                actor_id: Some(user.id),
                request_id: Some(msg.request_id.clone()),
            };
            let request_password_reset_cmd = account::RequestPasswordReset{
                metadata,
            };
            let (user, event, non_stored) = eventsourcing::execute(&conn, user, &request_password_reset_cmd)?;

            diesel::update(account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::id.eq(user.id)))
                .set(&user)
                .execute(&conn)?;
            diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // send email
            // we can safely unwrap user.password_reset_id because it's set when applying the event to user
            send_password_reset(
                &msg.config,
                user.email.as_str(),
                format!("{} {}", &user.first_name, &user.last_name).as_str(),
                user.password_reset_id.unwrap().to_string().as_str(),
                &non_stored.cleartext_token,
            ).expect("error sending email");

            return Ok(());
        })?);
    }
}
