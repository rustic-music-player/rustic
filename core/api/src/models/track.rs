use crate::models::{AlbumModel, ArtistModel, ProviderTypeModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(typescript_definitions::TypescriptDefinition))]
pub struct TrackModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub album: Option<AlbumModel>,
    pub provider: ProviderTypeModel,
    pub coverart: Option<String>,
    pub duration: Option<u64>,
}

impl PartialOrd for TrackModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}
