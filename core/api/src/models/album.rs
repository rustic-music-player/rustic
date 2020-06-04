use crate::models::{ArtistModel, ProviderTypeModel, TrackModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct AlbumModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Vec<TrackModel>,
    pub provider: ProviderTypeModel,
    pub coverart: Option<String>,
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
