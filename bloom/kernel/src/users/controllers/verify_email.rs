use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::{
        PendingEmail,
        pending_email,
        User,
        user,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyEmail {
    pub user: User,
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for VerifyEmail {
    type Result = Result<User, KernelError>;
}

impl Handler<VerifyEmail> for DbActor {
    type Result = Result<User, KernelError>;

    fn handle(&mut self, msg: VerifyEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_users_events,
            kernel_users_events_events,
            kernel_users_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.user.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let user_to_update = msg.user;

            let pending_email: PendingEmail = kernel_users_events::dsl::kernel_users_events
                .filter(kernel_users_events::dsl::id.eq(msg.id))
                .filter(kernel_users_events::dsl::user_id.eq(user_to_update.id))
                .filter(kernel_users_events::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let verify_cmd = pending_email::Verify{
                id: msg.id,
                code: msg.code.clone(),
                email: pending_email.email.clone(),
                metadata: metadata.clone(),
            };

            let (pending_email, event, _) = eventsourcing::execute(&conn, pending_email, &verify_cmd)?;

            // update pending_email
            diesel::update(&pending_email)
                .set(&pending_email)
                .execute(&conn)?;
            diesel::insert_into(kernel_users_events_events::dsl::kernel_users_events_events)
                .values(&event)
                .execute(&conn)?;

            let user_to_update = match event.data {
                pending_email::EventData::VerificationSucceededV1 => {
                    let update_email_cmd = user::UpdateEmail{
                        email: pending_email.email,
                        metadata: metadata.clone(),
                    };

                    let (user_to_update, event, _) = eventsourcing::execute(&conn, user_to_update, &update_email_cmd)?;

                    // update user
                    diesel::update(&user_to_update)
                        .set(&user_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                        .values(&event)
                        .execute(&conn)?;

                    // delete all other pending emails for user
                    let pending_emails_to_delete: Vec<PendingEmail> = kernel_users_events::dsl::kernel_users_events
                        .filter(kernel_users_events::dsl::user_id.eq(user_to_update.id))
                        .filter(kernel_users_events::dsl::id.ne(msg.id))
                        .filter(kernel_users_events::dsl::deleted_at.is_null())
                        .for_update()
                        .load(&conn)?;

                    let delete_cmd = pending_email::Delete{
                        metadata: metadata.clone(),
                    };

                    for pending_email_to_delete in pending_emails_to_delete {
                        let (pending_email_to_delete, event, _) = eventsourcing::execute(&conn, pending_email_to_delete, &delete_cmd)?;
                        diesel::update(&pending_email_to_delete)
                            .set(&pending_email_to_delete)
                            .execute(&conn)?;
                        diesel::insert_into(kernel_users_events_events::dsl::kernel_users_events_events)
                            .values(&event)
                            .execute(&conn)?;
                    }
                    user_to_update
                },
                _ => user_to_update,
            };

            return match event.data {
                pending_email::EventData::VerificationFailedV1(err) => Err(KernelError::Validation(err.to_string())),
                _ => Ok(user_to_update),
            };
        })?);
    }
}
