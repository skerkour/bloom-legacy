use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::contacts::domain::contact,
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateContact {
    pub addresses: Vec<contact::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<contact::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<contact::Organization>,
    pub phones: Vec<contact::Phone>,
    pub websites: Vec<contact::Website>,
    pub user_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateContact {
    type Result = Result<contact::Contact, KernelError>;
}

impl Handler<CreateContact> for DbActor {
    type Result = Result<contact::Contact, KernelError>;

    fn handle(&mut self, msg: CreateContact, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            contacts_contacts,
            contacts_contacts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Contact
            let metadata = EventMetadata{
                actor_id: Some(msg.user_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = contact::Create{
                addresses: msg.addresses,
                birthday: msg.birthday,
                company: msg.company,
                emails: msg.emails,
                first_name: msg.first_name,
                last_name: msg.last_name,
                notes: msg.notes,
                occupation: msg.occupation,
                organizations: msg.organizations,
                phones: msg.phones,
                websites: msg.websites,
                owner_id: msg.user_id,
                metadata,
            };
            let (note, event, non_stored) = eventsourcing::execute(&conn, contact::Contact::new(), &create_cmd)?;

            diesel::insert_into(contacts_contacts::dsl::contacts_contacts)
                .values(&note)
                .execute(&conn)?;
            diesel::insert_into(contacts_contacts_events::dsl::contacts_contacts_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(note);
        })?);
    }
}
