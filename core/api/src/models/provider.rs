use crate::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
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

impl ProviderModel {
    pub fn internal() -> Self {
        ProviderModel {
            title: "Internal".into(),
            provider: ProviderTypeModel::Internal,
            explore: ProviderFolderModel::default(),
        }
    }
}

#[reflect_struct]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ProviderFolderModel {
    pub folders: Vec<String>,
    pub items: Vec<ProviderItemModel>,
}

#[reflect_struct]
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
    Internal,
    Pocketcasts,
    Soundcloud,
    #[serde(rename = "gmusic")]
    #[deprecated]
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
