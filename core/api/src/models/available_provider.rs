use crate::models::provider::ProviderType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailableProviderModel {
    pub title: String,
    pub provider: ProviderType,
    pub enabled: bool,
    pub auth_state: ProviderAuthenticationState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "state")]
pub enum ProviderAuthenticationState {
    NoAuthentication,
    OAuthAuthentication { url: String },
    PasswordAuthentication,
    Authenticated,
}
