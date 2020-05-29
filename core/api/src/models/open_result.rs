use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(target_arch = "wasm32", derive(typescript_definitions::TypescriptDefinition))]
#[serde(rename_all = "lowercase", tag = "type", content = "cursor")]
pub enum OpenResultModel {
    Track(String),
    Artist(String),
    Album(String),
    Playlist(String),
}
