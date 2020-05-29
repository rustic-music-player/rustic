use crate::models::{AlbumModel, TrackModel};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(target_arch = "wasm32", derive(typescript_definitions::TypescriptDefinition))]
pub struct ArtistModel {
    pub cursor: String,
    pub name: String,
    pub albums: Option<Vec<AlbumModel>>,
    pub tracks: Option<Vec<TrackModel>>,
    pub image: Option<String>,
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
