use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::gallery_albums,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "gallery_albums"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Album {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub name: String,

    pub owner_id: uuid::Uuid,
}


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "gallery_albums_items"]
#[changeset_options(treat_none_as_null = "true")]
pub struct AlbumItem {
    pub id: uuid::Uuid,

    pub album_id: uuid::Uuid,
    pub file_id: uuid::Uuid,

    pub owner_id: uuid::Uuid,
}
