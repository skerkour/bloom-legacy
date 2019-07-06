use crate::error::KernelError;
use crate::{db::DbActor, events::EventMetadata, myaccount::domain::account};
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
            let metadata = EventMetadata {
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let account_to_update = msg.account;

            // first_name
            if let Some(ref first_name) = msg.first_name {
                if first_name != &account_to_update.first_name {
                    let update_first_name_cmd = account::UpdateFirstName {
                        first_name: first_name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let _ = eventsourcing::execute(
                        &conn,
                        &mut account_to_update,
                        &update_first_name_cmd,
                    )?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                }
            }

            // last_name
            if let Some(ref last_name) = msg.last_name {
                if last_name != &account_to_update.last_name {
                    let update_last_name_cmd = account::UpdateLastName {
                        last_name: last_name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let _ = eventsourcing::execute(
                        &conn,
                        &mut account_to_update,
                        &update_last_name_cmd,
                    )?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                }
            }

            // bio
            if let Some(ref bio) = msg.bio {
                if bio != &account_to_update.bio {
                    let update_bio_cmd = account::UpdateBio {
                        bio: bio.to_string(),
                        metadata: metadata.clone(),
                    };

                    let _ = eventsourcing::execute(&conn, &mut account_to_update, &update_bio_cmd)?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                }
            }

            // display_name
            if let Some(ref display_name) = msg.display_name {
                if display_name != &account_to_update.display_name {
                    let update_display_name_cmd = account::UpdateDisplayName {
                        display_name: display_name.to_string(),
                        metadata: metadata.clone(),
                    };

                    let _ = eventsourcing::execute(
                        &conn,
                        &mut account_to_update,
                        &update_display_name_cmd,
                    )?;

                    // update account
                    diesel::update(&account_to_update)
                        .set(&account_to_update)
                        .execute(&conn)?;
                }
            }

            return Ok(account_to_update);
        })?);
    }
}
