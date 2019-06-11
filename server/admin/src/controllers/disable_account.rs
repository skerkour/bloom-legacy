use actix::{Message, Handler};
use kernel::{
    db::DbActor,
    myaccount::domain::{
        Account,
        account,
        session,
    },
    error::KernelError,
    events::EventMetadata,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DisableAccount {
    pub actor: Account,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DisableAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<DisableAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DisableAccount, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            kernel_accounts,
            kernel_accounts_events,
            kernel_sessions,
            kernel_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.actor.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            if !msg.actor.is_admin {
                return Err(KernelError::Forbidden("Admin role is required".to_string()));
            }

            let account_to_disable: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let disable_cmd = account::Disable{
                actor: msg.actor,
                metadata: metadata.clone(),
            };

            let (account_to_disable, event, _) = eventsourcing::execute(&conn, account_to_disable, &disable_cmd)?;
            diesel::update(&account_to_disable)
                .set(&account_to_disable)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // revoke all sessions
            let sessions: Vec<session::Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account_to_disable.id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke{
                current_session_id: None,
                reason: session::RevokedReason::AccountDisabled,
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

            return Ok(());
        })?);
    }
}
