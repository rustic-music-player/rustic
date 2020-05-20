use crate::models::{ProviderTypeModel, TrackModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
pub struct PlaylistModel {
    pub cursor: String,
    pub title: String,
    pub tracks: Vec<TrackModel>,
    pub provider: ProviderTypeModel,
}

impl PartialOrd for PlaylistModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlaylistModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}
