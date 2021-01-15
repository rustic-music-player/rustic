use std::time::Duration;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum LibraryEventModel {
    /// Emitted when a new track was added
    TrackAdded(TrackModel),
    /// Emitted when the track with the given cursor was removed
    TrackRemoved(String),
    /// Emitted when a new album was added
    AlbumAdded(AlbumModel),
    /// Emitted when the album with the given cursor was removed
    AlbumRemoved(String),
    /// Emitted when a new artist was added
    ArtistAdded(ArtistModel),
    /// Emitted when the artist with the given cursor was removed
    ArtistRemoved(String),
    /// Emitted when a new playlist was added
    PlaylistAdded(PlaylistModel),
    /// Emitted when the playlist with the given cursor was removed
    PlaylistRemoved(String),
}
