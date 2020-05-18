use crate::library::Track;
use crate::provider::ProviderType;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Option<usize>,
    pub title: String,
    pub tracks: Vec<Track>,
    pub provider: ProviderType,
    pub uri: String,
}

impl PartialEq for Playlist {
    fn eq(&self, other: &Playlist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Playlist {}

impl PartialOrd for Playlist {
    fn partial_cmp(&self, other: &Playlist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Playlist {
    fn cmp(&self, other: &Playlist) -> Ordering {
        self.title.cmp(&other.title)
    }
}
