use crate::domain::album;
use serde::{Deserialize, Serialize};

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
        AlbumResponse {
            id: album.id,
            name: album.name,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlbumWithMediaResponse {
    pub album: AlbumResponse,
    pub media: Vec<FileResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAlbumBody {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddToAlbumBody {
    pub media: Vec<uuid::Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveFromAlbumBody {
    pub media: Vec<uuid::Uuid>,
}
