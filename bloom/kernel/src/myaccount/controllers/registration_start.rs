use crate::{
    config::Config, db::DbActor, error::KernelError, myaccount::domain,
    myaccount::domain::pending_account,
    myaccount::notifications::emails::send_account_verification_code,
};
use actix::{Handler, Message};

#[derive(Clone, Debug)]
pub struct StartRegistration {
    pub display_name: String,
    pub email: String,
    pub auth_key: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
}

impl Message for StartRegistration {
    type Result = Result<domain::PendingAccount, KernelError>;
}

impl Handler<StartRegistration> for DbActor {
    type Result = Result<domain::PendingAccount, KernelError>;

    fn handle(&mut self, msg: StartRegistration, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();

            let create_cmd = pending_account::Create {
                display_name: msg.display_name.clone(),
                email: msg.email.clone(),
                auth_key: msg.auth_key.clone(),
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
                &msg.display_name,
                new_pending_account.id.to_string().as_str(),
                &event.verification_code,
            )
            .expect("error sending email");

            return Ok(new_pending_account);
        })?);
    }
}
