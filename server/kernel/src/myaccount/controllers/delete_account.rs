use actix::{Message, Handler};
use crate::{
    db::DbActor,
    myaccount::domain::{
        account,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteAccount {
    pub account: account::Account,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let delete_account_cmd = account::DeleteAccount{
                account: msg.account.clone(),
                metadata: metadata.clone(),
            };

            // just pass uuid
            let (account_to_delete, event, _) = eventsourcing::execute(&conn, msg.account, &delete_account_cmd)?;
            
            // update just deleted_at = chrono::Utc::now() or check that eventsourcing done that
            diesel::update(&account_to_delete)
                .set(&account_to_delete)
                .execute(&conn)?;
            
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            Ok(())
        })?);
    }
}
