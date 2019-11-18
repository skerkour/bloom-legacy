use crate::{
    db::DbActor,
    myaccount::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use bloom_error::BloomError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub actor: Account,
    pub session: Session,
}

impl Message for SignOut {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<SignOut> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: SignOut, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        return Ok(conn.transaction::<_, BloomError, _>(|| {
            let sign_out_cmd = session::SignOut {};

            let (session, _) = eventsourcing::execute(&conn, msg.session, &sign_out_cmd)?;

            // update session
            diesel::delete(&session).execute(&conn)?;

            return Ok(bloom_messages::kernel::Empty {}.into());
        })?);
    }
}
