use crate::{
    config::Config,
    db::DbActor,
    error::KernelError,
    events::EventMetadata,
    myaccount::domain::{account, session, Account, Session},
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
    pub session_id: Option<uuid::Uuid>,
}

impl Message for ResetPassword {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<ResetPassword> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: ResetPassword, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts, kernel_sessions,
        };
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::password_reset_id.eq(msg.reset_password_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let metadata = EventMetadata {
                actor_id: Some(account.id),
                request_id: Some(msg.request_id),
                session_id: msg.session_id,
            };

            let update_last_name_cmd = account::ResetPassword {
                new_password: msg.new_password,
                token: msg.token,
                config: msg.config,
                metadata: metadata.clone(),
            };

            let _ = eventsourcing::execute(&conn, &mut account, &update_last_name_cmd)?;

            // update account
            diesel::update(&account).set(&account).execute(&conn)?;

            // revoke all active sessions
            let sessions: Vec<Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account.id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: None,
                reason: session::RevokedReason::PasswordReseted,
                metadata: metadata.clone(),
            };

            for session in sessions {
                let _ = eventsourcing::execute(&conn, &mut session, &revoke_cmd)?;
                // update session
                diesel::update(&session).set(&session).execute(&conn)?;
            }

            // start new session
            let start_cmd = session::Start {
                account_id: account.id,
                ip: "127.0.0.1".to_string(), // TODO
                user_agent: "".to_string(),  // TODO
                metadata,
            };
            let new_session = Session::new();
            let event = eventsourcing::execute(&conn, &mut new_session, &start_cmd)?;

            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;

            let token_plaintext = if let session::EventData::StartedV1(ref data) = event.data {
                data.token_plaintext.clone()
            } else {
                return Err(KernelError::Internal(String::new()));
            };

            return Ok((new_session, event.token_plaintext));
        })?);
    }
}
