use crate::models::ProviderTypeModel;
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum SyncStateModel {
    Synchronizing(Vec<SyncItemModel>),
    Idle,
}

#[reflect_struct]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct SyncItemModel {
    pub provider: ProviderTypeModel,
    pub state: SyncItemStateModel,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub enum SyncItemStateModel {
    Idle,
    Syncing,
    Done,
    Error,
}
