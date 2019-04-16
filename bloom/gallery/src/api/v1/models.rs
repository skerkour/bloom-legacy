use serde::{Serialize, Deserialize};
use crate::domain::album;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileResponse {
    pub id: uuid::Uuid, // file.id
    pub name: String,
    pub parent_id: uuid::Uuid,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAlbumBody {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlbumResponse {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<album::Album> for AlbumResponse {
    fn from(album: album::Album) -> Self {
        AlbumResponse{
            id: album.id,
            name: album.name,
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
