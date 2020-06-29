use crate::models::provider::ProviderTypeModel;
use rustic_reflect_macros::reflect_struct;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[reflect_struct]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all = "camelCase")]
pub struct AvailableProviderModel {
    pub title: String,
    pub provider: ProviderTypeModel,
    pub enabled: bool,
    pub auth_state: ProviderAuthenticationState,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(typescript_definitions::TypescriptDefinition)
)]
#[serde(rename_all = "kebab-case", tag = "state")]
pub enum ProviderAuthenticationState {
    NoAuthentication,
    OAuthAuthentication { url: String },
    PasswordAuthentication,
    Authenticated,
}
