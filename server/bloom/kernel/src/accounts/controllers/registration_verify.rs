use crate::{
    accounts::domain::{pending_account, PendingAccount},
    db::DbActor,
};
use actix::{Handler, Message};
use bloom_error::BloomError;
use bloom_messages::kernel::Empty;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationVerify {
    pub message: bloom_messages::auth::RegistrationVerify,
}

impl Message for RegistrationVerify {
    type Result = Result<bloom_messages::Message, BloomError>;
}

impl Handler<RegistrationVerify> for DbActor {
    type Result = Result<bloom_messages::Message, BloomError>;

    fn handle(&mut self, msg: RegistrationVerify, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::pending_accounts;
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        match conn.transaction::<_, BloomError, _>(|| {
            let verify_cmd = pending_account::Verify {
                id: msg.message.id,
                code: msg.message.code.clone(),
            };

            let pending_account_to_verify: PendingAccount = pending_accounts::dsl::pending_accounts
                .filter(pending_accounts::dsl::id.eq(msg.message.id))
                .for_update()
                .first(&conn)?;

            let (pending_account_to_verify, _) =
                eventsourcing::execute(&conn, pending_account_to_verify, &verify_cmd)?;

            // update pending_account
            diesel::update(&pending_account_to_verify)
                .set(&pending_account_to_verify)
                .execute(&conn)?;

            return Ok(bloom_messages::Message::from(Empty {}));
        }) {
            Ok(_) => return Ok(Empty {}.into()),
            Err(err) => match err {
                BloomError::Validation(_) => {
                    let pending_account_to_verify: PendingAccount =
                        pending_accounts::dsl::pending_accounts
                            .filter(pending_accounts::dsl::id.eq(msg.message.id))
                            .for_update()
                            .first(&conn)?;
                    let fail_cmd = pending_account::FailVerification {};
                    let (pending_account_to_verify, _) =
                        eventsourcing::execute(&conn, pending_account_to_verify, &fail_cmd)?;
                    // update pending_account_to_verify trials
                    diesel::update(&pending_account_to_verify)
                        .set(&pending_account_to_verify)
                        .execute(&conn)?;
                    return Err(err);
                }
                _ => return Err(err),
            },
        }
    }
}
