use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        Account,
        account,
        Session,
        session,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePassword {
    pub account: Account,
    pub current_session: Session,
    pub current_password: String,
    pub new_password: String,
    pub request_id: String,
}

impl Message for UpdatePassword {
    type Result = Result<(), KernelError>;
}

impl Handler<UpdatePassword> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: UpdatePassword, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_accounts,
            account_accounts_events,
            account_sessions,
            account_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id.clone()),
            };

            let account_to_update = msg.account;

            let update_last_name_cmd = account::UpdatePassword{
                current_password: msg.current_password,
                new_password: msg.new_password,
                metadata: metadata.clone(),
            };

            let (account_to_update, event, _) = eventsourcing::execute(&conn, account_to_update, &update_last_name_cmd)?;

            // update account
            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;
            diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // revoke all other active sessions
            let sessions: Vec<Session> = account_sessions::dsl::account_sessions
                .filter(account_sessions::dsl::account_id.eq(account_to_update.id))
                .filter(account_sessions::dsl::deleted_at.is_null())
                .filter(account_sessions::dsl::id.ne(msg.current_session.id))
                .load(&conn)?;

            let revoke_cmd = session::Revoke{
                current_session_id: Some(msg.current_session.id),
                reason: session::RevokedReason::PasswordUpdated,
                metadata: metadata.clone(),
            };

            for session in sessions {
                let (session, event, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::update(&session)
                    .set(&session)
                    .execute(&conn)?;
                diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
