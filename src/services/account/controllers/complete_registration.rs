use actix::{Message, Handler};
use crate::{
    db::DbActor,
    error::KernelError,
    services::account::domain::{
        Account,
        PendingAccount,
        pending_account,
    },
};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: String,
    pub code: String,
    pub username: String,
}

impl Message for CompleteRegistration {
    type Result = Result<bool, KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<bool, KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending account
        use crate::db::schema::{
            account_pending_accounts,
            account_pending_accounts_events,
        };
        use diesel::RunQueryDsl;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::ExpressionMethods;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let pending_account_id = uuid::Uuid::parse_str(&msg.id)
            .map_err(|_| KernelError::Validation("id is not a valid uuid".to_string()))?;

        let mut pending_account_to_update: PendingAccount = account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
            .filter(account_pending_accounts::dsl::deleted_at.is_null())
            .first(&conn)?;

        let now = Utc::now();
        let complete_registration_cmd = pending_account::CompleteRegistration{
            id: msg.id.clone(),
            code: msg.code.clone(),
        };

        // validate
        complete_registration_cmd.validate(&conn, &pending_account_to_update)?;

        diesel::update(account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
        )
            .set(account_pending_accounts::dsl::deleted_at.eq(Some(now)))
            .execute(&conn)?;

        let event = pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: pending_account::EventData::RegistrationCompletedV1,
            aggregate_id: pending_account_to_update.id,
            metadata: pending_account::EventMetadata{},
        };
        diesel::insert_into(account_pending_accounts_events::dsl::account_pending_accounts_events)
            .values(&event)
            .execute(&conn)?;

        // pending_account.trials += 1;

        // create account

        // start Session

        return Ok(false);
    }
}
