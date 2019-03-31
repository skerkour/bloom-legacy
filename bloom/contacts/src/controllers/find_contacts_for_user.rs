use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    db::DbActor,
};
use crate::domain::contact;



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
        use kernel::db::schema::{
            contacts_contacts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let contacts: Vec<contact::Contact> = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::owner_id.eq(msg.account_id))
                .filter(contacts_contacts::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(contacts);
    }
}
