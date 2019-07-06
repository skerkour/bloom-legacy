use crate::{
    config::Config, db::DbActor, error::KernelError, events::EventMetadata, myaccount::domain,
    myaccount::domain::pending_account,
    myaccount::notifications::emails::send_account_verification_code,
};
use actix::{Handler, Message};

#[derive(Clone, Debug)]
pub struct StartRegistration {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
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

            let metadata = EventMetadata {
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };

            let create_cmd = pending_account::Create {
                first_name: msg.first_name.clone(),
                last_name: msg.last_name.clone(),
                email: msg.email.clone(),
                password: msg.password.clone(),
                config: msg.config.clone(),
                metadata,
            };
            let new_pending_account = pending_account::PendingAccount::new();
            let event = eventsourcing::execute(&conn, &mut new_pending_account, &create_cmd)?;

            diesel::insert_into(kernel_pending_accounts::dsl::kernel_pending_accounts)
                .values(&new_pending_account)
                .execute(&conn)?;

            let code = if let pending_account::EventData::CreatedV1(ref data) = event.data {
                data.code.clone()
            } else {
                return Err(KernelError::Internal(String::new()));
            };

            send_account_verification_code(
                &config,
                new_pending_account.email.as_str(),
                format!(
                    "{} {}",
                    &new_pending_account.first_name, &new_pending_account.last_name
                )
                .as_str(),
                new_pending_account.id.to_string().as_str(),
                &code,
            )
            .expect("error sending email");

            return Ok(new_pending_account);
        })?);
    }
}
