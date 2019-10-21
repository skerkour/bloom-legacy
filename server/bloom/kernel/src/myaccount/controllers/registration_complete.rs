use crate::{
    config::Config,
    db::DbActor,
    myaccount::domain::{account, pending_account, session, Account, PendingAccount, Session},
};
use actix::{Handler, Message};
use bloom_const::myaccount;
use bloom_error::BloomError;
use crypto42::kdf::argon2id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub message: bloom_messages::auth::RegistrationComplete,
    pub config: Config,
    pub ip: String,
    pub user_agent: String,
}

impl Message for CompleteRegistration {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending account
        use crate::db::schema::{kernel_accounts, kernel_pending_accounts, kernel_sessions};
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        return Ok(conn.transaction::<_, BloomError, _>(|| {
            println!("COMPLETE CONTROLLER");
            let pending_account_to_update: PendingAccount =
                kernel_pending_accounts::dsl::kernel_pending_accounts
                    .filter(kernel_pending_accounts::dsl::id.eq(msg.message.id))
                    .for_update()
                    .first(&conn)?;

            // complete registration
            let complete_registration_cmd = pending_account::Delete {};
            let (pending_account_to_update, _) = eventsourcing::execute(
                &conn,
                pending_account_to_update,
                &complete_registration_cmd,
            )?;

            diesel::update(&pending_account_to_update)
                .set(&pending_account_to_update)
                .execute(&conn)?;

            let auth_key_hash = argon2id::hash_password(
                msg.message.auth_key.as_bytes(),
                myaccount::PASSWORD_ARGON2_OPSLIMIT,
                myaccount::PASSWORD_ARGON2_MEMLIMIT,
            )?
            .to_string();

            // create account
            let create_cmd = account::Create {
                display_name: pending_account_to_update.display_name.clone(),
                email: pending_account_to_update.email,
                auth_key_hash,
                username: msg.message.username.clone(),
            };
            let (new_account, event) = eventsourcing::execute(&conn, Account::new(), &create_cmd)?;

            diesel::insert_into(kernel_accounts::dsl::kernel_accounts)
                .values(&new_account)
                .execute(&conn)?;

            eventsourcing::publish::<_, _, BloomError>(&conn, &event)?;

            // start Session
            let start_cmd = session::Start {
                account_id: new_account.id,
                ip: msg.ip,
                user_agent: msg.user_agent,
            };

            let (new_session, event) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;
            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;

            return Ok(bloom_messages::auth::Session {
                id: new_session.id,
                token: event.token_plaintext,
            }
            .into());
        })?);
    }
}
