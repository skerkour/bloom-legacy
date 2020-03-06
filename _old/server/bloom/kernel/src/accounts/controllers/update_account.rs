use crate::error::KernelError;
use crate::{db::DbActor, accounts::domain::account};
use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccount {
    pub account: account::Account,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateAccount {
    type Result = Result<account::Account, KernelError>;
}

impl Handler<UpdateAccount> for DbActor {
    type Result = Result<account::Account, KernelError>;

    fn handle(&mut self, msg: UpdateAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account_to_update = msg.account;

            // first_name
            let account_to_update = match &msg.first_name {
                Some(first_name) if first_name != &account_to_update.first_name => {
                    let update_first_name_cmd = account::UpdateFirstName {
                        first_name: first_name.to_string(),
                    };

                    let (account_to_update, _) =
                        eventsourcing::execute(&conn, account_to_update, &update_first_name_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                    account_to_update
                }
                _ => account_to_update,
            };

            // last_name
            let account_to_update = match &msg.last_name {
                Some(last_name) if last_name != &account_to_update.last_name => {
                    let update_last_name_cmd = account::UpdateLastName {
                        last_name: last_name.to_string(),
                    };

                    let (account_to_update, _) =
                        eventsourcing::execute(&conn, account_to_update, &update_last_name_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                    account_to_update
                }
                _ => account_to_update,
            };

            // bio
            let account_to_update = match &msg.bio {
                Some(bio) if bio != &account_to_update.bio => {
                    let update_bio_cmd = account::UpdateBio {
                        bio: bio.to_string(),
                    };

                    let (account_to_update, _) =
                        eventsourcing::execute(&conn, account_to_update, &update_bio_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                    account_to_update
                }
                _ => account_to_update,
            };

            // display_name
            let account_to_update = match &msg.display_name {
                Some(display_name) if display_name != &account_to_update.display_name => {
                    let update_display_name_cmd = account::UpdateDisplayName {
                        display_name: display_name.to_string(),
                    };

                    let (account_to_update, _) =
                        eventsourcing::execute(&conn, account_to_update, &update_display_name_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                    account_to_update
                }
                _ => account_to_update,
            };

            return Ok(account_to_update);
        })?);
    }
}
