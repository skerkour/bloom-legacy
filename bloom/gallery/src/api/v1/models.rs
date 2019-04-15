use serde::{Serialize, Deserialize};
use crate::domain::album;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileResponse {
    pub id: uuid::Uuid, // file.id
    pub name: String,
    pub parent_id: uuid::Uuid,
    pub url: String,
}

impl From<album::File> for FileResponse {
    fn from(file: album::File) -> Self {
        FileResponse{
            id: file.id,
            name: file.name,
            parent_id: file.parent_id,
            url: file.url,
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlbumItemResponse {
    pub id: uuid::Uuid, // file.id
    pub name: String,
    pub parent_id: uuid::Uuid,
    pub album_id: uuid::Uuid,
    pub url: String,
}
