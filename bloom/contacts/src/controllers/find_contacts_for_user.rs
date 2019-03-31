use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    db::DbActor,
};
use crate::domain::contact;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindContactsForUser {
    pub user_id: uuid::Uuid,
}

impl Message for FindContactsForUser {
    type Result = Result<Vec<contact::Contact>, KernelError>;
}

impl Handler<FindContactsForUser> for DbActor {
    type Result = Result<Vec<contact::Contact>, KernelError>;

    fn handle(&mut self, msg: FindContactsForUser, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            contacts_contacts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let contacts: Vec<contact::Contact> = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::owner_id.eq(msg.user_id))
                .filter(contacts_contacts::dsl::deleted_at.is_null())
                .load(&conn)?;

        return Ok(contacts);
    }
}
