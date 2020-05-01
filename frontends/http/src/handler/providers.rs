use std::sync::Arc;

use failure::err_msg;

use rustic_core::provider::Authentication;
use rustic_core::{Provider, Rustic};
use rustic_api::models::*;

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
    let folder = ProviderFolderModel::from(folder);

    Ok(folder)
}

pub fn authenticate(
    rustic: &Arc<Rustic>,
    provider: Provider,
    token: &str,
) -> Result<(), failure::Error> {
    let provider = rustic
        .providers
        .iter()
        .find(|p| p.read().unwrap().provider() == provider);

    if let Some(provider) = provider {
        let mut provider = provider.write().unwrap();
        let auth = Authentication::Token(token.to_owned());
        provider.authenticate(auth)?;
    }

    Ok(())
}
