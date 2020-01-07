use diesel::Queryable;
use eventsourcing::Aggregate;
use kernel::db::schema::{music_playlists, music_playlists_files};
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
#[table_name = "music_playlists"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Playlist {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub name: String,

    pub owner_id: uuid::Uuid,
}

impl Playlist {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Playlist {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            name: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "music_playlists_files"]
pub struct PlaylistFile {
    pub id: uuid::Uuid,

    pub playlist_id: uuid::Uuid,
    pub file_id: uuid::Uuid,
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}
