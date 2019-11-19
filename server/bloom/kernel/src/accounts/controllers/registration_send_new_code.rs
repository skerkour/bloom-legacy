use crate::{
    accounts::domain::{pending_account, PendingAccount},
    accounts::notifications::emails::send_account_verification_code,
    config::Config,
    db::DbActor,
};
use actix::{Handler, Message};
use bloom_error::BloomError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationSendNewCode {
    pub message: bloom_messages::auth::RegistrationSendNewCode,
    pub config: Config,
}

impl Message for RegistrationSendNewCode {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<RegistrationSendNewCode> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: RegistrationSendNewCode, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        return Ok(conn.transaction::<_, BloomError, _>(|| {
            let resend_code_cmd = pending_account::SendNewCode {};

            let pending_account: PendingAccount = pending_accounts::dsl::pending_accounts
                .filter(pending_accounts::dsl::id.eq(msg.message.id))
                .for_update()
                .first(&conn)?;

            let (pending_account, event) =
                eventsourcing::execute(&conn, pending_account, &resend_code_cmd)?;

            // update pending_account
            diesel::update(&pending_account)
                .set(&pending_account)
                .execute(&conn)?;

            let config = msg.config.clone();
            send_account_verification_code(
                &config,
                pending_account.email.as_str(),
                &pending_account.display_name,
                &event.code,
            )?;

            return Ok(bloom_messages::kernel::Empty {}.into());
        })?);
    }
}
