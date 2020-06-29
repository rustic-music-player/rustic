use std::time::Duration;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::models::TrackModel;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum PlayerEventModel {
    /// Emitted when the player state changes
    StateChanged(bool),
    /// Emitted when the player seeks to a different position
    Seek(Duration),
    /// The currently playing track has changed
    TrackChanged(TrackModel),
    /// The player is waiting for I/O
    Buffering,
    /// The current volume has changed
    VolumeChanged(f32),
}
