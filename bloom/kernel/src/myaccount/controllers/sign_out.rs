use crate::error::KernelError;
use crate::{
    db::DbActor,
    myaccount::domain::{session, Account, Session},
};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub actor: Account,
    pub session: Session,
}

impl Message for SignOut {
    type Result = Result<messages::Message, KernelError>;
}

impl Handler<SignOut> for DbActor {
    type Result = Result<messages::Message, KernelError>;

    fn handle(&mut self, msg: SignOut, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let sign_out_cmd = session::SignOut {};

            let (session, _) = eventsourcing::execute(&conn, msg.session, &sign_out_cmd)?;

            // update session
            diesel::delete(&session).execute(&conn)?;

            return Ok(messages::kernel::Empty {}.into());
        })?);
    }
}
