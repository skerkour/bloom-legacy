use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        Account,
        account,
        session,
        Session,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub username: String,
    pub password: String,
    pub request_id: String,
}

impl Message for SignIn {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<SignIn> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_accounts,
            account_sessions,
            account_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let user: Account = account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::username.eq(&msg.username))
                .filter(account_accounts::dsl::deleted_at.is_null())
                .first(&conn)
                .map_err(|_| KernelError::Unauthorized("Invalid username/password combination".to_string()))?;

            println!("yeaaaaaahahahahahahaha");
            println!("{}", &user.password);
            // verify password
            if !bcrypt::verify(&msg.password, &user.password)
                .map_err(|_| KernelError::Bcrypt)? {
                return Err(KernelError::Unauthorized("Invalid username/password combination".to_string()));
            }

            // start Session
            let start_cmd = session::Start{
                account_id: user.id,
                ip: "127.0.0.1".to_string(), // TODO
                user_agent: "".to_string(), // TODO
                metadata: EventMetadata{
                    actor_id: Some(user.id),
                    request_id: Some(msg.request_id.clone()),
                },
            };
            let (new_session, event, non_stored) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(account_sessions::dsl::account_sessions)
                .values(&new_session)
                .execute(&conn)?;
            diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok((new_session, non_stored.token_cleartext));
        })?);
    }
}
