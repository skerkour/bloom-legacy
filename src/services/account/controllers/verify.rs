use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        PendingAccount,
        pending_account,
        pending_account::EventData,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: String,
    pub code: String,
    pub request_id: String,
}

impl Message for Verify {
    type Result = Result<(), KernelError>;
}

// TODO: transaction
impl Handler<Verify> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: Verify, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_accounts,
            account_pending_accounts_events,
        };
        use diesel::RunQueryDsl;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::ExpressionMethods;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;
        let now = Utc::now();
        let cmd = pending_account::Verify{
            id: msg.id.clone(),
            code: msg.code.clone(),
        };

        let pending_account_id = uuid::Uuid::parse_str(&cmd.id)
            .map_err(|_| KernelError::Validation("id is not a valid uuid".to_string()))?;

        let mut pending_account: PendingAccount = account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
            .filter(account_pending_accounts::dsl::deleted_at.is_null())
            .first(&conn)?;

        // validate
        let event_data = match cmd.validate(&conn, &pending_account) {
            Ok(_) => EventData::VerificationSucceededV1,
            Err(_) => EventData::VerificationFailedV1,
        };

        pending_account.trials += 1;
        pending_account.version += 1;
        pending_account.updated_at = now;

        // update pending_account
        diesel::update(account_pending_accounts::dsl::account_pending_accounts)
            .set((
                account_pending_accounts::dsl::trials.eq(pending_account.trials),
                account_pending_accounts::dsl::version.eq(pending_account.version),
                account_pending_accounts::dsl::updated_at.eq(pending_account.updated_at),
            ))
            .execute(&conn)?;

        let event = pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: event_data,
            aggregate_id: pending_account.id,
            metadata: EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
            },
        };
        diesel::insert_into(account_pending_accounts_events::dsl::account_pending_accounts_events)
            .values(&event)
            .execute(&conn)?;

        // TODO: return error
        // if event_data == EventData::VerificationFailedV1 {
        // }
        return Ok(());
    }
}
