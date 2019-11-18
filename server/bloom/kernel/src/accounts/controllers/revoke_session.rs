use crate::{
    db::DbActor,
    accounts::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use bloom_error::BloomError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RevokeSession {
    pub message: bloom_messages::auth::RevokeSesison,
    pub actor: Account,
    pub current_session_id: uuid::Uuid,
}

impl Message for RevokeSession {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<RevokeSession> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: RevokeSession, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::sessions;
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        return Ok(conn.transaction::<_, BloomError, _>(|| {
            let session: Session = sessions::dsl::sessions
                .filter(sessions::dsl::id.eq(msg.message.id))
                .filter(sessions::dsl::account_id.eq(msg.actor.id))
                .for_update()
                .first(&conn)?;

            let revoke_cmd = session::Revoke {
                current_session_id: Some(msg.current_session_id),
                // reason: session::RevokedReason::Manually,
            };

            let (session, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;

            // delete session
            diesel::delete(&session).execute(&conn)?;

            return Ok(bloom_messages::kernel::Empty {}.into());
        })?);
    }
}
