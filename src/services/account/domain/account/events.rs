use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_accounts,
    db::schema::account_accounts_events,
    services::common::events::EventMetadata,
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NonPersistedData {
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


impl eventsourcing::Event for Event {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: &Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Account {
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
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}


