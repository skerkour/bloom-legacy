use diesel::Queryable;
use eventsourcing::Aggregate;
use kernel::db::schema::{gallery_albums, gallery_albums_files};
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
#[table_name = "gallery_albums"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Album {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub name: String,

    pub owner_id: uuid::Uuid,
}

impl Album {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Album {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            name: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Album {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "gallery_albums_files"]
pub struct AlbumFile {
    pub id: uuid::Uuid,

    pub album_id: uuid::Uuid,
    pub file_id: uuid::Uuid,
}
