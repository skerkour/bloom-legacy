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
pub struct UpdateAccount {
    pub account: Account,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
    pub request_id: String,
}

impl Message for UpdateAccount {
    type Result = Result<Account, KernelError>;
}

impl Handler<UpdateAccount> for DbActor {
    type Result = Result<Account, KernelError>;

    fn handle(&mut self, msg: UpdateAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_accounts,
            account_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // let metadata = EventMetadata{
            //     actor_id: Some(msg.actor.id),
            //     request_id: Some(msg.request_id.clone()),
            // };

            // let session: Session = account_sessions::dsl::account_sessions
            //     .filter(account_sessions::dsl::id.eq(msg.session_id))
            //     .filter(account_sessions::dsl::account_id.eq(msg.actor.id))
            //     .filter(account_sessions::dsl::deleted_at.is_null())
            //     .first(&conn)?;

            // let revoke_cmd = session::Revoke{
            //     current_session_id: msg.current_session_id,
            //     metadata,
            // };

            // let (session, event, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;

            // // update session
            // diesel::update(account_sessions::dsl::account_sessions
            //     .filter(account_sessions::dsl::id.eq(session.id))
            // )
            //     .set((
            //         account_sessions::dsl::version.eq(session.version),
            //         account_sessions::dsl::updated_at.eq(session.updated_at),
            //         account_sessions::dsl::deleted_at.eq(session.deleted_at),
            //     ))
            //     .execute(&conn)?;

            // diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
            //     .values(&event)
            //     .execute(&conn)?;

            return Ok(msg.account);
        })?);
    }
}
