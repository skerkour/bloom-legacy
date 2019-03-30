use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::user,
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateUser {
    pub user: user::User,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateUser {
    type Result = Result<User, KernelError>;
}

impl Handler<UpdateUser> for DbActor {
    type Result = Result<User, KernelError>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_users_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let user_to_update = msg.account;

            // first_name
            let user_to_update = match &msg.first_name {
                Some(first_name) if first_name != &user_to_update.first_name => {
                    let update_first_name_cmd = account::UpdateFirstName{
                        first_name: first_name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let (user_to_update, event, _) = eventsourcing::execute(&conn, user_to_update, &update_first_name_cmd)?;

                    // update account
                    diesel::update(&user_to_update)
                        .set(&user_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                        .values(&event)
                        .execute(&conn)?;
                    user_to_update
                },
                _ => user_to_update,
            };

            // last_name
            let user_to_update = match &msg.last_name {
                Some(last_name) if last_name != &user_to_update.last_name => {
                    let update_last_name_cmd = account::UpdateLastName{
                        last_name: last_name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let (user_to_update, event, _) = eventsourcing::execute(&conn, user_to_update, &update_last_name_cmd)?;

                    // update account
                    diesel::update(&user_to_update)
                        .set(&user_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                        .values(&event)
                        .execute(&conn)?;
                    user_to_update
                },
                _ => user_to_update,
            };

            return Ok(user_to_update);
        })?);
    }
}
