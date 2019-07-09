use crate::domain::contact;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindContactsForAccount {
    pub account_id: uuid::Uuid,
}

impl Message for FindContactsForAccount {
    type Result = Result<Vec<contact::Contact>, KernelError>;
}

impl Handler<FindContactsForAccount> for DbActor {
    type Result = Result<Vec<contact::Contact>, KernelError>;

    fn handle(&mut self, msg: FindContactsForAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::contacts_contacts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let contacts: Vec<contact::Contact> = contacts_contacts::dsl::contacts_contacts
            .filter(contacts_contacts::dsl::owner_id.eq(msg.account_id))
            .load(&conn)?;

        return Ok(contacts);
    }
}
