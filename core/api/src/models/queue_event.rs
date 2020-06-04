use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::models::TrackModel;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum QueueEventModel {
    /// The queue has been changed
    QueueUpdated(Vec<TrackModel>),
}
