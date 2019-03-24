use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_accounts_events,
    services::common::events::EventMetadata,
    services::account::domain,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_accounts_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    FirstNameUpdatedV1(FirstNameUpdatedV1),
    LastNameUpdatedV1(LastNameUpdatedV1),
    PasswordUpdatedV1(PasswordUpdatedV1),
    EmailUpdatedV1(EmailUpdatedV1),
    SignInFailedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirstNameUpdatedV1 {
    pub first_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LastNameUpdatedV1 {
    pub last_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordUpdatedV1 {
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailUpdatedV1 {
    pub email: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => domain::Account {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,
                avatar_url: data.avatar_url.clone(),
                email: data.email.clone(),
                first_name: data.first_name.clone(),
                is_admin: data.is_admin,
                last_name: data.last_name.clone(),
                password: data.password.clone(),
                recovery_id: None,
                recovery_token: None,
                username: data.username.clone(),
            },
            // FirstNameUpdatedV1
            EventData::FirstNameUpdatedV1(ref data) => domain::Account {
                first_name: data.first_name.clone(),
                ..aggregate
            },
            // LastNameUpdatedV1
            EventData::LastNameUpdatedV1(ref data) => domain::Account {
                last_name: data.last_name.clone(),
                ..aggregate
            },
            // PasswordUpdatedV1
            EventData::PasswordUpdatedV1(ref data) => domain::Account {
                password: data.password.clone(),
                ..aggregate
            },
            // EmailUpdatedV1
            EventData::EmailUpdatedV1(ref data) => domain::Account {
                email: data.email.clone(),
                ..aggregate
            },
            // SignInFailedV1
            EventData::SignInFailedV1 => domain::Account {
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}


