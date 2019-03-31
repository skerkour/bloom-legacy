use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain,
    users::domain::pending_user,
    users::notifications::emails::send_user_verification_code,
    events::EventMetadata,
    config::Config,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct StartRegistration {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
    // pub logger: RequestLogger,
}

impl Message for StartRegistration {
    type Result = Result<domain::PendingUser, KernelError>;
}

impl Handler<StartRegistration> for DbActor {
    type Result = Result<domain::PendingUser, KernelError>;

    fn handle(&mut self, msg: StartRegistration, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_pending_users,
            kernel_pending_users_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let config = msg.config.clone();

            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };


            let create_cmd = pending_user::Create{
                first_name: msg.first_name.clone(),
                last_name: msg.last_name.clone(),
                email: msg.email.clone(),
                password: msg.password.clone(),
                metadata,
            };
            let (new_pending_user, event, non_persisted) = eventsourcing::execute(&conn, pending_user::PendingUser::new(), &create_cmd)?;

            diesel::insert_into(kernel_pending_users::dsl::kernel_pending_users)
                .values(&new_pending_user)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_users_events::dsl::kernel_pending_users_events)
                .values(&event)
                .execute(&conn)?;

            send_user_verification_code(
                &config,
                new_pending_user.email.as_str(),
                format!("{} {}", &new_pending_user.first_name, &new_pending_user.last_name).as_str(),
                new_pending_user.id.to_string().as_str(),
                &non_persisted.code,
            ).expect("error sending email");

            return Ok(new_pending_user);
        })?);
    }
}
