use crate::domain::playlist;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileResponse {
    pub id: uuid::Uuid, // file.id
    pub name: String,
    pub parent_id: uuid::Uuid,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePlaylistBody {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaylistResponse {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<playlist::Playlist> for PlaylistResponse {
    fn from(playlist: playlist::Playlist) -> Self {
        PlaylistResponse {
            id: playlist.id,
            name: playlist.name,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaylistWithMediaResponse {
    pub playlist: PlaylistResponse,
    pub musics: Vec<FileResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePlaylistBody {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddToPlaylistBody {
    pub musics: Vec<uuid::Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveFromPlaylistBody {
    pub musics: Vec<uuid::Uuid>,
}
