use crate::domain::contact;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteConatct {
    pub contact_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteConatct {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteConatct> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteConatct, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::contacts_contacts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let delete_cmd = contact::Delete {};

            let contact_to_update: contact::Contact = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::id.eq(msg.contact_id))
                .filter(contacts_contacts::dsl::owner_id.eq(msg.actor_id))
                .for_update()
                .first(&conn)?;

            let (contact_to_update, _) =
                eventsourcing::execute(&conn, contact_to_update, &delete_cmd)?;

            // delete contact
            diesel::delete(&contact_to_update).execute(&conn)?;

            return Ok(());
        })?);
    }
}
