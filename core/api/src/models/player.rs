use crate::models::TrackModel;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct PlayerModel {
    pub cursor: String,
    pub name: String,
    pub playing: bool,
    pub volume: f32,
    pub current: Option<TrackModel>,
}
