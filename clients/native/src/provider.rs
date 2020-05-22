use failure::err_msg;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::models::*;
use rustic_core::provider::{Authentication, Provider};

use crate::RusticNativeClient;

#[async_trait]
impl ProviderApiClient for RusticNativeClient {
    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        let mut provider_models = Vec::new();
        for provider in self.app.providers.iter() {
            let provider = provider.get().await;
            if !provider.auth_state().is_authenticated() {
                continue;
            }
            let title = provider.title().to_owned();
            let provider_type = provider.provider();
            let explore = provider.root();

            let model = ProviderModel {
                title,
                provider: provider_type.into(),
                explore: explore.into(),
            };
            provider_models.push(model);
        }

        Ok(provider_models)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        let mut provider_models = Vec::new();
        for provider in self.app.providers.iter() {
            let provider = provider.get().await;
            let provider_type = provider.provider();
            let auth_state = provider.auth_state();

            let model = AvailableProviderModel {
                provider: provider_type.into(),
                title: provider.title().to_owned(),
                enabled: true,
                auth_state: auth_state.into(),
            };
            provider_models.push(model);
        }

        Ok(provider_models)
    }

    async fn navigate_provider(
        &self,
        provider_type: ProviderTypeModel,
        path: &str,
    ) -> Result<ProviderFolderModel> {
        let provider = self
            .get_provider(provider_type)
            .ok_or_else(|| err_msg("Invalid provider"))?;

        let provider = provider.get().await;
        let path = path.split('/').map(String::from).collect();
        let folder = provider.navigate(path).await?;
        let folder = ProviderFolderModel::from(folder);

        Ok(folder)
    }

    async fn authenticate_provider(
        &self,
        provider_type: ProviderTypeModel,
        auth: ProviderAuthModel,
    ) -> Result<()> {
        let provider = self.get_provider(provider_type);

        if let Some(provider) = provider {
            // TODO: we should await instead of blocking
            let mut provider = provider.get_mut().await;
            let auth = Authentication::from(auth);
            provider.authenticate(auth).await?;
        }

        Ok(())
    }
}

impl RusticNativeClient {
    fn get_provider(&self, provider_type: ProviderTypeModel) -> Option<&Provider> {
        self.app
            .providers
            .iter()
            .find(|p| p.provider_type == provider_type.into())
    }
}
