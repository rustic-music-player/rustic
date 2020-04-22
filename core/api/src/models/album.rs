use crate::models::{ArtistModel, ProviderType, TrackModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct AlbumModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Vec<TrackModel>,
    pub provider: ProviderType,
    pub coverart: Option<String>,
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
