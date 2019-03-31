use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::contacts_contacts_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "contacts_contacts_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    DeletedV1,
    AddressesUpdatedV1(AddressesUpdatedV1),
    BirthdayUpdatedV1(BirthdayUpdatedV1),
    EmailsUpdatedV1(EmailsUpdatedV1),
    FirstNameUpdatedV1(FirstNameUpdatedV1),
    LastNameUpdatedV1(LastNameUpdatedV1),
    NotesUpdatedV1(NotesUpdatedV1),
    OrganizationsUpdatedV1(OrganizationsUpdatedV1),
    PhonesUpdatedV1(PhonesUpdatedV1),
    WebsitesUpdatedV1(WebsitesUpdatedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub addresses: Vec<super::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<super::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<super::Organization>,
    pub phones: Vec<super::Phone>,
    pub websites: Vec<super::Website>,
    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddressesUpdatedV1 {
    pub addresses: Vec<super::Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BirthdayUpdatedV1 {
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailsUpdatedV1 {
    pub emails: Vec<super::Email>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirstNameUpdatedV1 {
    pub first_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LastNameUpdatedV1 {
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotesUpdatedV1 {
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizationsUpdatedV1 {
    pub organizations: Vec<super::Organization>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhonesUpdatedV1 {
    pub phones: Vec<super::Phone>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebsitesUpdatedV1 {
    pub websites: Vec<super::Website>,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Contact{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                addresses: data.addresses.clone(),
                birthday: data.birthday.clone(),
                company: data.company.clone(),
                emails: data.emails.clone(),
                first_name: data.first_name.clone(),
                last_name: data.last_name.clone(),
                notes: data.notes.clone(),
                occupation: data.occupation.clone(),
                organizations: data.organizations.clone(),
                phones: data.phones.clone(),
                websites: data.websites.clone(),

                owner_id: data.owner_id,
            },
            // DeletedV1
            EventData::DeletedV1 => super::Contact{
                deleted_at: Some(self.timestamp),
                ..aggregate.clone()
            },
            // AddressesUpdatedV1
            EventData::AddressesUpdatedV1(ref data) => super::Contact{
                addresses: data.addresses.clone(),
                ..aggregate.clone()
            },
            // BirthdayUpdatedV1
            EventData::BirthdayUpdatedV1(ref data) => super::Contact{
                birthday: data.birthday.clone(),
                ..aggregate.clone()
            },
            // EmailsUpdatedV1
            EventData::EmailsUpdatedV1(ref data) => super::Contact{
                emails: data.emails.clone(),
                ..aggregate.clone()
            },
            // FirstNameUpdatedV1
            EventData::FirstNameUpdatedV1(ref data) => super::Contact{
                first_name: data.first_name.clone(),
                ..aggregate.clone()
            },
            // LastNameUpdatedV1
            EventData::LastNameUpdatedV1(ref data) => super::Contact{
                last_name: data.last_name.clone(),
                ..aggregate.clone()
            },
            // NotesUpdatedV1
            EventData::NotesUpdatedV1(ref data) => super::Contact{
                notes: data.notes.clone(),
                ..aggregate.clone()
            },
            // OrganizationsUpdatedV1
            EventData::OrganizationsUpdatedV1(ref data) => super::Contact{
                organizations: data.organizations.clone(),
                ..aggregate.clone()
            },
            // PhonesUpdatedV1
            EventData::PhonesUpdatedV1(ref data) => super::Contact{
                phones: data.phones.clone(),
                ..aggregate.clone()
            },
            // WebsitesUpdatedV1
            EventData::WebsitesUpdatedV1(ref data) => super::Contact{
                websites: data.websites.clone(),
                ..aggregate.clone()
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
