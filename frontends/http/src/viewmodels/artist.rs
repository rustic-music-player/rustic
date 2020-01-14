use std::cmp::Ordering;

use cursor::to_cursor;
use rustic_core::library::Artist;
use viewmodels::{AlbumModel, TrackModel};

#[derive(Clone, Debug, Serialize, Eq)]
pub struct ArtistModel {
    pub cursor: String,
    pub name: String,
    pub albums: Option<Vec<AlbumModel>>,
    pub tracks: Option<Vec<TrackModel>>,
    pub image: Option<String>,
}

impl ArtistModel {
    pub fn new(artist: Artist) -> ArtistModel {
        ArtistModel {
            cursor: to_cursor(&artist.uri),
            name: artist.name,
            albums: None,
            tracks: None,
            image: artist.image_url.clone(),
        }
    }
}

impl PartialEq for ArtistModel {
    fn eq(&self, other: &Self) -> bool {
        self.cursor == other.cursor
    }
}

impl PartialOrd for ArtistModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArtistModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
