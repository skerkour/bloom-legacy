use crate::domain::contact;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindContactForAccount {
    pub contact_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
}

impl Message for FindContactForAccount {
    type Result = Result<contact::Contact, KernelError>;
}

impl Handler<FindContactForAccount> for DbActor {
    type Result = Result<contact::Contact, KernelError>;

    fn handle(&mut self, msg: FindContactForAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::contacts_contacts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let contact: contact::Contact = contacts_contacts::dsl::contacts_contacts
            .filter(contacts_contacts::dsl::id.eq(msg.contact_id))
            .filter(contacts_contacts::dsl::owner_id.eq(msg.account_id))
            .first(&conn)?;

        return Ok(contact);
    }
}
