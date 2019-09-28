use crate::error::KernelError;
use crate::{
    config::Config,
    db::DbActor,
    messages,
    myaccount::domain::{pending_account, PendingAccount},
    myaccount::notifications::emails::send_account_verification_code,
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationSendNewCode {
    pub message: messages::auth::RegistrationSendNewCode,
    pub config: Config,
}

impl Message for RegistrationSendNewCode {
    type Result = Result<messages::Message, KernelError>;
}

impl Handler<RegistrationSendNewCode> for DbActor {
    type Result = Result<messages::Message, KernelError>;

    fn handle(&mut self, msg: RegistrationSendNewCode, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let resend_code_cmd = pending_account::SendNewCode {};

            let pending_account: PendingAccount =
                kernel_pending_accounts::dsl::kernel_pending_accounts
                    .filter(kernel_pending_accounts::dsl::id.eq(msg.message.id))
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
                pending_account.id.to_string().as_str(),
                &event.code,
            )
            .expect("error sending email");

            return Ok(messages::kernel::NoData {}.into());
        })?);
    }
}
