use crate::models::aggregations::Aggregate;
use crate::models::{
    AlbumCollection, AlbumModel, ArtistCollection, ArtistModel, ProviderTypeModel, MetaValueModel,
};
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct TrackModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub album: Option<AlbumModel>,
    pub provider: ProviderTypeModel,
    pub coverart: Option<String>,
    pub duration: Option<u64>,
    pub meta: HashMap<String, MetaValueModel>,
}

#[reflect_struct]
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
pub struct QueuedTrackModel {
    #[serde(flatten)]
    pub track: TrackModel,
    pub playing: bool,
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

// This could be generic but would have implications on generated ffi and wasm apis
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AggregatedTrack {
    Single(TrackModel),
    Multi(TrackCollection),
}

impl From<TrackCollection> for AggregatedTrack {
    fn from(mut collection: TrackCollection) -> Self {
        match collection.entries.len() {
            1 => AggregatedTrack::Single(collection.entries.remove(0)),
            _ => AggregatedTrack::Multi(collection),
        }
    }
}

impl From<TrackModel> for AggregatedTrack {
    fn from(track: TrackModel) -> Self {
        AggregatedTrack::Single(track)
    }
}

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct TrackCollection {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistCollection>,
    pub album: Option<AlbumCollection>,
    pub entries: Vec<TrackModel>,
    pub coverart: Option<String>,
    pub duration: Option<u64>,
}

impl Aggregate<TrackModel> for TrackCollection {
    fn add_entry(&mut self, track: TrackModel) {
        self.entries.push(track);
        let cursors = self
            .entries
            .iter()
            .map(|entry| entry.cursor.clone())
            .collect::<Vec<_>>();
        self.cursor = format!("a:{}", cursors.join(":"));
        self.album = Aggregate::aggregate(
            self.entries
                .iter()
                .filter_map(|track| track.album.clone())
                .collect(),
        )
        .first()
        .cloned();
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
        self.duration = self
            .entries
            .iter()
            .filter_map(|entry| entry.duration)
            .collect::<Vec<_>>()
            .first()
            .copied();
    }

    fn contains(&self, track: &TrackModel) -> bool {
        self.title == track.title
    }
}

impl From<TrackModel> for TrackCollection {
    fn from(track: TrackModel) -> Self {
        TrackCollection {
            cursor: String::new(),
            title: track.title.clone(),
            artist: None,
            album: None,
            coverart: track.coverart.clone(),
            duration: track.duration,
            entries: vec![track],
        }
    }
}
