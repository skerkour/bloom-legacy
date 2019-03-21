use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        PendingAccount,
        pending_account,
        session,
        pending_account::EventData,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub session_id: String,
    pub request_id: String,
}

impl Message for SignOut {
    type Result = Result<(), KernelError>;
}

// TODO: transaction
impl Handler<SignOut> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: SignOut, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_sessions,
            account_sessions_events,
        };
        use diesel::RunQueryDsl;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::ExpressionMethods;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let metdata = EventMetadata{
            actor_id: None,
            request_id: Some(msg.request_id.clone()),
        };
        let end_cmd = session::End{
            metdata,
        };

        let pending_account_id = uuid::Uuid::parse_str(&msg.session_id)
            .map_err(|_| KernelError::Validation("Session id is not valid.".to_string()))?;

        let pending_account: PendingAccount = account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
            .filter(account_pending_accounts::dsl::deleted_at.is_null())
            .first(&conn)?;

        let (pending_account, event, _) = eventsourcing::execute(&conn, pending_account, &verify_cmd)?;

        // update pending_account
        diesel::update(account_pending_accounts::dsl::account_pending_accounts)
            .set((
                account_pending_accounts::dsl::trials.eq(pending_account.trials),
                account_pending_accounts::dsl::version.eq(pending_account.version),
                account_pending_accounts::dsl::updated_at.eq(pending_account.updated_at),
            ))
            .execute(&conn)?;


        diesel::insert_into(account_pending_accounts_events::dsl::account_pending_accounts_events)
            .values(&event)
            .execute(&conn)?;

        return match event.data {
            EventData::VerificationFailedV1(err) => Err(KernelError::Validation(err)),
            _ => Ok(()),
        };
    }
}
