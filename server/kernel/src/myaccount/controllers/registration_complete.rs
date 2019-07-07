use crate::{
    config::Config,
    db::DbActor,
    error::KernelError,
    myaccount::domain::{account, pending_account, session, Account, PendingAccount, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};
use eventsourcing::{Event, EventTs};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: uuid::Uuid,
    pub username: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteRegistration {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending account
        use crate::db::schema::{kernel_accounts, kernel_pending_accounts, kernel_sessions};
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let pending_account_to_update: PendingAccount =
                kernel_pending_accounts::dsl::kernel_pending_accounts
                    .filter(kernel_pending_accounts::dsl::id.eq(msg.id))
                    .filter(kernel_pending_accounts::dsl::deleted_at.is_null())
                    .for_update()
                    .first(&conn)?;

            // complete registration
            let complete_registration_cmd = pending_account::Delete {};
            let _ = eventsourcing::execute(
                &conn,
                &mut pending_account_to_update,
                &complete_registration_cmd,
            )?;

            diesel::update(&pending_account_to_update)
                .set(&pending_account_to_update)
                .execute(&conn)?;

            // create account
            let create_cmd = account::Create {
                first_name: pending_account_to_update.first_name.clone(),
                last_name: pending_account_to_update.last_name.clone(),
                email: pending_account_to_update.email.clone(),
                password: pending_account_to_update.password.clone(),
                username: msg.username.clone(),
                host: msg.config.host,
            };
            let new_account = Account::new();
            let event = eventsourcing::execute(&conn, &mut new_account, &create_cmd)?;

            diesel::insert_into(kernel_accounts::dsl::kernel_accounts)
                .values(&new_account)
                .execute(&conn)?;

            eventsourcing::publish::<_, _, KernelError>(&conn, &event)?;

            // start Session
            let start_cmd = session::Start {
                account_id: new_account.id,
                ip: "127.0.0.1".to_string(), // TODO
                user_agent: "".to_string(),  // TODO
            };

            let (new_session, event) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;
            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;

            return Ok((new_session, event.token_plaintext));
        })?);
    }
}
