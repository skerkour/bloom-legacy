use crate::{
    config::Config,
    db::DbActor,
    error::KernelError,
    accounts::domain::{account, session, Account, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePassword {
    pub account: Account,
    pub current_session: Session,
    pub current_password: String,
    pub new_password: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
}

impl Message for UpdatePassword {
    type Result = Result<(), KernelError>;
}

impl Handler<UpdatePassword> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: UpdatePassword, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_sessions;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account_to_update = msg.account;

            let update_last_name_cmd = account::UpdatePassword {
                current_password: msg.current_password,
                new_password: msg.new_password,
                config: msg.config,
            };

            let (account_to_update, _) =
                eventsourcing::execute(&conn, account_to_update, &update_last_name_cmd)?;

            // update account
            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;

            // revoke all other active sessions
            let sessions: Vec<Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account_to_update.id))
                .filter(kernel_sessions::dsl::id.ne(msg.current_session.id))
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: Some(msg.current_session.id),
                reason: session::RevokedReason::PasswordUpdated,
            };

            for session in sessions {
                let (session, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::delete(&session).execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
