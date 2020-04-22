use crate::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResults {
    pub tracks: Vec<TrackModel>,
    pub albums: Vec<AlbumModel>,
    pub artists: Vec<ArtistModel>,
    pub playlists: Vec<PlaylistModel>,
}
