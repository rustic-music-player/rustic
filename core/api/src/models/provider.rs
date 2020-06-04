use crate::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ProviderModel {
    pub title: String,
    pub provider: ProviderTypeModel,
    pub explore: ProviderFolderModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ProviderFolderModel {
    pub folders: Vec<String>,
    pub items: Vec<ProviderItemModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ProviderItemModel {
    pub label: String,
    pub data: ProviderItemTypeModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all = "camelCase")]
pub enum ProviderItemTypeModel {
    Track(TrackModel),
    Album(AlbumModel),
    Artist(ArtistModel),
    Playlist(PlaylistModel),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all = "lowercase")]
pub enum ProviderTypeModel {
    Pocketcasts,
    Soundcloud,
    #[serde(rename = "gmusic")]
    GooglePlayMusic,
    Spotify,
    #[serde(rename = "local")]
    LocalMedia,
    Youtube,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(untagged)]
pub enum ProviderAuthModel {
    OAuthToken {
        code: String,
        state: Option<String>,
        scope: Option<String>,
    },
    UserPass {
        username: String,
        password: String,
    },
}
