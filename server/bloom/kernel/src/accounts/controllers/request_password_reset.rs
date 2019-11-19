use crate::{
    config::Config, db::DbActor, error::KernelError, accounts::domain::account,
    accounts::notifications::emails::send_password_reset,
};
use actix::{Handler, Message};

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
        use crate::db::schema::kernel_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account_to_update: account::Account = kernel_accounts::dsl::kernel_accounts
                .filter(
                    kernel_accounts::dsl::email
                        .eq(&msg.email_or_username)
                        .or(kernel_accounts::dsl::username.eq(&msg.email_or_username)),
                )
                .for_update()
                .first(&conn)?;

            let request_password_reset_cmd = account::RequestPasswordReset {};
            let (account_to_update, event) =
                eventsourcing::execute(&conn, account_to_update, &request_password_reset_cmd)?;

            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;

            // send email
            // we can safely unwrap account.password_reset_id because it's set when applying the event to account
            send_password_reset(
                &msg.config,
                account_to_update.email.as_str(),
                format!(
                    "{} {}",
                    &account_to_update.first_name, &account_to_update.last_name
                )
                .as_str(),
                account_to_update
                    .password_reset_id
                    .unwrap()
                    .to_string()
                    .as_str(),
                &event.token_plaintext,
            )
            .expect("error sending email");

            return Ok(());
        })?);
    }
}
