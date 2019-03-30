use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::contacts::domain,
    error::KernelError,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindContactsForUser {
    pub account_id: uuid::Uuid,
}

impl Message for FindContactsForUser {
    type Result = Result<Vec<domain::Contact>, KernelError>;
}

impl Handler<FindContactsForUser> for DbActor {
    type Result = Result<Vec<domain::Contact>, KernelError>;

    fn handle(&mut self, msg: FindContactsForUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            contacts_contacts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let contacts: Vec<domain::Contact> = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::owner_id.eq(msg.account_id))
                .filter(contacts_contacts::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(contacts);
    }
}
