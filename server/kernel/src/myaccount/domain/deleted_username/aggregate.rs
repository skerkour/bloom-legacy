use crate::db::schema::kernel_deleted_usernames;
use diesel::Queryable;
use eventsourcing::Aggregate;
use serde::{Deserialize, Serialize};

#[derive(Aggregate, AsChangeset, Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "kernel_deleted_usernames"]
#[changeset_options(treat_none_as_null = "true")]
pub struct DeletedUsername {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
    pub username: String,
}

impl DeletedUsername {
    // create a new, unitialized DeletedUsername
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return DeletedUsername {
            created_at: now,
            username: String::new(),
            version: 0,
            updated_at: now,
        };
    }
}

impl Default for DeletedUsername {
    fn default() -> Self {
        Self::new()
    }
}
