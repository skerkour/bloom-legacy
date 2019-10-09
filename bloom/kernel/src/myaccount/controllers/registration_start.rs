use crate::{
    config::Config, db::DbActor, myaccount::domain::pending_account,
    myaccount::notifications::emails::send_account_verification_code,
};
use actix::{Handler, Message};
use bloom_error::BloomError;

#[derive(Clone, Debug)]
pub struct StartRegistration {
    pub message: bloom_messages::auth::RegistrationStart,
    pub config: Config,
}

impl Message for StartRegistration {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<StartRegistration> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: StartRegistration, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        return Ok(conn.transaction::<_, BloomError, _>(|| {
            let config = msg.config.clone();

            let create_cmd = pending_account::Create {
                display_name: msg.message.display_name.clone(),
                email: msg.message.email.clone(),
                // auth_key: msg.message.auth_key.clone(),
                config: msg.config.clone(),
            };
            let (new_pending_account, event) =
                eventsourcing::execute(&conn, pending_account::PendingAccount::new(), &create_cmd)?;

            diesel::insert_into(kernel_pending_accounts::dsl::kernel_pending_accounts)
                .values(&new_pending_account)
                .execute(&conn)?;

            send_account_verification_code(
                &config,
                new_pending_account.email.as_str(),
                &msg.message.display_name,
                &event.verification_code,
            )?;

            return Ok(bloom_messages::auth::RegistrationStarted {
                id: new_pending_account.id,
            }
            .into());
        })?);
    }
}
