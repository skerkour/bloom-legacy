use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::contacts::domain,
    error::KernelError,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindContactForUser {
    pub contact_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
}

impl Message for FindContactForUser {
    type Result = Result<domain::Contact, KernelError>;
}

impl Handler<FindContactForUser> for DbActor {
    type Result = Result<domain::Contact, KernelError>;

    fn handle(&mut self, msg: FindContactForUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            contacts_contacts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let contact: domain::Contact = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::id.eq(msg.contact_id))
                .filter(contacts_contacts::dsl::owner_id.eq(msg.account_id))
                .filter(contacts_contacts::dsl::deleted_at.is_null())
                .first(&conn)?;

        return Ok(contact);
    }
}
