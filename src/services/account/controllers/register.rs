use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account,
    services::account::domain::pending_account,
    services::account::notifications::emails::send_account_verification_code,
    services::common::events::EventMetadata,
    config::Config,
    services::common::utils,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub config: Config,
    pub request_id: String,
    // pub logger: RequestLogger,
}

impl Message for Register {
    type Result = Result<domain::PendingAccount, KernelError>;
}

impl Handler<Register> for DbActor {
    type Result = Result<domain::PendingAccount, KernelError>;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_accounts,
            account_pending_accounts_events,
        };
        use diesel::RunQueryDsl;
        use diesel::Connection;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();

            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id.clone()),
            };
            let new_pending_account = pending_account::PendingAccount::new();
            let create_cmd = pending_account::Create{
                first_name: msg.first_name.clone(),
                last_name: msg.last_name.clone(),
                email: msg.email.clone(),
                password: msg.password.clone(),
                metadata,
            };
            let (new_pending_account, event, non_persisted) = eventsourcing::execute(&conn, new_pending_account, &create_cmd)?;

            diesel::insert_into(account_pending_accounts::dsl::account_pending_accounts)
                .values(&new_pending_account)
                .execute(&conn)?;
            diesel::insert_into(account_pending_accounts_events::dsl::account_pending_accounts_events)
                .values(&event)
                .execute(&conn)?;

            send_account_verification_code(
                &config,
                new_pending_account.email.as_str(),
                format!("{} {}", &new_pending_account.first_name, &new_pending_account.last_name).as_str(),
                new_pending_account.id.to_string().as_str(),
                &non_persisted.code,
            ).expect("error sending email");

            return Ok(new_pending_account);
        })?);
    }
}
