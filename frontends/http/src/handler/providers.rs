use std::sync::Arc;

use failure::err_msg;

use rustic_core::{Provider, Rustic};
use viewmodels::*;
use rustic_core::provider::Authentication;

pub fn get_providers(rustic: &Arc<Rustic>) -> Vec<ProviderModel> {
    rustic
        .providers
        .iter()
        .filter(|provider| provider.read()
            .map(|provider| provider.auth_state().is_authenticated())
            .unwrap_or(false))
        .map(|provider| {
            let provider = provider.read().unwrap();
            let title = provider.title().to_owned();
            let provider_type = provider.provider();
            let explore = provider.root();

            ProviderModel {
                title,
                provider: provider_type,
                explore,
            }
        })
        .collect()
}

pub fn navigate(
    rustic: &Arc<Rustic>,
    provider_type: Provider,
    path: &str,
) -> Result<ProviderFolderModel, failure::Error> {
    let provider = rustic
        .providers
        .iter()
        .find(|provider| provider.read().unwrap().provider() == provider_type)
        .ok_or_else(|| err_msg("Invalid provider"))?;

    let provider = provider.read().unwrap();
    let path = path.split("/").map(String::from).collect();
    let folder = provider.navigate(path)?;
    let folder = ProviderFolderModel::new(folder);

    Ok(folder)
}

pub fn get_available_providers(rustic: &Arc<Rustic>) -> Vec<AvailableProviderModel> {
    rustic.providers.iter()
        .map(|provider| {
            let provider = provider.read().expect("can't read provider");
            let provider_type = provider.provider();
            let auth_state = provider.auth_state();

            AvailableProviderModel {
                provider: provider_type,
                title: provider.title().to_owned(),
                enabled: true,
                auth_state: auth_state.into()
            }
        })
        .collect()
}

pub fn authenticate(rustic: &Arc<Rustic>, provider: Provider, token: &str) -> Result<(), failure::Error> {
    let provider = rustic.providers
        .iter()
        .find(|p| p.read().unwrap().provider() == provider);

    if let Some(provider) = provider {
        let mut provider = provider.write().unwrap();
        let auth = Authentication::Token(token.to_owned());
        provider.authenticate(auth)?;
    }

    Ok(())
}