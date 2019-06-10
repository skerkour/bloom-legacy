use actix::{Message, Handler};
use crate::{
    db::DbActor,
    myaccount::domain::{
        Account,
        account,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnableAccount {
    pub actor: Account,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for EnableAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<EnableAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: EnableAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts,
            kernel_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.actor.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let account_to_disable: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .filter(kernel_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let disable_cmd = account::Enable{
                actor: msg.actor,
                metadata,
            };

            let (account_to_disable, event, _) = eventsourcing::execute(&conn, account_to_disable, &disable_cmd)?;
            diesel::update(&account_to_disable)
                .set(&account_to_disable)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
