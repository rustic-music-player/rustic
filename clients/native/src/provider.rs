use failure::err_msg;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::models::*;
use rustic_core::provider::{Authentication, SharedProvider};

use crate::RusticNativeClient;

#[async_trait]
impl ProviderApiClient for RusticNativeClient {
    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        let providers = self.app
            .providers
            .iter()
            .filter(|provider| {
                provider
                    .read()
                    .map(|provider| provider.auth_state().is_authenticated())
                    .unwrap_or(false)
            })
            .map(|provider| {
                let provider = provider.read().unwrap();
                let title = provider.title().to_owned();
                let provider_type = provider.provider();
                let explore = provider.root();

                ProviderModel {
                    title,
                    provider: provider_type.into(),
                    explore: explore.into(),
                }
            })
            .collect();

        Ok(providers)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        let providers = self.app
            .providers
            .iter()
            .map(|provider| {
                let provider = provider.read().expect("can't read provider");
                let provider_type = provider.provider();
                let auth_state = provider.auth_state();

                AvailableProviderModel {
                    provider: provider_type.into(),
                    title: provider.title().to_owned(),
                    enabled: true,
                    auth_state: auth_state.into(),
                }
            })
            .collect();

        Ok(providers)
    }

    async fn navigate_provider(&self, provider_type: ProviderType, path: &str) -> Result<ProviderFolderModel> {
        let provider = self
            .get_provider(provider_type)
            .ok_or_else(|| err_msg("Invalid provider"))?;

        let provider = provider.read().unwrap();
        let path = path.split('/').map(String::from).collect();
        let folder = provider.navigate(path)?;
        let folder = ProviderFolderModel::from(folder);

        Ok(folder)
    }

    async fn authenticate_provider(&self, provider_type: ProviderType, auth: ProviderAuthModel) -> Result<()> {
        let provider = self.get_provider(provider_type);

        if let Some(provider) = provider {
            let mut provider = provider.write().unwrap();
            let auth = Authentication::from(auth);
            provider.authenticate(auth)?;
        }

        Ok(())
    }
}

impl RusticNativeClient {
    fn get_provider(&self, provider_type: ProviderType) -> Option<&SharedProvider> {
        self.app
            .providers
            .iter()
            .find(|p| p.read().unwrap().provider() == provider_type.into())
    }
}
