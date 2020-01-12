use std::cmp::Ordering;
use std::sync::Arc;

use cursor::to_cursor;
use rustic_core::library::Album;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use viewmodels::{ArtistModel, TrackModel};

#[derive(Clone, Debug, Serialize, Eq)]
pub struct AlbumModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Vec<TrackModel>,
    pub provider: Provider,
    pub coverart: Option<String>,
}

impl AlbumModel {
    pub fn new(album: Album, app: &Arc<Rustic>) -> AlbumModel {
        let coverart = album.coverart(app);
        AlbumModel {
            cursor: to_cursor(&album.uri),
            title: album.title,
            artist: album.artist.map(|artist| ArtistModel::new(artist, app)),
            tracks: album
                .tracks
                .into_iter()
                .map(|track| TrackModel::new(track, app))
                .collect(),
            provider: album.provider,
            coverart,
        }
    }
}

impl PartialEq for AlbumModel {
    fn eq(&self, other: &Self) -> bool {
        self.cursor == other.cursor
    }
}

impl PartialOrd for AlbumModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AlbumModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}
