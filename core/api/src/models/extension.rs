use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ExtensionModel {
    pub name: String,
    pub id: String,
    pub version: String,
    pub enabled: bool,
    pub controls: ExtensionControlsModel,
}

#[reflect_struct]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ExtensionControlsModel {
    pub actions: Vec<ExtensionActionModel>,
    pub infos: Vec<ExtensionInfoModel>,
}

#[reflect_struct]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
pub struct ExtensionActionModel {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(tag = "type", content = "value")]
pub enum ExtensionInfoModel {
    Link(String),
}
