use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::{
        User,
        session,
        Session,
        user,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub username: String,
    pub password: String,
    pub request_id: uuid::Uuid,
}

impl Message for SignIn {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<SignIn> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_users,
            kernel_users_events,
            kernel_sessions,
            kernel_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let user: User = kernel_users::dsl::kernel_users
                .filter(kernel_users::dsl::username.eq(&msg.username))
                .filter(kernel_users::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)
                .map_err(|_| KernelError::Unauthorized("Invalid username/password combination".to_string()))?;

            // verify password
            if !bcrypt::verify(&msg.password, &user.password)
                .map_err(|_| KernelError::Bcrypt)? {
                // store a SignInFailed event
                let metadata = EventMetadata{
                    actor_id: None,
                    request_id: Some(msg.request_id),
                    session_id: None,
                };
                let fail_sign_in_cmd = user::FailSignIn{metadata};
                let (_, event, _) = eventsourcing::execute(&conn, user, &fail_sign_in_cmd)?;
                diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                    .values(&event)
                    .execute(&conn)?;
                return Err(KernelError::Unauthorized("Invalid username/password combination".to_string()));
            }

            // start Session
            let metadata = EventMetadata{
                actor_id: Some(user.id),
                request_id: Some(msg.request_id),
                session_id: None,
            };
            let start_cmd = session::Start{
                user_id: user.id,
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
