use crate::error::KernelError;
use crate::{
    db::DbActor,
    events::EventMetadata,
    myaccount::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub actor: Account,
    pub session: Session,
    pub request_id: uuid::Uuid,
}

impl Message for SignOut {
    type Result = Result<(), KernelError>;
}

impl Handler<SignOut> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: SignOut, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.actor.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session.id),
            };
            let sign_out_cmd = session::SignOut { metadata };

            let _ = eventsourcing::execute(&conn, &mut msg.session, &sign_out_cmd)?;

            // update session
            diesel::update(&msg.session).set(&msg.session).execute(&conn)?;

            return Ok(());
        })?);
    }
}
