use actix::{Message, Handler};
use crate::{
    db::DbActor,
    error::KernelError,
    config::Config,
    users::domain::{
        PendingUser,
        pending_user,
        Session,
        session,
        User,
        user,
    },
    users,
    events::EventMetadata,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: uuid::Uuid,
    pub username: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteRegistration {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending user
        use crate::db::schema::{
            kernel_pending_users,
            kernel_pending_users_events,
            kernel_users,
            kernel_users_events,
            kernel_sessions,
            kernel_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };

            let pending_user_to_update: PendingUser = kernel_pending_users::dsl::kernel_pending_users
                .filter(kernel_pending_users::dsl::id.eq(msg.id))
                .filter(kernel_pending_users::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // complete registration
            let complete_registration_cmd = pending_user::CompleteRegistration{
                id: msg.id,
                metadata: metadata.clone(),
            };
            let (pending_user_to_update, event, _) = eventsourcing::execute(&conn, pending_user_to_update, &complete_registration_cmd)?;

            diesel::update(&pending_user_to_update)
                .set(&pending_user_to_update)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_users_events::dsl::kernel_pending_users_events)
                .values(&event)
                .execute(&conn)?;

            // create user
            let create_cmd = user::Create{
                first_name: pending_user_to_update.first_name.clone(),
                last_name: pending_user_to_update.last_name.clone(),
                email: pending_user_to_update.email.clone(),
                password: pending_user_to_update.password.clone(),
                username: msg.username.clone(),
                avatar_url: format!("{}{}", msg.config.www_host(), users::AVATAR_DEFAULT_PATH),
                metadata: metadata.clone(),
            };
            let (new_user, event, _) = eventsourcing::execute(&conn, User::new(), &create_cmd)?;

            diesel::insert_into(kernel_users::dsl::kernel_users)
                .values(&new_user)
                .execute(&conn)?;
            diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                .values(&event)
                .execute(&conn)?;

            // start Session
            let metadata = EventMetadata{
                actor_id: Some(new_user.id),
                ..metadata.clone()
            };
            let start_cmd = session::Start{
                user_id: new_user.id,
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
