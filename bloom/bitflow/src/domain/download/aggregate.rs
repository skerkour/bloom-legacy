use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::{
        bitflow_downloads,
    },
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "bitflow_downloads"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Download {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,


    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub state: DownloadState,
    pub url: String,

    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, Serialize)]
pub enum DownloadState {
    Admin,
    User,
}

impl Download {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Download{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            error: None,
            name: String::new(),
            progress: 0,
            removed_at: None,
            state: DownloadState::Admin,
            url: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Download {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
