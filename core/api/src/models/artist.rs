use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rustic_reflect_macros::reflect_struct;

use crate::models::aggregations::Aggregate;
use crate::models::{
    AlbumCollection, AlbumModel, PlaylistModel, ProviderTypeModel, TrackCollection, TrackModel,
};

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ArtistModel {
    pub cursor: String,
    pub name: String,
    pub albums: Option<Vec<AlbumModel>>,
    pub tracks: Option<Vec<TrackModel>>,
    pub playlists: Option<Vec<PlaylistModel>>,
    pub image: Option<String>,
    pub provider: ProviderTypeModel,
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

// This could be generic but would have implications on generated ffi and wasm apis
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AggregatedArtist {
    Single(ArtistModel),
    Multi(ArtistCollection),
}

impl From<ArtistCollection> for AggregatedArtist {
    fn from(mut collection: ArtistCollection) -> Self {
        match collection.entries.len() {
            1 => AggregatedArtist::Single(collection.entries.remove(0)),
            _ => AggregatedArtist::Multi(collection),
        }
    }
}

impl From<ArtistModel> for AggregatedArtist {
    fn from(artist: ArtistModel) -> Self {
        AggregatedArtist::Single(artist)
    }
}

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ArtistCollection {
    pub cursor: String,
    pub name: String,
    pub entries: Vec<ArtistModel>,
    pub image: Option<String>,
    pub albums: Option<Vec<AlbumCollection>>,
    pub tracks: Option<Vec<TrackCollection>>,
    pub playlists: Option<Vec<PlaylistModel>>,
}

impl Aggregate<ArtistModel> for ArtistCollection {
    fn add_entry(&mut self, artist: ArtistModel) {
        self.entries.push(artist);
        let cursors = self
            .entries
            .iter()
            .map(|entry| entry.cursor.clone())
            .collect::<Vec<_>>();
        self.cursor = format!("a:{}", cursors.join(":"));
        self.albums = if self.entries.iter().any(|entry| entry.albums.is_some()) {
            Some(Aggregate::aggregate(
                self.entries
                    .iter()
                    .flat_map(|artist| artist.albums.clone().unwrap_or_default())
                    .collect(),
            ))
        } else {
            None
        };

        self.tracks = if self.entries.iter().any(|entry| entry.tracks.is_some()) {
            Some(Aggregate::aggregate(
                self.entries
                    .iter()
                    .flat_map(|artist| artist.tracks.clone().unwrap_or_default())
                    .collect(),
            ))
        } else {
            None
        };

        self.playlists = if self.entries.iter().any(|entry| entry.playlists.is_some()) {
            Some(
                self.entries
                    .iter()
                    .flat_map(|artist| artist.playlists.clone().unwrap_or_default())
                    .collect(),
            )
        } else {
            None
        };
        self.image = self
            .entries
            .iter()
            .filter_map(|entry| entry.image.as_ref())
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .map(String::from);
    }

    fn contains(&self, artist: &ArtistModel) -> bool {
        self.name == artist.name
    }
}

impl From<ArtistModel> for ArtistCollection {
    fn from(artist: ArtistModel) -> Self {
        ArtistCollection {
            cursor: String::new(),
            name: artist.name.clone(),
            image: artist.image.clone(),
            entries: vec![artist],
            albums: None,
            tracks: None,
            playlists: None,
        }
    }
}
