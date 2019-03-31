use actix::{Message, Handler};
use crate::{
    db::DbActor,
    error::KernelError,
    config::Config,
    accounts::domain::{
        PendingAccount,
        pending_accounts,
        Session,
        sessions,
        Account,
        accounts,
    },
    events::EventMetadata,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: uuid::Uuid,
    pub username: String,
    pub config: Config,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteRegistration {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending account
        use crate::db::schema::{
            kernel_pending_accounts,
            kernel_pending_accounts_events,
            kernel_accounts,
            kernel_accounts_events,
            kernel_sessions,
            kernel_sessions_events,
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

            let pending_account_to_update: PendingAccount = kernel_pending_accounts::dsl::kernel_pending_accounts
                .filter(kernel_pending_accounts::dsl::id.eq(msg.id))
                .filter(kernel_pending_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // complete registration
            let complete_registration_cmd = pending_accounts::CompleteRegistration{
                id: msg.id,
                metadata: metadata.clone(),
            };
            let (pending_account_to_update, event, _) = eventsourcing::execute(&conn, pending_account_to_update, &complete_registration_cmd)?;

            diesel::update(&pending_account_to_update)
                .set(&pending_account_to_update)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_accounts_events::dsl::kernel_pending_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // create account
            let create_cmd = accounts::Create{
                first_name: pending_account_to_update.first_name.clone(),
                last_name: pending_account_to_update.last_name.clone(),
                email: pending_account_to_update.email.clone(),
                password: pending_account_to_update.password.clone(),
                username: msg.username.clone(),
                www_host: msg.config.www_host(),
                metadata: metadata.clone(),
            };
            let (new_account, event, _) = eventsourcing::execute(&conn, Account::new(), &create_cmd)?;

            diesel::insert_into(kernel_accounts::dsl::kernel_accounts)
                .values(&new_account)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // start Session
            let metadata = EventMetadata{
                actor_id: Some(new_account.id),
                ..metadata.clone()
            };
            let start_cmd = sessions::Start{
                account_id: new_account.id,
                ip: "127.0.0.1".to_string(), // TODO
                account_agent: "".to_string(), // TODO
                metadata,
            };
            let (new_session, event, non_stored) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(kernel_sessions::dsl::kernel_sessions)
                .values(&new_session)
                .execute(&conn)?;
            diesel::insert_into(kernel_sessions_events::dsl::kernel_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok((new_session, non_stored.token_plaintext));
        })?);
    }
}
