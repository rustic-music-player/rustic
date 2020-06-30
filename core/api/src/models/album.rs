use crate::models::aggregations::Aggregate;
use crate::models::{
    ArtistCollection, ArtistModel, ProviderTypeModel, TrackCollection, TrackModel,
};
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all = "camelCase")]
pub struct AlbumModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Vec<TrackModel>,
    pub provider: ProviderTypeModel,
    pub coverart: Option<String>,
    pub in_library: bool,
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

// This could be generic but would have implications on generated ffi and wasm apis
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AggregatedAlbum {
    Single(AlbumModel),
    Multi(AlbumCollection),
}

impl From<AlbumCollection> for AggregatedAlbum {
    fn from(mut collection: AlbumCollection) -> Self {
        match collection.entries.len() {
            1 => AggregatedAlbum::Single(collection.entries.remove(0)),
            _ => AggregatedAlbum::Multi(collection),
        }
    }
}

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct AlbumCollection {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistCollection>,
    pub tracks: Vec<TrackCollection>,
    pub entries: Vec<AlbumModel>,
    pub coverart: Option<String>,
}

impl Aggregate<AlbumModel> for AlbumCollection {
    fn add_entry(&mut self, album: AlbumModel) {
        self.entries.push(album);
        // TODO: calculate cursor
        // TODO: set tracks
        self.artist = Aggregate::aggregate(
            self.entries
                .iter()
                .filter_map(|track| track.artist.clone())
                .collect(),
        )
        .first()
        .cloned();
        self.coverart = self
            .entries
            .iter()
            .filter_map(|entry| entry.coverart.as_ref())
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .map(String::from);
    }

    fn contains(&self, album: &AlbumModel) -> bool {
        self.title == album.title
    }
}

impl From<AlbumModel> for AlbumCollection {
    fn from(album: AlbumModel) -> Self {
        AlbumCollection {
            cursor: String::new(),
            title: album.title.clone(),
            artist: None,
            coverart: album.coverart.clone(),
            tracks: Vec::new(),
            entries: vec![album],
        }
    }
}
