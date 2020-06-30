use crate::models::{
    AggregatedAlbum, AggregatedArtist, AggregatedTrack, AlbumModel, ArtistModel, PlaylistModel,
    TrackModel,
};
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct SearchResults {
    pub tracks: Vec<TrackModel>,
    pub albums: Vec<AlbumModel>,
    pub artists: Vec<ArtistModel>,
    pub playlists: Vec<PlaylistModel>,
}

#[reflect_struct]
#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct AggregatedSearchResults {
    pub tracks: Vec<AggregatedTrack>,
    pub albums: Vec<AggregatedAlbum>,
    pub artists: Vec<AggregatedArtist>,
    pub playlists: Vec<PlaylistModel>,
}
