use crate::models::{ArtistModel, ProviderTypeModel, TrackModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use rustic_reflect_macros::reflect_struct;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all="camelCase")]
pub struct AlbumModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Vec<TrackModel>,
    pub provider: ProviderTypeModel,
    pub coverart: Option<String>,
    pub in_library: bool
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
