use actix::{Handler, Message};
use kernel::{
    db::DbActor,
    error::KernelError,
    myaccount::domain::{account, session, Account},
};
use serde::{Deserialize, Serialize};

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
        use diesel::prelude::*;
        use kernel::db::schema::{kernel_accounts, kernel_sessions};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if !msg.actor.is_admin {
                return Err(KernelError::Forbidden("Admin role is required".to_string()));
            }

            let account_to_disable: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            if account_to_disable.is_admin {
                let remaining_admins: i64 = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::is_admin.eq(true))
                    .filter(kernel_accounts::dsl::disabled_at.is_null())
                    .count()
                    .get_result(&conn)?;
                if remaining_admins < 2 {
                    return Err(KernelError::Validation(
                        "You can't disable last admin's account".to_string(),
                    ));
                }
            }

            let disable_cmd = account::Disable { actor: msg.actor };

            let (account_to_disable, _) =
                eventsourcing::execute(&conn, account_to_disable, &disable_cmd)?;
            diesel::update(&account_to_disable)
                .set(&account_to_disable)
                .execute(&conn)?;

            // revoke all sessions
            let sessions: Vec<session::Session> = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::account_id.eq(account_to_disable.id))
                .for_update()
                .load(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: None,
                reason: session::RevokedReason::AccountDisabled,
            };

            for session in sessions {
                let (session, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::update(&session).set(&session).execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
