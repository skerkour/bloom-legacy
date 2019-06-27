use crate::domain::contact;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateContactBody {
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContactResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateContactBody {
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
}
