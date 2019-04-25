use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::bitflow_profiles,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "bitflow_profiles"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Profile {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub account_id: uuid::Uuid,
    pub download_folder_id: uuid::Uuid,
}


impl Profile {
    // create a new, unitialized Profile
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Profile{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            total_space: 0,
            used_space: 0,

            account_id: uuid::Uuid::new_v4(),
            download_folder_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Profile {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
