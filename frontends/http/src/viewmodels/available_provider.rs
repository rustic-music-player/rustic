use rustic_core::provider::AuthState;
use rustic_core::Provider;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailableProviderModel {
    pub title: String,
    pub provider: Provider,
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

impl From<AuthState> for ProviderAuthenticationState {
    fn from(state: AuthState) -> Self {
        match state {
            AuthState::NoAuthentication => ProviderAuthenticationState::NoAuthentication,
            AuthState::RequiresOAuth(url) => {
                ProviderAuthenticationState::OAuthAuthentication { url }
            }
            AuthState::RequiresPassword => ProviderAuthenticationState::PasswordAuthentication,
            AuthState::Authenticated(_) => ProviderAuthenticationState::Authenticated,
        }
    }
}
