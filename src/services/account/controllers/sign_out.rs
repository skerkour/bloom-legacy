use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        Account,
        session,
        Session,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub actor: Account,
    pub session: Session,
    pub request_id: String,
}

impl Message for SignOut {
    type Result = Result<(), KernelError>;
}

impl Handler<SignOut> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: SignOut, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_sessions,
            account_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.actor.id),
                request_id: Some(msg.request_id.clone()),
            };
            let sign_out_cmd = session::SignOut{
                metadata,
            };

            let (session, event, _) = eventsourcing::execute(&conn, msg.session, &sign_out_cmd)?;

            // update session
            diesel::update(&session)
                .set(&session)
                .execute(&conn)?;

            diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
