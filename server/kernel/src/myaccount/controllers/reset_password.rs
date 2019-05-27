use actix::{Message, Handler};
use crate::{
    db::DbActor,
    myaccount::domain::{
        Account,
        account,
        Session,
        session,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResetPassword {
    pub reset_password_id: uuid::Uuid,
    pub token: String,
    pub new_password: String,
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
            kernel_accounts,
            kernel_accounts_events,
            kernel_sessions,
            kernel_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::password_reset_id.eq(msg.reset_password_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: Some(account.id),
                request_id: Some(msg.request_id),
                session_id: msg.session_id,
            };

            let update_last_name_cmd = account::ResetPassword{
                new_password: msg.new_password,
                token: msg.token,
                metadata: metadata.clone(),
            };

            let (account, event, _) = eventsourcing::execute(&conn, account, &update_last_name_cmd)?;

            // update account
            diesel::update(&account)
                .set(&account)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // revoke all active sessions
            let sessions: Vec<Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account.id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke{
                current_session_id: None,
                reason: session::RevokedReason::PasswordReseted,
                metadata: metadata.clone(),
            };

            for session in sessions {
                let (session, event, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::update(&session)
                    .set(&session)
                    .execute(&conn)?;
                diesel::insert_into(kernel_sessions_events::dsl::kernel_sessions_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            // start new session
            let start_cmd = session::Start{
                account_id: account.id,
                ip: "127.0.0.1".to_string(), // TODO
                user_agent: "".to_string(), // TODO
                metadata,
            };
            let (new_session, event, non_stored) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;
            diesel::insert_into(kernel_sessions_events::dsl::kernel_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok((new_session, non_stored.token_plaintext));
        })?);
    }
}
