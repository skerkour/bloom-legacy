use crate::{
    config::Config,
    db::DbActor,
    error::KernelError,
    accounts::domain::{account, session, Account, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResetPassword {
    pub reset_password_id: uuid::Uuid,
    pub token: String,
    pub new_password: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
    pub ip: String,
    pub session_id: Option<uuid::Uuid>,
}

impl Message for ResetPassword {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<ResetPassword> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: ResetPassword, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{kernel_accounts, kernel_sessions};
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::password_reset_id.eq(msg.reset_password_id))
                .for_update()
                .first(&conn)?;

            let update_last_name_cmd = account::ResetPassword {
                new_password: msg.new_password,
                token: msg.token,
                config: msg.config,
            };

            let (account, _) = eventsourcing::execute(&conn, account, &update_last_name_cmd)?;

            // update account
            diesel::update(&account).set(&account).execute(&conn)?;

            // revoke all active sessions
            let sessions: Vec<Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account.id))
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: None,
                reason: session::RevokedReason::PasswordReseted,
            };

            for session in sessions {
                let (session, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::delete(&session).execute(&conn)?;
            }

            // start new session
            let start_cmd = session::Start {
                account_id: account.id,
                ip: msg.ip,                 // TODO
                user_agent: "".to_string(), // TODO
            };
            let (new_session, event) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;

            return Ok((new_session, event.token_plaintext));
        })?);
    }
}
