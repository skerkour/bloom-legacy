use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        Account,
        account,
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
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id.clone()),
            };

            let account_to_update = msg.account;

            // first_name
            let account_to_update = match msg.first_name {
                Some(first_name) => {
                    let update_first_name_cmd = account::UpdateFirstName{
                        first_name,
                        metadata: metadata.clone(),
                    };

                    let (account_to_update, event, _) = eventsourcing::execute(&conn, account_to_update, &update_first_name_cmd)?;

                    // update account
                    diesel::update(account_accounts::dsl::account_accounts
                        .filter(account_accounts::dsl::id.eq(account_to_update.id)))
                        .set((
                            account_accounts::dsl::version.eq(account_to_update.version),
                            account_accounts::dsl::updated_at.eq(account_to_update.updated_at),
                            account_accounts::dsl::first_name.eq(&account_to_update.first_name),
                        ))
                        .execute(&conn)?;
                    diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                        .values(&event)
                        .execute(&conn)?;
                    account_to_update
                },
                None => account_to_update,
            };

            // last_name
            let account_to_update = match msg.last_name {
                Some(last_name) => {
                    let update_last_name_cmd = account::UpdateLastName{
                        last_name,
                        metadata: metadata.clone(),
                    };

                    let (account_to_update, event, _) = eventsourcing::execute(&conn, account_to_update, &update_last_name_cmd)?;

                    // update account
                    diesel::update(account_accounts::dsl::account_accounts
                        .filter(account_accounts::dsl::id.eq(account_to_update.id)))
                        .set((
                            account_accounts::dsl::version.eq(account_to_update.version),
                            account_accounts::dsl::updated_at.eq(account_to_update.updated_at),
                            account_accounts::dsl::last_name.eq(&account_to_update.last_name),
                        ))
                        .execute(&conn)?;
                    diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                        .values(&event)
                        .execute(&conn)?;
                    account_to_update
                },
                None => account_to_update,
            };

            // password
            let account_to_update = match (msg.new_password, msg.current_password) {
                (Some(new_password), Some(current_password)) => {
                    let update_last_name_cmd = account::UpdatePassword{
                        current_password,
                        new_password,
                        metadata: metadata.clone(),
                    };

                    let (account_to_update, event, _) = eventsourcing::execute(&conn, account_to_update, &update_last_name_cmd)?;

                    // update account
                    // TODO: remove all active session other than the current
                    diesel::update(account_accounts::dsl::account_accounts
                        .filter(account_accounts::dsl::id.eq(account_to_update.id)))
                        .set((
                            account_accounts::dsl::version.eq(account_to_update.version),
                            account_accounts::dsl::updated_at.eq(account_to_update.updated_at),
                            account_accounts::dsl::password.eq(&account_to_update.password),
                        ))
                        .execute(&conn)?;
                    diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                        .values(&event)
                        .execute(&conn)?;
                    account_to_update
                },
                (None, Some(_)) | (Some(_), None) => return Err(KernelError::Validation("new_password and current_password must be both provided to update password".to_string())),
                _ => account_to_update,
            };

            return Ok(account_to_update);
        })?);
    }
}
