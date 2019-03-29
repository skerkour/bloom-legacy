use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::contacts_contacts_events,
    services::common::events::EventMetadata,
};
use std::string::ToString;



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
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
