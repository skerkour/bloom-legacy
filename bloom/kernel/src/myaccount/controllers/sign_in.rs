use crate::error::KernelError;
use crate::{
    db::DbActor,
    messages,
    myaccount::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use crypto42::kdf::argon2id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub message: messages::auth::SignIn,
    pub ip: String,
    pub user_agent: String,
}

impl Message for SignIn {
    type Result = Result<messages::Message, KernelError>;
}

impl Handler<SignIn> for DbActor {
    type Result = Result<messages::Message, KernelError>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{kernel_accounts, kernel_sessions};
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if msg.message.username.contains('@') {
                return Err(KernelError::Validation(
                    "You need to sign in with your username, not your email.".to_string(),
                ));
            }

            let account: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::username.eq(&msg.message.username))
                .for_update()
                .first(&conn)
                .map_err(|_| {
                    KernelError::Unauthorized("Invalid username/password combination".to_string())
                })?;

            // verify auth_key
            if !argon2id::verify_password(
                &account.auth_key_hash.into(),
                msg.message.auth_key.as_bytes(),
            ) {
                // // store a SignInFailed event
                // let metadata = EventMetadata {
                //     actor_id: None,
                //     request_id: Some(msg.request_id),
                //     session_id: None,
                // };
                // let fail_sign_in_cmd = account::FailSignIn { metadata };
                // let (_, event, _) = eventsourcing::execute(&conn, account, &fail_sign_in_cmd)?;
                // diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                //     .values(&event)
                //     .execute(&conn)?;
                return Err(KernelError::Unauthorized(
                    "Invalid username/password combination".to_string(),
                ));
            }

            if account.disabled_at.is_some() {
                return Err(KernelError::Unauthorized(
                    "Account is disabled. Please contact support.".to_string(),
                ));
            }

            // start Session
            let start_cmd = session::Start {
                account_id: account.id,
                ip: msg.ip,
                user_agent: msg.user_agent,
            };
            let (new_session, event) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;

            return Ok(messages::auth::Session {
                id: new_session.id,
                token: event.token_plaintext,
            }
            .into());
        })?);
    }
}
