use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::{
        PendingUser,
        pending_user,
        pending_user::EventData,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyPendingUser {
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: uuid::Uuid,
}

impl Message for VerifyPendingUser {
    type Result = Result<(), KernelError>;
}

impl Handler<VerifyPendingUser> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: VerifyPendingUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_pending_users,
            kernel_pending_users_events,
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
            let verify_pending_user_cmd = pending_user::Verify{
                id: msg.id,
                code: msg.code.clone(),
                metadata,
            };

            let pending_user: PendingUser = kernel_pending_users::dsl::kernel_pending_users
                .filter(kernel_pending_users::dsl::id.eq(msg.id))
                .filter(kernel_pending_users::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (pending_user, event, _) = eventsourcing::execute(&conn, pending_user, &verify_pending_user_cmd)?;

            // update pending_user
            diesel::update(&pending_user)
                .set(&pending_user)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_users_events::dsl::kernel_pending_users_events)
                .values(&event)
                .execute(&conn)?;

            return match event.data {
                EventData::VerificationFailedV1(err) => Err(KernelError::Validation(err.to_string())),
                _ => Ok(()),
            };
        })?);
    }
}
