use actix::{Message, Handler};
use crate::{
    db::DbActor,
    accounts::domain::accounts,
    accounts::notifications::emails::send_password_reset,
    events::EventMetadata,
    config::Config,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct RequestPasswordReset {
    pub email_or_username: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
    pub session_id: Option<uuid::Uuid>,
}

impl Message for RequestPasswordReset {
    type Result = Result<(), KernelError>;
}

impl Handler<RequestPasswordReset> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RequestPasswordReset, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts,
            kernel_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account: accounts::Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::email.eq(&msg.email_or_username)
                    .or(kernel_accounts::dsl::username.eq(&msg.email_or_username))
                )
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;


            let metadata = EventMetadata{
                actor_id: Some(account.id),
                request_id: Some(msg.request_id),
                session_id: msg.session_id,
            };
            let request_password_reset_cmd = accounts::RequestPasswordReset{
                metadata,
            };
            let (account, event, non_stored) = eventsourcing::execute(&conn, account, &request_password_reset_cmd)?;

            diesel::update(&account)
                .set(&account)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // send email
            // we can safely unwrap account.password_reset_id because it's set when applying the event to account
            send_password_reset(
                &msg.config,
                account.email.as_str(),
                format!("{} {}", &account.first_name, &account.last_name).as_str(),
                account.password_reset_id.unwrap().to_string().as_str(),
                &non_stored.plaintext_token,
            ).expect("error sending email");

            return Ok(());
        })?);
    }
}
