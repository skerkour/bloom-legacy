use crate::domain::contact;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateContact {
    pub addresses: Option<Vec<contact::Address>>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Option<Vec<contact::Email>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Option<Vec<contact::Organization>>,
    pub phones: Option<Vec<contact::Phone>>,
    pub websites: Option<Vec<contact::Website>>,
    pub contact_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for UpdateContact {
    type Result = Result<contact::Contact, KernelError>;
}

impl Handler<UpdateContact> for DbActor {
    type Result = Result<contact::Contact, KernelError>;

    #[allow(clippy::cognitive_complexity)]
    fn handle(&mut self, msg: UpdateContact, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::contacts_contacts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let contact_to_update: contact::Contact = contacts_contacts::dsl::contacts_contacts
                .filter(contacts_contacts::dsl::id.eq(msg.contact_id))
                .filter(contacts_contacts::dsl::owner_id.eq(msg.actor_id))
                .for_update()
                .first(&conn)?;

            // addresses
            let contact_to_update = match &msg.addresses {
                Some(addresses) if addresses != &contact_to_update.addresses => {
                    let update_addresses_cmd = contact::UpdateAddresses {
                        addresses: addresses.clone(),
                    };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_addresses_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // birthday
            let contact_to_update = match &msg.birthday {
                Some(birthday) if Some(*birthday) != contact_to_update.birthday => {
                    let update_birthday_cmd = contact::UpdateBirthday {
                        birthday: Some(*birthday),
                    };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_birthday_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // emails
            let contact_to_update = match &msg.emails {
                Some(emails) if emails != &contact_to_update.emails => {
                    let update_emails_cmd = contact::UpdateEmails {
                        emails: emails.clone(),
                    };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_emails_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // first_name
            let contact_to_update = match &msg.first_name {
                Some(first_name)
                    if Some(first_name.to_string()) != contact_to_update.first_name =>
                {
                    let first_name = first_name.trim();
                    let first_name = if first_name.is_empty() {
                        None
                    } else {
                        Some(first_name.to_string())
                    };
                    let update_first_name_cmd = contact::UpdateFirstName { first_name };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_first_name_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // last_name
            let contact_to_update = match &msg.last_name {
                Some(last_name) if Some(last_name.to_string()) != contact_to_update.last_name => {
                    let last_name = last_name.trim();
                    let last_name = if last_name.is_empty() {
                        None
                    } else {
                        Some(last_name.to_string())
                    };
                    let update_last_name_cmd = contact::UpdateLastName { last_name };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_last_name_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // notes
            let contact_to_update = match &msg.notes {
                Some(notes) if Some(notes.to_string()) != contact_to_update.notes => {
                    let notes = notes.trim();
                    let notes = if notes.is_empty() {
                        None
                    } else {
                        Some(notes.to_string())
                    };
                    let update_notes_cmd = contact::UpdateNotes { notes };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_notes_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // organizations
            let contact_to_update = match &msg.organizations {
                Some(organizations) if organizations != &contact_to_update.organizations => {
                    let update_organizations_cmd = contact::UpdateOrganizations {
                        organizations: organizations.clone(),
                    };

                    let (contact_to_update, _) = eventsourcing::execute(
                        &conn,
                        contact_to_update,
                        &update_organizations_cmd,
                    )?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // phones
            let contact_to_update = match &msg.phones {
                Some(phones) if phones != &contact_to_update.phones => {
                    let update_phones_cmd = contact::UpdatePhones {
                        phones: phones.clone(),
                    };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_phones_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            // websites
            let contact_to_update = match &msg.websites {
                Some(websites) if websites != &contact_to_update.websites => {
                    let update_phones_cmd = contact::UpdateWebsites {
                        websites: websites.clone(),
                    };

                    let (contact_to_update, _) =
                        eventsourcing::execute(&conn, contact_to_update, &update_phones_cmd)?;

                    // update note
                    diesel::update(&contact_to_update)
                        .set(&contact_to_update)
                        .execute(&conn)?;
                    contact_to_update
                }
                _ => contact_to_update,
            };

            return Ok(contact_to_update);
        })?);
    }
}
