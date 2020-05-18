use crate::models::provider::ProviderTypeModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableProviderModel {
    pub title: String,
    pub provider: ProviderTypeModel,
    pub enabled: bool,
    pub auth_state: ProviderAuthenticationState,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", tag = "state")]
pub enum ProviderAuthenticationState {
    NoAuthentication,
    OAuthAuthentication { url: String },
    PasswordAuthentication,
    Authenticated,
}
