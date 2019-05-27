use actix::{Message, Handler};
use crate::{
    db::DbActor,
    myaccount::domain::{
        PendingAccount,
        pending_account,
        pending_account::EventData,
    },
    events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyPendingAccount {
    pub id: uuid::Uuid,
    pub code: String,
    pub request_id: uuid::Uuid,
}

impl Message for VerifyPendingAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<VerifyPendingAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: VerifyPendingAccount, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_pending_accounts,
            kernel_pending_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };
            let verify_pending_account_cmd = pending_account::Verify{
                id: msg.id,
                code: msg.code.clone(),
                metadata,
            };

            let pending_account: PendingAccount = kernel_pending_myaccount::dsl::kernel_pending_accounts
                .filter(kernel_pending_myaccount::dsl::id.eq(msg.id))
                .filter(kernel_pending_myaccount::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (pending_account, event, _) = eventsourcing::execute(&conn, pending_account, &verify_pending_account_cmd)?;

            // update pending_account
            diesel::update(&pending_account)
                .set(&pending_account)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_accounts_events::dsl::kernel_pending_accounts_events)
                .values(&event)
                .execute(&conn)?;

            return match event.data {
                EventData::VerificationFailedV1(err) => Err(KernelError::Validation(err.to_string())),
                _ => Ok(()),
            };
        })?);
    }
}
