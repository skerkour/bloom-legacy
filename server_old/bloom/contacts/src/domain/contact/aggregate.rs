use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use eventsourcing::Aggregate;
use kernel::db::schema::contacts_contacts;
use serde::{Deserialize, Serialize};

#[derive(
    Aggregate,
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
)]
#[table_name = "contacts_contacts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Contact {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub addresses: Vec<Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<Organization>,
    pub phones: Vec<Phone>,
    pub websites: Vec<Website>,

    pub owner_id: uuid::Uuid,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub label: AddressLabel,
    pub postal_code: Option<String>,
    pub street_address: Option<String>,
    pub street_address2: Option<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AddressLabel {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "WORK")]
    Work,
    #[serde(rename = "OTHER")]
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Email {
    pub email: String,
    pub label: EmailLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum EmailLabel {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "OTHEWORKR")]
    Work,
    #[serde(rename = "OTHER")]
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Organization {
    pub name: Option<String>,
    pub title: Option<String>,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Phone {
    pub phone: String,
    pub label: PhoneLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PhoneLabel {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "WORK")]
    Work,
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "MAIN")]
    Main,
    #[serde(rename = "HOME_FAX")]
    HomeFax,
    #[serde(rename = "WORK_FAX")]
    WorkFax,
    #[serde(rename = "OTHER")]
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Website {
    pub website: String,
    pub label: WebsiteLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WebsiteLabel {
    #[serde(rename = "PROFILE")]
    Profile,
    #[serde(rename = "BLOG")]
    Blog,
    #[serde(rename = "HOME_PAGE")]
    HomePage,
    #[serde(rename = "WORK")]
    Work,
    #[serde(rename = "OTHER")]
    Other,
}

impl Contact {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Contact {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            addresses: Vec::new(),
            birthday: None,
            company: None,
            emails: Vec::new(),
            first_name: None,
            last_name: None,
            notes: None,
            occupation: None,
            organizations: Vec::new(),
            phones: Vec::new(),
            websites: Vec::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self::new()
    }
}
