use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::{
        User,
        session,
        Session,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RevokeSession {
    pub actor: User,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub current_session_id: uuid::Uuid,
}

impl Message for RevokeSession {
    type Result = Result<(), KernelError>;
}

impl Handler<RevokeSession> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RevokeSession, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_sessions,
            kernel_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.actor.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.current_session_id),
            };

            let session: Session = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::id.eq(msg.session_id))
                .filter(kernel_sessions::dsl::user_id.eq(msg.actor.id))
                .filter(kernel_sessions::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let revoke_cmd = session::Revoke{
                current_session_id: Some(msg.current_session_id),
                reason: session::RevokedReason::Manually,
                metadata,
            };

            let (session, event, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;

            // update session
            diesel::update(&session)
                .set(&session)
                .execute(&conn)?;
            diesel::insert_into(kernel_sessions_events::dsl::kernel_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
