use crate::error::KernelError;
use crate::{
    db::DbActor,
    myaccount::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RevokeSession {
    pub message: messages::auth::RevokeSesison,
    pub actor: Account,
    pub current_session_id: uuid::Uuid,
}

impl Message for RevokeSession {
    type Result = Result<messages::Message, KernelError>;
}

impl Handler<RevokeSession> for DbActor {
    type Result = Result<messages::Message, KernelError>;

    fn handle(&mut self, msg: RevokeSession, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::kernel_sessions;
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let session: Session = kernel_sessions::dsl::kernel_sessions
                .filter(kernel_sessions::dsl::id.eq(msg.message.id))
                .filter(kernel_sessions::dsl::account_id.eq(msg.actor.id))
                .for_update()
                .first(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: Some(msg.current_session_id),
                // reason: session::RevokedReason::Manually,
            };

            let (session, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;

            // delete session
            diesel::delete(&session).execute(&conn)?;

            return Ok(messages::kernel::NoData {}.into());
        })?);
    }
}
