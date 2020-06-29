use crate::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
use serde::{Deserialize, Serialize};
use rustic_reflect_macros::reflect_struct;
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
